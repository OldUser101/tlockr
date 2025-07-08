use wayland_client::{Connection, Dispatch, EventQueue, QueueHandle};
use wayland_protocols::ext::session_lock::v1::client::{
    ext_session_lock_surface_v1::{self, ExtSessionLockSurfaceV1},
    ext_session_lock_v1::{self, ExtSessionLockV1},
};

use std::ffi::CString;
use std::os::raw::c_void;

use crate::{ffi::render_single_frame, state::LockState};

#[derive(PartialEq)]
pub enum State {
    None,
    Initialized,
    Ready,
    Locked,
    Unlocked,
}

impl Dispatch<ExtSessionLockV1, ()> for LockState {
    fn event(
        state: &mut Self,
        proxy: &ExtSessionLockV1,
        event: <ExtSessionLockV1 as wayland_client::Proxy>::Event,
        _data: &(),
        _conn: &Connection,
        qh: &QueueHandle<Self>,
    ) {
        match event {
            ext_session_lock_v1::Event::Locked => {
                println!("Session is locked");
                state.state = State::Locked;
                if let Some(surface) = &state.interfaces.surface {
                    if let Some(output) = &state.interfaces.output {
                        let lock_surface = proxy.get_lock_surface(surface, output, &qh, ());
                        state.interfaces.session_lock_surface = Some(lock_surface);
                    }
                }
            }
            ext_session_lock_v1::Event::Finished => {
                println!("Session is unlocked");
                state.state = State::Unlocked;
            }
            _ => {}
        }
    }
}

impl Dispatch<ExtSessionLockSurfaceV1, ()> for LockState {
    fn event(
        state: &mut Self,
        proxy: &ExtSessionLockSurfaceV1,
        event: <ExtSessionLockSurfaceV1 as wayland_client::Proxy>::Event,
        _data: &(),
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
    ) {
        match event {
            ext_session_lock_surface_v1::Event::Configure {
                serial,
                width: _,
                height: _,
            } => {
                proxy.ack_configure(serial);

                if let (Some(surface), Some(viewport)) =
                    (&state.interfaces.surface, &state.interfaces.viewport)
                {
                    let buffer = &mut state.interfaces.buffers.as_mut().unwrap()[0];
                    buffer.in_use = true;

                    unsafe {
                        let qml_path = CString::new(state.qml_path.as_str()).unwrap();
                        render_single_frame(
                            qml_path.as_ptr(),
                            state.interfaces.width,
                            state.interfaces.height,
                            buffer.data as *mut c_void,
                        );
                    }

                    surface.attach(Some(&buffer.buffer), 0, 0);

                    surface.damage_buffer(0, 0, i32::MAX, i32::MAX);

                    viewport.set_destination(state.interfaces.width, state.interfaces.height);

                    surface.commit();

                    println!("Acknowledged and configured surface.");
                }
            }
            _ => {}
        }
    }
}

impl LockState {
    pub fn lock(
        &mut self,
        event_queue: &EventQueue<LockState>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let qh = event_queue.handle();

        if let (Some(compositor), Some(viewporter)) =
            (&self.interfaces.compositor, &self.interfaces.viewporter)
        {
            let surface = compositor.create_surface(&qh, ());
            let viewport = viewporter.get_viewport(&surface, &qh, ());
            self.interfaces.surface = Some(surface);
            self.interfaces.viewport = Some(viewport);
        }

        if let Some(session_lock_manager) = &self.interfaces.session_lock_manager {
            self.interfaces.session_lock = Some(session_lock_manager.lock(&qh, ()));
        } else {
            return Err("Failed to lock session.".to_string().into());
        }

        Ok(())
    }
}
