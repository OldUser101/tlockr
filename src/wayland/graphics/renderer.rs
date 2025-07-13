use crate::{
    shared::{
        ffi::{cleanup_renderer, initialize_renderer, set_callbacks},
        interface::{get_qml_path, get_renderer, set_renderer},
    },
    wayland::state::WaylandState,
};
use std::ffi::c_void;

impl WaylandState {
    unsafe extern "C" fn get_buffer_callback(user_data: *mut c_void) -> *mut c_void {
        let state = unsafe { &mut *(user_data as *mut WaylandState) };

        state.buffers.as_ref().expect("buffers is None")[0].data as *mut c_void
    }

    unsafe extern "C" fn frame_ready_callback(user_data: *mut c_void, buffer: *mut c_void) {
        let state = unsafe { &mut *(user_data as *mut WaylandState) };

        if let (Some(surface), Some(viewport)) = (&state.surface, &state.viewport) {
            if let Some(buffers) = &state.buffers {
                if let Some(found_buffer) = buffers.iter().find(|b| b.data as *mut c_void == buffer)
                {
                    surface.attach(Some(&found_buffer.buffer), 0, 0);
                    surface.damage_buffer(0, 0, i32::MAX, i32::MAX);
                    viewport.set_destination(state.width, state.height);
                    surface.commit();
                } else {
                    println!("No matching buffer found.");
                }
            }
        }
    }

    pub fn initialize_renderer(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let renderer = unsafe {
            initialize_renderer(
                self.width,
                self.height,
                get_qml_path(self.app_state).unwrap(),
            )
        };

        if renderer != std::ptr::null_mut() {
            set_renderer(self.app_state, renderer);

            unsafe {
                set_callbacks(
                    renderer,
                    Self::get_buffer_callback,
                    Self::frame_ready_callback,
                    self as *mut _ as *mut c_void,
                );
            }
        } else {
            return Err("QML renderer initialization failed".into());
        }

        Ok(())
    }

    pub fn destroy_renderer(&mut self) {
        if let Some(renderer) = get_renderer(self.app_state) {
            unsafe {
                cleanup_renderer(renderer);
                set_renderer(self.app_state, std::ptr::null_mut());
            }
        }
    }
}
