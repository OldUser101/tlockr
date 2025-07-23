// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2025, Nathan Gill

/*
    manager.rs:
        Communication manager structure for creating communication channels, and dispatching events
*/

use crate::wayland::communication::{
    channel::CommunicationChannel,
    event::{Event, EventType},
    handler::EventHandler,
    param::EventParam,
};
use nix::sys::epoll::EpollFlags;
use std::{collections::HashMap, os::fd::RawFd};

/// Structure for creating and managing communication channels, and dispatching events
pub struct CommunicationManager {
    channels: HashMap<EventType, CommunicationChannel>,
    event_handlers: HashMap<EventType, Box<dyn EventHandler>>,
}

impl CommunicationManager {
    /// Create a new `CommunicationManager`
    pub fn new() -> Self {
        Self {
            channels: HashMap::new(),
            event_handlers: HashMap::new(),
        }
    }

    /// Get a `&CommunciationChannel` corresponding to an `EventType`
    ///
    /// If there is no corresponding channel, `Option::None` will be returned.
    pub fn get_channel(&self, event_type: EventType) -> Option<&CommunicationChannel> {
        self.channels.get(&event_type)
    }

    /// Open a new `CommunciationChannel` for the given `EventType`
    ///
    /// The opened channel is stored internally, and can be retrieved with
    /// the `get_channel` function.
    pub fn create_channel(
        &mut self,
        event_type: EventType,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let channel = CommunicationChannel::new()?;
        self.channels.insert(event_type, channel);
        Ok(())
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

    /// Construct and send an event object over the channel specified by `event_type`
    ///
    /// `param_1` and `param_2` must be compatible with `EventType`.
    ///
    /// An error is returned if there is no channel corresponding to `event_type`.
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

    /// Read a single `Event` object from the channel specified by `event_type`
    ///
    /// An error is returned if there is no channel corresponding to `event_type`.
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

    /// Get read file descriptors for all pipes stored by the current `CommunicationManager`
    /// instance.
    ///
    /// The returned data is of an `epoll` compatible format, containing the
    /// file descriptor, `EpollFlags`, and data.
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

    /// Receive and dispatch a single event of type `event_type`.
    pub fn handle_event(
        &mut self,
        event_type: EventType,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let event = self.receive_event(event_type)?;
        self.dispatch_event(event)?;
        Ok(())
    }
}
