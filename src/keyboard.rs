use wayland_client::{
    Connection, Dispatch, QueueHandle, WEnum,
    protocol::{
        wl_keyboard::{self, KeymapFormat, WlKeyboard},
        wl_seat::{self, Capability, WlSeat},
    },
};

use crate::state::LockState;

impl Dispatch<WlKeyboard, ()> for LockState {
    fn event(
        _state: &mut Self,
        _proxy: &WlKeyboard,
        event: <WlKeyboard as wayland_client::Proxy>::Event,
        _data: &(),
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
    ) {
        match event {
            wl_keyboard::Event::Keymap {
                format,
                fd: _,
                size: _,
            } => {
                if format == WEnum::Value(KeymapFormat::XkbV1) {
                    println!("Found xkbV1 format keymap.");
                }
            }
            _ => {}
        }
    }
}

impl Dispatch<WlSeat, ()> for LockState {
    fn event(
        state: &mut Self,
        _proxy: &WlSeat,
        event: <WlSeat as wayland_client::Proxy>::Event,
        _data: &(),
        _conn: &Connection,
        qh: &QueueHandle<Self>,
    ) {
        match event {
            wl_seat::Event::Capabilities { capabilities } => {
                if capabilities == WEnum::Value(Capability::Keyboard) {
                    let keyboard = state.interfaces.seat.as_ref().unwrap().get_keyboard(qh, ());
                    state.interfaces.keyboard = Some(keyboard);
                    println!("Acquired keyboard input interface.");
                }
            }
            _ => {}
        }
    }
}
