use std::{collections::HashMap, os::fd::RawFd};

use nix::sys::epoll::EpollFlags;

use crate::wayland::communication::{
    channel::CommunicationChannel,
    event::{Event, EventType},
    handler::EventHandler,
    param::EventParam,
};

pub struct CommunicationManager {
    channels: HashMap<EventType, CommunicationChannel>,
    event_handlers: HashMap<EventType, Box<dyn EventHandler>>,
}

impl CommunicationManager {
    pub fn new() -> Self {
        Self {
            channels: HashMap::new(),
            event_handlers: HashMap::new(),
        }
    }

    pub fn get_channel(&self, event_type: EventType) -> Option<&CommunicationChannel> {
        self.channels.get(&event_type)
    }

    pub fn create_channel(
        &mut self,
        event_type: EventType,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let channel = CommunicationChannel::new()?;
        self.channels.insert(event_type, channel);
        Ok(())
    }

    pub fn register_handler<H>(&mut self, handler: H)
    where
        H: EventHandler + 'static,
    {
        let event_type = handler.event_type();
        self.event_handlers.insert(event_type, Box::new(handler));
    }

    pub fn dispatch_event(&mut self, event: Event) -> Result<(), Box<dyn std::error::Error>> {
        let event_type = event.event_type;

        let handler = self
            .event_handlers
            .get_mut(&event_type)
            .ok_or::<Box<dyn std::error::Error>>("No handler set for event type".into())?;

        handler.handle_event(&event)?;

        Ok(())
    }

    pub fn send_event<P1, P2>(
        &mut self,
        event_type: EventType,
        param_1: P1,
        param_2: P2,
    ) -> Result<(), Box<dyn std::error::Error>>
    where
        P1: Into<EventParam>,
        P2: Into<EventParam>,
    {
        let channel = self
            .channels
            .get_mut(&event_type)
            .ok_or::<Box<dyn std::error::Error>>("No channel for specified event type".into())?;

        let event = Event::new(event_type, param_1.into(), param_2.into());
        channel.send_event(event)?;
        Ok(())
    }

    pub fn receive_event(
        &mut self,
        event_type: EventType,
    ) -> Result<Event, Box<dyn std::error::Error>> {
        let channel = self
            .channels
            .get_mut(&event_type)
            .ok_or::<Box<dyn std::error::Error>>("No channel for specified event type".into())?;

        let event = channel.receive_event()?;

        Ok(event)
    }

    pub fn get_fds(&self) -> Vec<(RawFd, EpollFlags, u64)> {
        self.channels
            .iter()
            .map(|(event_type, channel)| {
                let fd = channel.read_fd_raw();
                let events = EpollFlags::EPOLLIN;
                let data = *event_type as u64;
                (fd, events, data)
            })
            .collect()
    }

    pub fn handle_event(
        &mut self,
        event_type: EventType,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let event = self.receive_event(event_type)?;
        self.dispatch_event(event)?;
        Ok(())
    }
}
