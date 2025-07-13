/*
    Session lock related dispatch handlers
*/

use crate::shared::interface::get_renderer;
use crate::shared::state::State;
use crate::wayland::interface::WaylandState;
use crate::{shared::interface::set_state, wayland::ffi::start_renderer};
use wayland_client::{Connection, Dispatch, QueueHandle};
use wayland_protocols::ext::session_lock::v1::client::{
    ext_session_lock_surface_v1::{self, ExtSessionLockSurfaceV1},
    ext_session_lock_v1::{self, ExtSessionLockV1},
};

impl Dispatch<ExtSessionLockV1, ()> for WaylandState {
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
                set_state(state.app_state, State::Locked);
                if let Some(surface) = &state.surface {
                    if let Some(output) = &state.output {
                        let lock_surface = proxy.get_lock_surface(surface, output, &qh, ());
                        state.session_lock_surface = Some(lock_surface);
                    }
                }
            }
            ext_session_lock_v1::Event::Finished => {
                println!("Session is unlocked");
                set_state(state.app_state, State::Unlocked);
            }
            _ => {}
        }
    }
}

impl Dispatch<ExtSessionLockSurfaceV1, ()> for WaylandState {
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

                unsafe {
                    start_renderer(get_renderer(state.app_state).unwrap());
                }
            }
            _ => {}
        }
    }
}
