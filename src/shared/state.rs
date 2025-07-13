use std::os::raw::{c_char, c_int};

use crate::wayland::ffi::QmlRenderer;

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
    pub wayland_fd: c_int,
}

impl ApplicationState {
    pub fn new(qml_path: *mut c_char) -> Self {
        Self {
            qml_path: qml_path,
            state: State::None,
            renderer: std::ptr::null_mut(),
            renderer_fd: -1,
            wayland_fd: -1,
        }
    }
}
