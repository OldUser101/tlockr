// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2025, Nathan Gill

/*
    handler.rs:
        `EventHandler` trait for objects that handle communication events
*/

use std::os::fd::BorrowedFd;

use crate::wayland::{communication::manager::CommunicationManager, event::event::EventType};

/// Trait for objects that handle communication events
pub trait EventHandler {
    /// This function is called when a new event is available for this handler.
    ///
    /// The event can be read from the file descriptor that was returned by
    /// `get_file_descriptor`.
    ///
    /// If using `CommunicationManager`, this event will take the form of a
    /// pointer. `CommunicationManager` can read this event object.
    fn notify_event(&mut self) -> Result<(), Box<dyn std::error::Error>>;

    /// Return the event type corresponding to this handler
    fn event_type(&self) -> EventType;

    /// Get the file descriptor for this handler
    ///
    /// This is called during each event loop iteration.
    /// The handler should return either:
    /// - A self-managed file descriptor
    /// - Obtain a file descriptor from the `CommunicationManager`
    /// - `None` if this event does not need to be monitored
    fn get_file_descriptor(
        &mut self,
        comm_manager: Option<&CommunicationManager>,
    ) -> Option<BorrowedFd<'_>>;

    /// This function is called before file descriptors are added
    fn prepare_handler(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    /// This function is called after event processing is complete
    fn cleanup_handler(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}
