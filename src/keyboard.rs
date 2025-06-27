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
use xkbcommon::xkb;

pub struct KeyboardMapping {
    _file: std::fs::File,
    mmap: memmap2::Mmap,
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
                            let mapping = KeyboardMapping {
                                _file: file,
                                mmap: mmap,
                            };
                            lock_state.interfaces.keymap = Some(mapping);
                            println!("Found xkbV1 format keymap.");
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
                println!("KEY");
                if let Some(ref keymap) = lock_state.interfaces.keymap {
                    let context = xkb::Context::new(xkb::CONTEXT_NO_FLAGS);
                    let keymap_str = std::str::from_utf8(&keymap.mmap).unwrap_or("");
                    let xkb_keymap = xkb::Keymap::new_from_string(
                        &context,
                        keymap_str.to_string(),
                        xkb::KEYMAP_FORMAT_TEXT_V1,
                        xkb::KEYMAP_COMPILE_NO_FLAGS,
                    );
                    if let Some(xkb_keymap) = xkb_keymap {
                        let state_xkb = xkb::State::new(&xkb_keymap);

                        let keycode = key + 8;
                        let keysym = state_xkb.key_get_one_sym(xkb::Keycode::from(keycode));
                        let utf8 = xkb::keysym_get_name(keysym);
                        println!("Key pressed: {}", utf8);
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
        _proxy: &WlSeat,
        event: <WlSeat as wayland_client::Proxy>::Event,
        _data: &(),
        _conn: &Connection,
        qh: &QueueHandle<Self>,
    ) {
        match event {
            wl_seat::Event::Capabilities { capabilities } => {
                if capabilities == WEnum::Value(Capability::Keyboard)
                    && state.interfaces.keyboard.is_none()
                {
                    let keyboard = state.interfaces.seat.as_ref().unwrap().get_keyboard(qh, ());
                    state.interfaces.keyboard = Some(keyboard);
                    println!("Acquired keyboard input interface.");
                }
            }
            _ => {}
        }
    }
}
