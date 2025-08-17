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
    Unlocking = 4,
    Unlocked = 5,
}

#[repr(C)]
pub struct ApplicationState {
    pub qml_path: *mut c_char,
    pub state: State,
    pub renderer: *mut QmlRenderer,
    pub renderer_write_fd: c_int,
    pub renderer_read_fd: c_int,
    pub auth_write_fd: c_int,
    pub auth_read_fd: c_int,
    pub output_width: c_int,
    pub output_height: c_int,
}

impl ApplicationState {
    pub fn new(qml_path: *mut c_char) -> Self {
        Self {
            qml_path: qml_path,
            state: State::None,
            renderer: std::ptr::null_mut(),
            renderer_write_fd: -1,
            renderer_read_fd: -1,
            auth_write_fd: -1,
            auth_read_fd: -1,
            output_width: -1,
            output_height: -1,
        }
    }
}

/// Pointer wrapper for `ApplicationState` for cross-thread use
pub struct ApplicationStatePtr(*mut ApplicationState);

unsafe impl Send for ApplicationStatePtr {}
unsafe impl Sync for ApplicationStatePtr {}

impl ApplicationStatePtr {
    /// Wrap a `ApplicationState` pointer
    pub fn new(ptr: *mut ApplicationState) -> Self {
        Self(ptr)
    }

    /// Get the stored `ApplicationState` pointer
    pub fn get(&self) -> *mut ApplicationState {
        self.0
    }
}
