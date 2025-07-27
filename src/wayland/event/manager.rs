// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2025, Nathan Gill

/*
    manager.rs:
        Event manager structure and event handler registration.
*/

use crate::wayland::{
    communication::manager::CommunicationManager,
    event::{event::EventType, handler::EventHandler},
};
use std::collections::HashMap;

/// `EventManager` strcuture used for event handling and event loops
///
/// This structure is used to delegate events to various event handlers,
/// that are registered with this function.
pub struct EventManager {
    pub comm_manager: CommunicationManager,
    pub event_handlers: HashMap<EventType, Box<dyn EventHandler>>,
}

impl EventManager {
    /// Create a new event manager, with a `CommunicationManager`
    ///
    /// The `CommunicationManager` passed to this function is moved, and is
    /// used exclusively by this object, and registered event handlers.
    pub fn new(comm_manager: CommunicationManager) -> Self {
        Self {
            comm_manager: comm_manager,
            event_handlers: HashMap::new(),
        }
    }

    /// Register an event handler with an event
    ///
    /// The `handler` object must implement the `EventHandler` trait.
    /// The event type of the handler is inferred from the `EventHandler::event_type`
    /// function.
    pub fn register_handler<H>(&mut self, handler: H)
    where
        H: EventHandler + 'static,
    {
        let event_type = handler.event_type();
        self.event_handlers.insert(event_type, Box::new(handler));
    }
}
