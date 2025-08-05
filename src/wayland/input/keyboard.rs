// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2025, Nathan Gill

/*
    keyboard.rs:
        Redirects keyboard events to Qt
*/

use crate::wayland::{
    event::{event::Event, event_param::EventParam, event_type::EventType},
    state::WaylandState,
};
use std::os::fd::IntoRawFd;
use wayland_client::{
    Connection, Dispatch, QueueHandle, WEnum,
    protocol::wl_keyboard::{self, KeymapFormat, WlKeyboard},
};

impl Dispatch<WlKeyboard, ()> for WaylandState {
    fn event(
        wayland_state: &mut Self,
        _proxy: &WlKeyboard,
        event: <WlKeyboard as wayland_client::Proxy>::Event,
        _data: &(),
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
    ) {
        // Redirect events into the Qt event loop for further processing
        match event {
            wl_keyboard::Event::Keymap { format, fd, size } => {
                if format == WEnum::Value(KeymapFormat::XkbV1) {
                    let event = Event::new(
                        EventType::KeyboardKeymap,
                        EventParam::from(fd.into_raw_fd() as u64),
                        EventParam::from(size as u64),
                    );
                    let _ = event.write_to(
                        wayland_state
                            .renderer_write_pipe
                            .as_ref()
                            .unwrap()
                            .write_fd(),
                    );
                }
            }
            wl_keyboard::Event::Key {
                serial: _,
                time: _,
                key,
                state,
            } => {
                let event = Event::new(
                    EventType::KeyboardKey,
                    EventParam::from(key as u64),
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
            wl_keyboard::Event::Modifiers {
                serial: _,
                mods_depressed,
                mods_latched,
                mods_locked,
                group,
            } => {
                let param_1 =
                    EventParam::from(((mods_depressed as u64) << 32) | (mods_latched as u64));
                let param_2 = EventParam::from(((mods_locked as u64) << 32) | (group as u64));
                let event = Event::new(EventType::KeyboardModifiers, param_1, param_2);
                let _ = event.write_to(
                    wayland_state
                        .renderer_write_pipe
                        .as_ref()
                        .unwrap()
                        .write_fd(),
                );
            }
            wl_keyboard::Event::RepeatInfo { rate, delay } => {
                let event = Event::new(
                    EventType::KeyboardRepeatInfo,
                    EventParam::from(rate as u64),
                    EventParam::from(delay as u64),
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
