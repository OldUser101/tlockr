use std::collections::HashMap;

use crate::wayland::{
    communication::manager::CommunicationManager,
    event::{
        event::{Event, EventType},
        handler::EventHandler,
    },
};

pub struct EventManager<'a> {
    pub comm_manager: Option<&'a CommunicationManager>,
    pub event_handlers: HashMap<EventType, Box<dyn EventHandler>>,
}

impl<'a> EventManager<'a> {
    pub fn new(comm_manager: &'a CommunicationManager) -> Self {
        Self {
            comm_manager: Some(comm_manager),
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

    /// Dispatch an `Event` object to it's relevant handler.
    ///
    /// This handler must be registered with `register_handler` before calling
    /// this function.
    pub fn dispatch_event(&mut self, event: Event) -> Result<(), Box<dyn std::error::Error>> {
        let event_type = event.event_type;

        let handler = self
            .event_handlers
            .get_mut(&event_type)
            .ok_or::<Box<dyn std::error::Error>>("No handler set for event type".into())?;

        handler.handle_event(&event)?;

        Ok(())
    }
}
