use crate::wayland::communication::event::{Event, EventType};

pub trait EventHandler {
    fn handle_event(&mut self, event: &Event) -> Result<(), Box<dyn std::error::Error>>;
    fn event_type(&self) -> EventType;
}
