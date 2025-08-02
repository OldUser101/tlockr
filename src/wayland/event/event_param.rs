// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2025, Nathan Gill

/*
    event_param.rs:
        Event parameter structure and conversions
*/

use std::os::raw::c_void;

/// Structure for holding parameters for `Event` objects
///
/// This structure is fully C-compatible.
///
/// This structure stores a single `u64` which can be converted to other
/// types using the `From<EventParam>` trait.
#[derive(Debug, Clone, Copy)]
pub struct EventParam(u64);

impl EventParam {
    /// Returns a new `EventType` from a value
    ///
    /// The type of this value must implement the `From` trait for `EventParam`
    pub fn new<T: Into<EventParam>>(value: T) -> Self {
        value.into()
    }

    /// Return the currently held value as type `T`
    ///
    /// Type `T` must have the trait `From<EventParam>` implemented.
    pub fn as_<T: From<EventParam>>(self) -> T {
        T::from(self)
    }

    /// Return the raw `u64` held by this object
    pub fn raw(self) -> u64 {
        self.0
    }
}

/*
    Conversions to/from `EventParam` and other types
*/

impl From<EventParam> for *mut c_void {
    fn from(param: EventParam) -> Self {
        param.0 as *mut c_void
    }
}

impl From<*mut c_void> for EventParam {
    fn from(ptr: *mut c_void) -> Self {
        EventParam(ptr as u64)
    }
}

impl From<*const c_void> for EventParam {
    fn from(ptr: *const c_void) -> Self {
        EventParam(ptr as u64)
    }
}
