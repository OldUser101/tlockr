/*
    WlOutput related dispatch handlers
*/

use crate::wayland::state::LockState;
use wayland_client::{
    Connection, Dispatch, QueueHandle,
    protocol::wl_output::{self, WlOutput},
};

impl Dispatch<WlOutput, ()> for LockState {
    fn event(
        state: &mut Self,
        _proxy: &WlOutput,
        event: <WlOutput as wayland_client::Proxy>::Event,
        _data: &(),
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
    ) {
        match event {
            wl_output::Event::Mode {
                flags,
                width,
                height,
                refresh: _,
            } => {
                if flags.into_result().unwrap() == wl_output::Mode::Current {
                    println!("Output mode: {} x {} pixels", width, height);
                    state.interfaces.width = width;
                    state.interfaces.height = height;
                }
            }
            wl_output::Event::Done => {
                state.interfaces.output_configured = true;
            }
            _ => {}
        }
    }
}
