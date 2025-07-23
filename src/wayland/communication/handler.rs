// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2025, Nathan Gill

/*
    handler.rs:
        `EventHandler` trait for objects that handle communication events
*/

use crate::wayland::communication::event::{Event, EventType};

/// Trait for objects that handle communication events
pub trait EventHandler {
    /// Process the given event
    ///
    /// The event object parsed to this function will be of the correct `EventType`,
    /// as specified by `event_type`.
    fn handle_event(&mut self, event: &Event) -> Result<(), Box<dyn std::error::Error>>;

    /// Return the event type corresponding to this handler
    fn event_type(&self) -> EventType;
}
