// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2025, Nathan Gill

/*
    event_loop.rs:
        Contains the event loop for the `AuthenticatorState` struct
*/

use crate::{
    auth::state::AuthenticatorState,
    shared::{ffi::ForeignBuffer, interface::set_state, state::State},
    wayland::event::{event::Event, event_type::EventType},
};
use nix::poll::{PollFd, PollFlags, poll};
use pam::{Client, PamError};
use std::{
    ffi::{CStr, c_void},
    os::{fd::AsFd, raw::c_char},
    sync::atomic::Ordering,
};
use tracing::{debug, error, info, warn};

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

        Err("poll interrupted unexpectedly".into())
    }

    /// Attempt to authenticate the current user with the specified password
    fn authenticate(&mut self, password: String) -> Result<(), PamError> {
        let mut client = Client::with_password("system-auth")?;
        client
            .conversation_mut()
            .set_credentials(self.user.clone(), password);
        client.authenticate()
    }

    /// Set the application state to unlocking, to allow the Wayland thread
    /// to release the screen lock, and clear up.
    fn unlock(&mut self) {
        set_state(self.app_state.get(), State::Unlocking);
    }

    /// Handle a received AuthSubmit event
    ///
    /// This routine takes a received event and attempts to authenticate
    /// the current user based on the event-provided password.
    fn handle_auth_submit(&mut self, event: Event) {
        debug!("Received AuthSubmit event");

        let pfbu = event.param_1.raw() as *mut ForeignBuffer;
        if pfbu.is_null() {
            warn!("AuthSubmit contained NULL pointer");
            return;
        }

        let fbu = unsafe { &*pfbu };

        let c_pwd = unsafe { CStr::from_ptr(fbu.ptr as *const c_char) };
        match c_pwd.to_str() {
            Ok(password) => match self.authenticate(password.to_string()) {
                Ok(()) => {
                    info!("Authentication successful for '{}'", self.user);
                    self.unlock();
                }
                Err(e) => error!("Authentication error: {}", e),
            },
            Err(e) => {
                error!("Invalid password string received: {}", e);
            }
        }

        unsafe {
            // Drop the object, freeing the internal buffer...
            std::ptr::drop_in_place(pfbu);

            // ...and then deallocate the object itself.
            if let Some(d) = (*pfbu).dealloc {
                d(pfbu as *mut c_void);
            }
        }
    }

    /// Run the authenticator event loop until `stop_flag` is set
    pub fn run_event_loop(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        info!("Started authenticator event loop");

        while !self.stop_flag.load(Ordering::Relaxed) {
            let r = self.wait_for_event();

            if self.stop_flag.load(Ordering::Relaxed) {
                break;
            }

            r?;

            let event = self.read_event()?;

            if event.event_type == EventType::AuthSubmit {
                self.handle_auth_submit(event);
            }
        }

        info!("Exited authenticator event loop");

        Ok(())
    }
}
