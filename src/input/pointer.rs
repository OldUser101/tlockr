// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2025, Nathan Gill

/*
    pointer.rs:
        Redirects pointer events to Qt
*/

use crate::event::{Event, EventParam, EventType};
use crate::wayland::WaylandState;

use std::time::{Duration, Instant};
use wayland_client::{
    Connection, Dispatch, QueueHandle, WEnum,
    protocol::wl_pointer::{self, WlPointer},
};

impl Dispatch<WlPointer, ()> for WaylandState {
    fn event(
        wayland_state: &mut Self,
        _proxy: &WlPointer,
        event: <WlPointer as wayland_client::Proxy>::Event,
        _data: &(),
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
    ) {
        match event {
            wl_pointer::Event::Motion {
                time: _,
                surface_x,
                surface_y,
            } => {
                let mut send_event = false;
                let event = Event::new(
                    EventType::PointerMotion,
                    EventParam::from(surface_x),
                    EventParam::from(surface_y),
                );
                let now = Instant::now();

                if wayland_state.pointer_timestamp.is_none() {
                    wayland_state.pointer_timestamp = Some(now);
                    send_event = true;
                }

                let timestamp = wayland_state.pointer_timestamp.unwrap();
                let delta = now.duration_since(timestamp);

                // Cap motion events at 60 events per second to avoid filling the event queue
                if delta >= Duration::from_millis(1000 / 60) {
                    send_event = true;
                }

                if send_event {
                    wayland_state.pointer_timestamp = Some(now);

                    let _ = event.write_to(
                        wayland_state
                            .renderer_write_pipe
                            .as_ref()
                            .unwrap()
                            .write_fd(),
                    );
                } else {
                    wayland_state.pending_pointer_event = Some(event);
                }
            }
            wl_pointer::Event::Button {
                serial: _,
                time: _,
                button,
                state,
            } => {
                let event = Event::new(
                    EventType::PointerButton,
                    EventParam::from(button as u64),
                    EventParam::from(match state {
                        WEnum::Value(val) => val as u64,
                        WEnum::Unknown(val) => val as u64,
                    }),
                );
                let _ = event.write_to(
                    wayland_state
                        .renderer_write_pipe
                        .as_ref()
                        .unwrap()
                        .write_fd(),
                );
            }
            _ => {}
        }
    }
}
