// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2025, Nathan Gill

/*
    event.rs:
        `Event` and `EventType` objects for event handling
*/

use crate::wayland::event::{param::EventParam, serial::next_serial};

/// The type of event that is being handled
///
/// This enum is C-compatible.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum EventType {
    Wayland = 0,
    Renderer = 1,
}

impl TryFrom<u32> for EventType {
    type Error = &'static str;

    /// Create an `EventType` from an event tag value
    fn try_from(tag: u32) -> Result<Self, Self::Error> {
        match tag {
            0 => Ok(EventType::Wayland),
            1 => Ok(EventType::Renderer),
            _ => Err("Invalid EventType tag"),
        }
    }
}

/// Event structure containing event serial, type, and parameters
///
/// This structure is C-compatible, and is intended to be sent by pipe using
/// a `CommunicationChannel` object. The event serial is a unique value that
/// can be used to identify an event object instance.
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Event {
    pub serial: u32,
    pub event_type: EventType,
    pub param_1: EventParam,
    pub param_2: EventParam,
}

impl Event {
    /// Create a new event object of a specified type, with two parameters
    ///
    /// The event serial is automatically generated using an atomic counter.
    pub fn new(event_type: EventType, param_1: EventParam, param_2: EventParam) -> Self {
        Self {
            serial: next_serial(),
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
