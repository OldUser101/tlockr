// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2025, Nathan Gill

/*
    loop.rs:
        Event loop and state that is used to periodically dispatch events from
        various sources.
*/

use crate::shared::state::ApplicationState;
use crate::shared::{interface::get_state, state::State};
use crate::wayland::communication::manager::CommunicationManager;
use crate::wayland::event::event::EventType;
use crate::wayland::event::handler::EventHandler;
use crate::wayland::event::manager::EventManager;
use nix::sys::epoll::{Epoll, EpollCreateFlags, EpollEvent, EpollFlags, EpollTimeout};
use std::collections::HashMap;
use std::os::fd::{AsRawFd, BorrowedFd, RawFd};

/// Event loop structure to hold values used as part of the event loop
///
/// Contains a `nix::sys::Epoll` object which can be waited on
/// Contains an events array for processing
/// Contains the renderer file descriptor, which is added to `Epoll`
struct EventLoopState {
    epoll: Epoll,
    events: [EpollEvent; 10],
}

impl EventLoopState {
    /// Create a new event loop given the raw renderer file descriptor
    ///
    /// The renderer file descriptor is added into `Epoll` automatically, and removed when dropping
    fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let epoll = Epoll::new(EpollCreateFlags::empty())?;
        let events = [EpollEvent::empty(); 10];

        Ok(Self { epoll, events })
    }

    /// Register the file descriptors of all `event_handlers` with an `Epoll`
    ///
    /// This function returns an `EpollCleanupGuard`, which handles the
    /// lifetime of all file descriptors and events registered with it.
    pub fn register_file_descriptors<'a>(
        &'a mut self,
        comm_manager: &CommunicationManager,
        event_handlers: &'a mut HashMap<EventType, Box<dyn EventHandler>>,
    ) -> Result<EpollCleanupGuard<'a>, Box<dyn std::error::Error>> {
        for (_, handler) in event_handlers.iter_mut() {
            handler.prepare_handler()?;
        }

        let mut registered_fds = Vec::new();
        for (event_type, event_handler) in event_handlers.iter_mut() {
            if let Some(fd) = event_handler.get_file_descriptor(Some(comm_manager)) {
                let raw_fd = fd.as_raw_fd();

                if raw_fd < 0 {
                    println!("Invalid FD {} for {:?}", raw_fd, event_type);
                    continue;
                }

                let event = EpollEvent::new(EpollFlags::EPOLLIN, *event_type as u64);
                self.epoll.add(fd, event)?;
                registered_fds.push((*event_type, raw_fd));
            }
        }

        Ok(EpollCleanupGuard {
            event_loop: self,
            registered_fds,
            event_handlers,
        })
    }
}

/// Cleanup guard for managing events registered with `Epoll`
///
/// This structure holds the entire event loop state, and handles event removal
/// when dropped. Event handlers are also signalled via `cleanup_handler`.
struct EpollCleanupGuard<'a> {
    event_loop: &'a mut EventLoopState,
    registered_fds: Vec<(EventType, RawFd)>,
    event_handlers: &'a mut HashMap<EventType, Box<dyn EventHandler>>,
}

impl<'a> EpollCleanupGuard<'a> {
    /// Wait for events to be received
    ///
    /// This method returns the number of events received. The event data
    /// can be accessed through the original `EventLoopState` that created
    /// this object.
    ///
    /// After this method is called, this object **must** be dropped to allow
    /// events to be read from the `EventLoopState`, whose ownership was
    /// previously transferred to this object.
    pub fn wait(&mut self) -> Result<usize, Box<dyn std::error::Error>> {
        Ok(self
            .event_loop
            .epoll
            .wait(&mut self.event_loop.events, EpollTimeout::NONE)?)
    }
}

impl Drop for EpollCleanupGuard<'_> {
    /// Drop this object, releasing all file descriptors and signalling handlers
    fn drop(&mut self) {
        for (_, raw_fd) in &self.registered_fds {
            let fd = unsafe { BorrowedFd::borrow_raw(*raw_fd) };
            let _ = self.event_loop.epoll.delete(fd);
        }

        for (_, handler) in self.event_handlers.iter_mut() {
            let _ = handler.cleanup_handler();
        }
    }
}

impl EventManager {
    /// This function returns a boolean value indicating whether the event loop should continue running
    fn continue_running(
        &self,
        app_state: *mut ApplicationState,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        Ok(get_state(app_state)
            .ok_or::<Box<dyn std::error::Error>>("Failed to read app state".into())?
            != State::Unlocked)
    }

    /// Wait for, and dispatch, events through `Epoll`
    ///  
    /// This function registers file descriptors from event handlers. These
    /// event handlers are notified when an event is available to be processed.
    /// File descriptors are removed from the `Epoll` automatically when
    /// exiting this function.
    fn dispatch_events(
        &mut self,
        event_loop: &mut EventLoopState,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut cleanup_guard =
            event_loop.register_file_descriptors(&self.comm_manager, &mut self.event_handlers)?;

        let num_events = cleanup_guard.wait()?;

        drop(cleanup_guard);

        for event in &event_loop.events[..num_events] {
            let event_type = EventType::try_from(event.data() as u32)?;

            if let Some(handler) = self.event_handlers.get_mut(&event_type) {
                handler.notify_event()?;
            }
        }

        Ok(())
    }

    /// Run the event loop until the lock exits
    pub fn run_event_loop(
        &mut self,
        app_state: *mut ApplicationState,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut event_loop = EventLoopState::new()?;

        while self.continue_running(app_state)? {
            self.dispatch_events(&mut event_loop)?;
        }

        Ok(())
    }
}
