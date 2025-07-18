// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2025, Nathan Gill

/*
    event.rs:
        Contains event loops that handle both Wayland events from an EventQueue
        and from an EventFd, used for signals from the renderer, via epoll.
*/

use crate::shared::{interface::get_state, state::State};
use crate::wayland::state::WaylandState;
use nix::sys::epoll::{Epoll, EpollCreateFlags, EpollEvent, EpollFlags, EpollTimeout};
use std::os::fd::BorrowedFd;
use std::os::fd::{AsRawFd, RawFd};
use wayland_client::EventQueue;
use wayland_client::backend::ReadEventsGuard;

const WAYLAND_EVENT_TAG: u64 = 0;
const RENDERER_EVENT_TAG: u64 = 1;

/// Enum for event tags used in the `epoll` event loop
enum EventType {
    Wayland,
    Renderer,
    Unknown(u64),
}

impl From<u64> for EventType {
    /// Return a new `EventType` object from a `u64`
    ///
    /// - A value of `0` corresponds to `EventType::Wayland`
    /// - A value of `1` corresponds to `EventType::Renderer`
    ///
    /// Any other value corresponds to `EventType::Unknown(tag)`
    fn from(tag: u64) -> Self {
        match tag {
            WAYLAND_EVENT_TAG => EventType::Wayland,
            RENDERER_EVENT_TAG => EventType::Renderer,
            other => EventType::Unknown(other),
        }
    }
}

/// Event loop structure to hold values used as part of the event loop
///
/// Contains a `nix::sys::Epoll` object which can be waited on
/// Contains an events array for processing
/// Contains the renderer file descriptor, which is added to `Epoll`
struct EventLoop {
    epoll: Epoll,
    events: [EpollEvent; 10],
    renderer_fd: BorrowedFd<'static>,
}

impl EventLoop {
    /// Create a new event loop given the raw renderer file descriptor
    ///
    /// The renderer file descriptor is added into `Epoll` automatically, and removed when dropping
    fn new(renderer_fd: RawFd) -> Result<Self, Box<dyn std::error::Error>> {
        let epoll = Epoll::new(EpollCreateFlags::empty())?;
        let events = [EpollEvent::empty(); 10];
        let renderer_fd = unsafe { BorrowedFd::borrow_raw(renderer_fd) };

        let renderer_event = EpollEvent::new(EpollFlags::EPOLLIN, RENDERER_EVENT_TAG);
        epoll.add(renderer_fd, renderer_event)?;

        Ok(Self {
            epoll,
            events,
            renderer_fd,
        })
    }
}

impl Drop for EventLoop {
    fn drop(&mut self) {
        let _ = self.epoll.delete(self.renderer_fd);
    }
}

/// This guard structure is used to ensure the Wayland file descriptor (given by a `ReadEventsGuard` object).
///
/// This structure contains a reference to an `Epoll` object (probably from an `EventLoop`).
/// The Wayland file descriptor is automatically removed from `Epoll` when dropped.
struct WaylandFdCleanup<'a> {
    epoll: &'a Epoll,
    fd: BorrowedFd<'a>,
}

impl Drop for WaylandFdCleanup<'_> {
    fn drop(&mut self) {
        let _ = self.epoll.delete(self.fd);
    }
}

impl WaylandState {
    /// This function processes all events in the `events` array
    ///
    /// Returns a `bool` indicating whether any Wayland events were received, which need further processing
    fn process_epoll_events(
        &mut self,
        events: &[EpollEvent],
        num_events: usize,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut wayland_event_received = false;

        for event in &events[..num_events] {
            match EventType::from(event.data()) {
                EventType::Wayland => {
                    wayland_event_received = true;
                }
                EventType::Renderer => {
                    self.handle_renderer_event()?;
                }
                EventType::Unknown(tag) => {
                    println!("Received unknown event: {}", tag);
                }
            }
        }

        Ok(wayland_event_received)
    }

    /// This function reads and dispatches waiting Wayland events
    ///
    /// The `ReadEventsGuard` object given to this function will be dropped
    fn handle_wayland_events(
        &mut self,
        read_guard: ReadEventsGuard,
        event_queue: &mut EventQueue<Self>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        read_guard.read()?;
        event_queue.dispatch_pending(self)?;
        Ok(())
    }

    /// Wait for, and dispatch events using `epoll`
    ///
    /// This functions waits for Wayland and renderer events using `epoll`.
    /// Upon receiving the events, they are dispatched to their relevant handlers via the `process_epoll_events` function.
    fn dispatch_events(
        &mut self,
        event_loop: &mut EventLoop,
        read_guard: ReadEventsGuard,
        event_queue: &mut EventQueue<Self>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let wayland_fd = read_guard.connection_fd();
        let wayland_event = EpollEvent::new(EpollFlags::EPOLLIN, WAYLAND_EVENT_TAG);

        event_loop.epoll.add(wayland_fd, wayland_event)?;

        // Use a scoped block here, so the Wayland file descriptor is dropped before moving `read_guard`
        let wayland_event_received = {
            // Ensure the Wayland file descriptor is cleaned up at the end of this block
            let _cleanup_guard = WaylandFdCleanup {
                epoll: &event_loop.epoll,
                fd: wayland_fd,
            };

            let num_events = event_loop
                .epoll
                .wait(&mut event_loop.events, EpollTimeout::NONE)?;

            self.process_epoll_events(&mut event_loop.events, num_events)?
        };

        if wayland_event_received {
            self.handle_wayland_events(read_guard, event_queue)?;
        } else {
            // Explicitly drop the read guard to cancel the read
            drop(read_guard);
        }

        Ok(())
    }

    /// This function returns a boolean value indicating whether the event loop should continue running
    fn continue_running(&self) -> Result<bool, Box<dyn std::error::Error>> {
        Ok(get_state(self.app_state)
            .ok_or::<Box<dyn std::error::Error>>("Failed to read app state".into())?
            != State::Unlocked)
    }

    /// Run the event loop until exit
    ///
    /// This function initializes event loop state and triggers event reading via `epoll`.
    pub fn run_event_loop(
        &mut self,
        event_queue: &mut EventQueue<Self>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut event_loop = EventLoop::new(
            self.renderer_read_fd
                .as_ref()
                .ok_or::<Box<dyn std::error::Error>>(
                    "Renderer read file descriptor not set".into(),
                )?
                .as_raw_fd(),
        )?;

        while self.continue_running()? {
            self.update_states(&event_queue)?;

            event_queue.flush()?;
            event_queue.dispatch_pending(self)?;

            if let Some(read_guard) = event_queue.prepare_read() {
                self.dispatch_events(&mut event_loop, read_guard, event_queue)?;
            }
        }

        Ok(())
    }
}
