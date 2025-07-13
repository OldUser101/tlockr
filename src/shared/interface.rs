// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2025, Nathan Gill

/*
    interface.rs:
        Safe getter/setter methods for the ApplicationState object.
*/

use crate::{
    shared::ffi::QmlRenderer,
    shared::state::{ApplicationState, State},
};
use std::os::raw::{c_char, c_int};

macro_rules! safe_getter {
    ($fn_name:ident, $field:ident, $return_type:ty) => {
        pub fn $fn_name(ptr: *const ApplicationState) -> Option<$return_type> {
            if ptr.is_null() {
                return None;
            }
            Some(unsafe { (*ptr).$field })
        }
    };
}

macro_rules! safe_setter {
    ($fn_name:ident, $field:ident, $param_type:ty) => {
        pub fn $fn_name(ptr: *mut ApplicationState, value: $param_type) -> bool {
            if ptr.is_null() {
                return false;
            }
            unsafe {
                (*ptr).$field = value;
            }
            true
        }
    };
}

safe_getter!(get_state, state, State);
safe_getter!(get_renderer_fd, renderer_fd, c_int);
safe_getter!(get_wayland_fd, wayland_fd, c_int);
safe_getter!(get_renderer, renderer, *mut QmlRenderer);
safe_getter!(get_qml_path, qml_path, *mut c_char);

safe_setter!(set_state, state, State);
safe_setter!(set_renderer_fd, renderer_fd, c_int);
safe_setter!(set_wayland_fd, wayland_fd, c_int);
safe_setter!(set_renderer, renderer, *mut QmlRenderer);
safe_setter!(set_qml_path, qml_path, *mut c_char);
