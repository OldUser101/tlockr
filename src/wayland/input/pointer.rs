// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2025, Nathan Gill

/*
    pointer.rs:
        Redirects pointer events to Qt
*/

use crate::wayland::state::WaylandState;
use wayland_client::{Connection, Dispatch, QueueHandle, protocol::wl_pointer::WlPointer};

impl Dispatch<WlPointer, ()> for WaylandState {
    fn event(
        _state: &mut Self,
        _proxy: &WlPointer,
        _event: <WlPointer as wayland_client::Proxy>::Event,
        _data: &(),
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
    ) {
    }
}
