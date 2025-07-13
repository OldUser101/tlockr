/*
    WlBuffer related dispatch handlers
*/

use wayland_client::{
    Connection, Dispatch, QueueHandle,
    protocol::wl_buffer::{self, WlBuffer},
};

use crate::wayland::interface::WaylandState;

impl Dispatch<WlBuffer, i32> for WaylandState {
    fn event(
        state: &mut Self,
        _proxy: &WlBuffer,
        event: <WlBuffer as wayland_client::Proxy>::Event,
        data: &i32,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
    ) {
        match event {
            wl_buffer::Event::Release => {
                if let Some(buffers) = state.buffers.as_mut() {
                    buffers[*data as usize].in_use = false;
                }
            }
            _ => {}
        }
    }
}
