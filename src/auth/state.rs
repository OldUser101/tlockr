// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2025, Nathan Gill

/*
    state.rs:
        Definitions of `AuthenticatorState`
*/

use crate::shared::{interface::set_auth_write_fd, pipe::Pipe, state::ApplicationStatePtr};
use std::{
    os::fd::{AsRawFd, RawFd},
    sync::atomic::AtomicBool,
};

/// Holds the state of the authenticator thread
pub struct AuthenticatorState {
    pub auth_pipe: Option<Pipe>,
    pub app_state: ApplicationStatePtr,
    pub renderer_fd: RawFd,
    pub stop_flag: &'static AtomicBool,
}

impl AuthenticatorState {
    /// Create a new `AuthenticatorState` with a pointer to the global state.
    ///
    /// The global state pointer is held internally for future use.
    pub fn new(app_state: ApplicationStatePtr, stop_flag: &'static AtomicBool) -> Self {
        Self {
            auth_pipe: None,
            app_state: app_state,
            renderer_fd: -1,
            stop_flag,
        }
    }

    /// Initialize this `AuthenticatorState` object
    ///
    /// This function requires the `RawFd` for the renderer's input pipe,
    /// this is used to send authentication state events to the renderer.
    pub fn initialize(&mut self, renderer_fd: RawFd) -> Result<(), Box<dyn std::error::Error>> {
        let auth_pipe = Pipe::new()?;

        set_auth_write_fd(self.app_state.get(), auth_pipe.write_fd().as_raw_fd());

        self.auth_pipe = Some(auth_pipe);

        self.renderer_fd = renderer_fd;

        Ok(())
    }
}
