// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2025, Nathan Gill

/*
    event.rs:
        `Event` and `EventType` objects for event handling
*/

use crate::wayland::event::{event_param::EventParam, event_type::EventType};

/// Event structure containing event serial, type, and parameters
///
/// This structure is C-compatible, and is intended to be sent with a `Pipe`
/// object.
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Event {
    pub event_type: EventType,
    pub param_1: EventParam,
    pub param_2: EventParam,
}

impl Event {
    /// Create a new event object of a specified type, with two parameters
    pub fn new(event_type: EventType, param_1: EventParam, param_2: EventParam) -> Self {
        Self {
            event_type,
            param_1,
            param_2,
        }
    }

    /// Dereference a constant event pointer into an `Event`
    ///
    /// This function handles raw pointers, thus, is unsafe.
    /// This function is not responsible for proper pointer management.
    pub unsafe fn from_ptr(ptr: *const Event) -> Self {
        unsafe { std::ptr::read(ptr) }
    }

    /// Dereference a mutable event pointer into an `Event`
    ///
    /// This function handles raw pointers, thus, is unsafe.
    /// This function is not responsible for proper pointer management.
    pub unsafe fn from_mut_ptr(ptr: *mut Event) -> Self {
        unsafe { std::ptr::read(ptr) }
    }
}
