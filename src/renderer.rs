use crate::{
    ffi::{QmlRenderer, cleanup_renderer, initialize_renderer, set_callbacks},
    state::LockState,
};

use std::ffi::{CString, c_void};

pub struct QmlRendererInterface {
    pub renderer: *mut QmlRenderer,
    pub qml_path: Option<CString>,
}

impl QmlRendererInterface {
    pub fn new() -> Self {
        Self {
            renderer: std::ptr::null_mut(),
            qml_path: None,
        }
    }
}

impl LockState {
    unsafe extern "C" fn get_buffer_callback(user_data: *mut c_void) -> *mut c_void {
        let lock_state = unsafe { &mut *(user_data as *mut LockState) };

        lock_state
            .interfaces
            .buffers
            .as_ref()
            .expect("buffers is None")[0]
            .data as *mut c_void
    }

    unsafe extern "C" fn frame_ready_callback(user_data: *mut c_void, buffer: *mut c_void) {
        let lock_state = unsafe { &mut *(user_data as *mut LockState) };

        if let (Some(surface), Some(viewport)) = (
            &lock_state.interfaces.surface,
            &lock_state.interfaces.viewport,
        ) {
            if let Some(buffers) = &lock_state.interfaces.buffers {
                if let Some(found_buffer) = buffers.iter().find(|b| b.data as *mut c_void == buffer)
                {
                    surface.attach(Some(&found_buffer.buffer), 0, 0);
                    surface.damage_buffer(0, 0, i32::MAX, i32::MAX);
                    viewport
                        .set_destination(lock_state.interfaces.width, lock_state.interfaces.height);
                    surface.commit();
                } else {
                    println!("No matching buffer found.");
                }
            }
        }
    }

    pub fn initialize_renderer(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let qml_path = CString::new(self.qml_path.as_str()).unwrap();

        let renderer = unsafe {
            initialize_renderer(
                self.interfaces.width,
                self.interfaces.height,
                qml_path.as_ptr(),
            )
        };

        if renderer != std::ptr::null_mut() {
            self.renderer.renderer = renderer;
            self.renderer.qml_path = Some(qml_path);

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
        if self.renderer.renderer != std::ptr::null_mut() {
            unsafe {
                cleanup_renderer(self.renderer.renderer);
                self.renderer.renderer = std::ptr::null_mut();
                self.renderer.qml_path = None;
            }
        }
    }
}
