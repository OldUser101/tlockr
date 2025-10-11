// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2025, Nathan Gill

/*
    seat.rs:
        Handles seat capability events and acquires keyboard and pointer interfaces
*/

use crate::wayland::WaylandState;

use tracing::debug;
use wayland_client::{
    Connection, Dispatch, QueueHandle, WEnum,
    protocol::wl_seat::{self, Capability, WlSeat},
};

impl Dispatch<WlSeat, ()> for WaylandState {
    fn event(
        state: &mut Self,
        proxy: &WlSeat,
        event: <WlSeat as wayland_client::Proxy>::Event,
        _data: &(),
        _conn: &Connection,
        qh: &QueueHandle<Self>,
    ) {
        match event {
            wl_seat::Event::Capabilities { capabilities } => match capabilities {
                WEnum::Value(bits) => {
                    if bits.contains(Capability::Keyboard) && state.keyboard.is_none() {
                        let keyboard = proxy.get_keyboard(qh, ());
                        state.keyboard = Some(keyboard);
                        debug!("Acquired keyboard input interface.");
                    }

                    if bits.contains(Capability::Pointer) && state.pointer.is_none() {
                        let pointer = proxy.get_pointer(qh, ());
                        state.pointer = Some(pointer);
                        debug!("Acquired pointer input interface.");
                    }

                    if !bits.contains(Capability::Keyboard) && state.keyboard.is_some() {
                        if let Some(ref keyboard) = state.keyboard {
                            keyboard.release();
                            state.keyboard = None;
                        }
                    }

                    if !bits.contains(Capability::Pointer) && state.pointer.is_some() {
                        if let Some(ref pointer) = state.pointer {
                            pointer.release();
                            state.pointer = None;
                        }
                    }
                }
                _ => {}
            },
            _ => {}
        }
    }
}
