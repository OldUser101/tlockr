// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2025, Nathan Gill

/*
    event_loop.rs:
        Contains the event loop for the `AuthenticatorState` struct
*/

use crate::{
    auth::state::AuthenticatorState,
    wayland::event::{event::Event, event_type::EventType},
};
use nix::poll::{PollFd, PollFlags, poll};
use std::{os::fd::AsFd, sync::atomic::Ordering};
use tracing::info;

impl AuthenticatorState {
    /// Read a single event from the authentictor event pipe
    ///
    /// This function may return an error if file descriptors are invalid or
    /// the event is not the correct size.
    fn read_event(&self) -> Result<Event, Box<dyn std::error::Error>> {
        let auth_fd = self
            .auth_pipe
            .as_ref()
            .ok_or::<Box<dyn std::error::Error>>("Authenticator pipe not initialized".into())?
            .read_fd();

        let mut event_buffer = [0u8; std::mem::size_of::<Event>()];
        let bytes_read = nix::unistd::read(auth_fd, &mut event_buffer)?;

        if bytes_read != std::mem::size_of::<Event>() {
            return Err(format!(
                "Failed to read authenticator event pipe, expected {} bytes, got {}.",
                std::mem::size_of::<Event>(),
                bytes_read
            )
            .into());
        }

        // https://stackoverflow.com/questions/42499049/how-to-transmute-a-u8-buffer-to-struct-in-rust
        let (head, body, _tail) = unsafe { event_buffer.align_to::<Event>() };
        assert!(head.is_empty(), "Event data was not aligned");
        let event = body[0];

        Ok(event)
    }

    /// Wait for an event to be recieved on the pipe
    fn wait_for_event(&self) -> Result<(), Box<dyn std::error::Error>> {
        let auth_pipe = self
            .auth_pipe
            .as_ref()
            .ok_or::<Box<dyn std::error::Error>>("Authenticator pipe not initialized".into())?;
        let borrowed_fd = auth_pipe.read_fd().as_fd();

        let mut pfds = [PollFd::new(borrowed_fd, PollFlags::POLLIN)];

        while !self.stop_flag.load(Ordering::Relaxed) {
            let res = poll(&mut pfds, 100u16)?;
            if res > 0 {
                if let Some(revents) = pfds[0].revents() {
                    if revents.contains(PollFlags::POLLIN) {
                        return Ok(());
                    }
                }
            }
        }

        Err("poll interrupted unexpected".into())
    }

    /// Run the authenticator event loop until `stop_flag` is set
    pub fn run_event_loop(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        info!("Started authenticator event loop");

        while !self.stop_flag.load(Ordering::Relaxed) {
            self.wait_for_event()?;

            let event = self.read_event()?;

            if event.event_type == EventType::AuthSubmit {
                info!("Received AuthSubmit event");
            }
        }

        info!("Exited authenticator event loop");

        Ok(())
    }
}
