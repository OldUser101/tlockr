// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2025, Nathan Gill

/*
    output.rs:
        Contains a dispatch method to obtain the output display dimensions.
*/

use crate::wayland::state::WaylandState;
use wayland_client::{
    Connection, Dispatch, QueueHandle,
    protocol::wl_output::{self, WlOutput},
};

impl Dispatch<WlOutput, ()> for WaylandState {
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
                    state.width = width;
                    state.height = height;
                }
            }
            wl_output::Event::Done => {
                state.output_configured = true;
            }
            _ => {}
        }
    }
}
