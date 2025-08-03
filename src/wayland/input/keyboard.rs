// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2025, Nathan Gill

/*
    keyboard.rs:
        Obtains a keymap which can be used to get key values from scancodes,
        and contains basic keypress handling.
*/

use crate::wayland::{
    event::{event::Event, event_param::EventParam, event_type::EventType},
    state::WaylandState,
};
use memmap2::MmapOptions;
use std::{
    fs::File,
    os::fd::{FromRawFd, IntoRawFd},
};
use wayland_client::{
    Connection, Dispatch, QueueHandle, WEnum,
    protocol::{
        wl_keyboard::{self, KeymapFormat, WlKeyboard},
        wl_seat::{self, Capability, WlSeat},
    },
};
use xkbcommon_rs::{Keymap, State};

pub struct KeyboardMapping {
    pub file: std::fs::File,
    pub mmap: memmap2::Mmap,
    pub keymap: Option<Keymap>,
    pub state: Option<State>,
}

impl Dispatch<WlKeyboard, ()> for WaylandState {
    fn event(
        wayland_state: &mut Self,
        _proxy: &WlKeyboard,
        event: <WlKeyboard as wayland_client::Proxy>::Event,
        _data: &(),
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
    ) {
        match event {
            wl_keyboard::Event::Keymap { format, fd, size } => {
                if format == WEnum::Value(KeymapFormat::XkbV1) {
                    let file = unsafe { File::from_raw_fd(fd.into_raw_fd()) };
                    let mmap_result = unsafe { MmapOptions::new().len(size as usize).map(&file) };
                    match mmap_result {
                        Ok(mmap) => {
                            let keymap_str =
                                std::str::from_utf8(&mmap).expect("Keymap mmap is not valid UTF-8");
                            let keymap_result = xkbcommon_rs::Keymap::new_from_string(
                                xkbcommon_rs::Context::new(0).unwrap(),
                                keymap_str,
                                xkbcommon_rs::KeymapFormat::TextV1,
                                0,
                            );

                            match keymap_result {
                                Ok(keymap) => {
                                    let state = State::new(keymap.clone());
                                    let mapping = KeyboardMapping {
                                        file: file,
                                        mmap: mmap,
                                        keymap: Some(keymap),
                                        state: Some(state),
                                    };
                                    wayland_state.keymap = Some(mapping);
                                    println!("Found xkbV1 format keymap and created state.");
                                }
                                Err(_) => {
                                    println!("Error compiling keymap.");
                                }
                            }
                        }
                        Err(e) => {
                            eprintln!("Failed to mmap keymap file: {}", e);
                        }
                    }
                }
            }
            wl_keyboard::Event::Key {
                serial: _,
                time: _,
                key,
                state: _,
            } => {
                if let Some(ref keymap) = wayland_state.keymap {
                    if let (Some(_keymap_file), Some(state)) = (&keymap.keymap, &keymap.state) {
                        let keycode = key + 8;
                        let keysym = state.key_get_one_sym(keycode);
                        let s = xkbcommon_rs::keysym::keysym_get_name(&keysym.unwrap()).unwrap();
                        println!("Pressed: {}", s);

                        // Write a dummy event into the renderer's event pipe
                        let event = Event::new(
                            EventType::Keyboard,
                            EventParam::from(0),
                            EventParam::from(0),
                        );
                        let event_ptr = &event as *const Event as *const u8;
                        let event_buf = unsafe {
                            std::slice::from_raw_parts(event_ptr, std::mem::size_of::<Event>())
                        };

                        match nix::unistd::write(
                            wayland_state
                                .renderer_write_pipe
                                .as_ref()
                                .unwrap()
                                .write_fd(),
                            event_buf,
                        ) {
                            Ok(_) => {
                                // Event sent successfully
                            }
                            Err(nix::errno::Errno::EAGAIN) => {
                                println!(
                                    "Warning: Dropping keyboard event, renderer pipe buffer full"
                                );
                            }
                            Err(e) => {
                                println!("Failed to send keyboard event to renderer: {}", e);
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }
}

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
                        println!("Acquired keyboard input interface.");
                    } else if !bits.contains(Capability::Keyboard) && state.keyboard.is_some() {
                        if let Some(ref keyboard) = state.keyboard {
                            keyboard.release();
                            state.keyboard = None;
                        }
                    }
                }
                _ => {}
            },
            _ => {}
        }
    }
}
