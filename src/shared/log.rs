// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2025, Nathan Gill

/*
    log.rs:
        Exposed logging wrappers for external code
*/

use std::ffi::{CStr, c_char};

unsafe fn cstr_to_str<'a>(ptr: *const c_char) -> &'a str {
    unsafe { CStr::from_ptr(ptr).to_str().unwrap_or("<invalid utf8>") }
}

#[unsafe(no_mangle)]
pub extern "C" fn trace_log(file: *const c_char, msg: *const c_char) {
    unsafe {
        let file_str = cstr_to_str(file);
        let msg_str = cstr_to_str(msg);
        log::trace!(target: file_str, "{}", msg_str);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn debug_log(file: *const c_char, msg: *const c_char) {
    unsafe {
        let file_str = cstr_to_str(file);
        let msg_str = cstr_to_str(msg);
        log::debug!(target: file_str, "{}", msg_str);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn info_log(file: *const c_char, msg: *const c_char) {
    unsafe {
        let file_str = cstr_to_str(file);
        let msg_str = cstr_to_str(msg);
        log::info!(target: file_str, "{}", msg_str);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn warn_log(file: *const c_char, msg: *const c_char) {
    unsafe {
        let file_str = cstr_to_str(file);
        let msg_str = cstr_to_str(msg);
        log::warn!(target: file_str, "{}", msg_str);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn error_log(file: *const c_char, msg: *const c_char) {
    unsafe {
        let file_str = cstr_to_str(file);
        let msg_str = cstr_to_str(msg);
        log::error!(target: file_str, "{}", msg_str);
    }
}
