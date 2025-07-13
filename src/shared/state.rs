// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2025, Nathan Gill

/*
    state.rs:
        This file defines the ApplicationState object, which is C-compatible,
        for the shared state.
*/

use std::os::raw::{c_char, c_int};

use crate::shared::ffi::QmlRenderer;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum State {
    None = 0,
    Initialized = 1,
    Ready = 2,
    Locked = 3,
    Unlocked = 4,
}

#[repr(C)]
pub struct ApplicationState {
    pub qml_path: *mut c_char,
    pub state: State,
    pub renderer: *mut QmlRenderer,
    pub renderer_fd: c_int,
}

impl ApplicationState {
    pub fn new(qml_path: *mut c_char) -> Self {
        Self {
            qml_path: qml_path,
            state: State::None,
            renderer: std::ptr::null_mut(),
            renderer_fd: -1,
        }
    }
}
