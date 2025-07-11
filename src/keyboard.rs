use memmap2::MmapOptions;
use wayland_client::{
    Connection, Dispatch, QueueHandle, WEnum,
    protocol::{
        wl_keyboard::{self, KeymapFormat, WlKeyboard},
        wl_seat::{self, Capability, WlSeat},
    },
};

use std::{
    fs::File,
    os::fd::{FromRawFd, IntoRawFd},
};

use crate::state::LockState;

use xkbcommon_rs::{Keymap, State};

pub struct KeyboardMapping {
    _file: std::fs::File,
    _mmap: memmap2::Mmap,
    keymap: Option<Keymap>,
    state: Option<State>,
}

impl Dispatch<WlKeyboard, ()> for LockState {
    fn event(
        lock_state: &mut Self,
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
                                        _file: file,
                                        _mmap: mmap,
                                        keymap: Some(keymap),
                                        state: Some(state),
                                    };
                                    lock_state.interfaces.keymap = Some(mapping);
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
                if let Some(ref keymap) = lock_state.interfaces.keymap {
                    if let (Some(_keymap_file), Some(state)) = (&keymap.keymap, &keymap.state) {
                        let keycode = key + 8;
                        let keysym = state.key_get_one_sym(keycode);
                        let s = xkbcommon_rs::keysym::keysym_get_name(&keysym.unwrap()).unwrap();
                        println!("Pressed: {}", s);
                    }
                }
            }
            _ => {}
        }
    }
}

impl Dispatch<WlSeat, ()> for LockState {
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
                    if bits.contains(Capability::Keyboard) && state.interfaces.keyboard.is_none() {
                        let keyboard = proxy.get_keyboard(qh, ());
                        state.interfaces.keyboard = Some(keyboard);
                        println!("Acquired keyboard input interface.");
                    } else if !bits.contains(Capability::Keyboard)
                        && state.interfaces.keyboard.is_some()
                    {
                        if let Some(ref keyboard) = state.interfaces.keyboard {
                            keyboard.release();
                            state.interfaces.keyboard = None;
                        }
                    }
                }
                _ => {}
            },
            _ => {}
        }
    }
}
