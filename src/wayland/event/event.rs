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
use std::os::fd::AsRawFd;
use std::os::fd::BorrowedFd;
use wayland_client::EventQueue;
use wayland_client::backend::ReadEventsGuard;

const WAYLAND_EVENT_TAG: u64 = 0;
const RENDERER_EVENT_TAG: u64 = 1;

impl WaylandState {
    fn dispatch_events(
        &mut self,
        epoll: &Epoll,
        events: &mut [EpollEvent],
        read_guard: ReadEventsGuard,
        event_queue: &mut EventQueue<Self>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let wayland_fd = read_guard.connection_fd();

        let wayland_event = EpollEvent::new(EpollFlags::EPOLLIN, WAYLAND_EVENT_TAG);
        epoll.add(wayland_fd, wayland_event)?;

        let num_events = epoll.wait(events, EpollTimeout::NONE)?;

        let mut wayland_event_received = false;
        for i in 0..num_events {
            match events[i].data() {
                WAYLAND_EVENT_TAG => wayland_event_received = true,
                RENDERER_EVENT_TAG => unsafe {
                    self.handle_renderer_event()?;
                },
                _ => {}
            }
        }

        // We don't want this anymore, but we have to cleanup before calling `read_guard.read()`
        epoll.delete(wayland_fd)?;

        if wayland_event_received {
            read_guard.read()?;
            event_queue.dispatch_pending(self)?;
        }

        Ok(())
    }

    pub fn run_event_loop(
        &mut self,
        event_queue: &mut EventQueue<Self>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let epoll = Epoll::new(EpollCreateFlags::empty())?;
        let mut events = [EpollEvent::empty(); 10];

        let renderer_fd_raw = self.renderer_read_fd.as_ref().unwrap().as_raw_fd();
        let renderer_fd = unsafe { BorrowedFd::borrow_raw(renderer_fd_raw) };

        let renderer_event = EpollEvent::new(EpollFlags::EPOLLIN, RENDERER_EVENT_TAG);
        epoll.add(renderer_fd, renderer_event)?;

        while get_state(self.app_state).unwrap() != State::Unlocked {
            self.update_states(&event_queue)?;

            event_queue.flush()?;
            event_queue.dispatch_pending(self)?;

            if let Some(read_guard) = event_queue.prepare_read() {
                self.dispatch_events(&epoll, &mut events, read_guard, event_queue)?;
            }
        }

        epoll.delete(renderer_fd)?;

        Ok(())
    }
}
