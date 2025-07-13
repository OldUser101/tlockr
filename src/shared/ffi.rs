// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2025, Nathan Gill

/*
    ffi.rs:
        Prototypes for C++ renderer FFI.
*/

use std::os::raw::{c_char, c_int, c_void};

use crate::shared::state::ApplicationState;

#[repr(C)]
pub struct QmlRenderer {
    _private: [u8; 0],
}

#[repr(C)]
pub struct BufferData {
    pub in_use: c_int,
    pub index: c_int,
    pub data: *mut c_void,
}

#[repr(C)]
pub struct RendererEvent {
    pub buffer: *mut c_void,
}

pub type RsGetBufferCallback = unsafe extern "C" fn(user_data: *mut c_void) -> *mut c_void;

unsafe extern "C" {
    pub fn initialize_renderer(
        width: c_int,
        height: c_int,
        qml_path: *const c_char,
        app_state: *mut ApplicationState,
    ) -> *mut QmlRenderer;

    pub fn start_renderer(renderer: *mut QmlRenderer) -> c_int;

    pub fn set_callbacks(
        renderer: *mut QmlRenderer,
        get_buffer: RsGetBufferCallback,
        user_data: *mut c_void,
    );

    pub fn cleanup_renderer(renderer: *mut QmlRenderer);
}
