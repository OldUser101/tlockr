use std::collections::HashMap;

use crate::wayland::{
    communication::manager::CommunicationManager,
    event::{event::EventType, handler::EventHandler},
};

pub struct EventManager {
    pub comm_manager: CommunicationManager,
    pub event_handlers: HashMap<EventType, Box<dyn EventHandler>>,
}

impl EventManager {
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
