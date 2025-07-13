// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2025, Nathan Gill

/*
    empty.rs:
        Empty dispatch methods for various Wayland objects.
*/

use crate::wayland::state::WaylandState;
use wayland_client::{
    Connection, Dispatch, QueueHandle,
    protocol::{
        wl_compositor::WlCompositor, wl_display::WlDisplay, wl_shm::WlShm, wl_shm_pool::WlShmPool,
        wl_surface::WlSurface,
    },
};
use wayland_protocols::{
    ext::session_lock::v1::client::ext_session_lock_manager_v1::ExtSessionLockManagerV1,
    wp::viewporter::client::{wp_viewport::WpViewport, wp_viewporter::WpViewporter},
};

macro_rules! empty_dispatch {
    ($state:ty, $($proxy:ty),*) => {
        $(
            impl Dispatch<$proxy, ()> for $state {
                fn event(
                    _state: &mut Self,
                    _proxy: &$proxy,
                    _event: <$proxy as wayland_client::Proxy>::Event,
                    _data: &(),
                    _conn: &Connection,
                    _qh: &QueueHandle<Self>,
                ) {}
            }
        )*
    };
}

empty_dispatch! {
    WaylandState,
    WlDisplay,
    WlCompositor,
    WpViewporter,
    ExtSessionLockManagerV1,
    WlSurface,
    WlShm,
    WlShmPool,
    WpViewport
}
