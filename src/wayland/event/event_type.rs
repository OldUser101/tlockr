// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2025, Nathan Gill

/*
    event_type.rs:
        This file contains the `EventType` enum and conversions
*/

/// The type of event that is being handled
///
/// This enum is C-compatible.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u64)]
pub enum EventType {
    Wayland = 1,
    Renderer = 2,

    KeyboardKeymap = 3,
    KeyboardKey = 4,
    KeyboardModifiers = 5,
    KeyboardRepeatInfo = 6,

    PointerMotion = 7,
    PointerButton = 8,

    AuthSubmit = 9,
    AuthPending = 10,
    AuthFail = 11,
    AuthSuccess = 12,
}

impl TryFrom<u64> for EventType {
    type Error = &'static str;

    /// Create an `EventType` from an event tag value
    fn try_from(tag: u64) -> Result<Self, Self::Error> {
        match tag {
            1 => Ok(EventType::Wayland),
            2 => Ok(EventType::Renderer),

            3 => Ok(EventType::KeyboardKeymap),
            4 => Ok(EventType::KeyboardKey),
            5 => Ok(EventType::KeyboardModifiers),
            6 => Ok(EventType::KeyboardRepeatInfo),

            7 => Ok(EventType::PointerMotion),
            8 => Ok(EventType::PointerButton),

            9 => Ok(EventType::AuthSubmit),
            10 => Ok(EventType::AuthPending),
            11 => Ok(EventType::AuthFail),
            12 => Ok(EventType::AuthSuccess),
            _ => Err("Invalid EventType tag"),
        }
    }
}
