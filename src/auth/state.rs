// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2025, Nathan Gill

/*
    state.rs:
        Definitions of `AuthenticatorState`
*/

use crate::shared::{interface::set_auth_write_fd, pipe::Pipe, state::ApplicationStatePtr};
use crate::wayland::event::event::Event;
use crate::wayland::event::event_param::EventParam;
use crate::wayland::event::event_type::EventType;
use std::{
    os::fd::{AsRawFd, FromRawFd, OwnedFd, RawFd},
    sync::atomic::AtomicBool,
};
use tracing::{error, info};
use uzers::get_current_username;

/// State enum sent for AuthStateUpdate events
///
/// This enum is C-compatible
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u64)]
pub enum AuthState {
    Pending = 0,
    Failed = 1,
    Success = 2,
}

impl TryFrom<u64> for AuthState {
    type Error = &'static str;

    fn try_from(tag: u64) -> Result<Self, Self::Error> {
        match tag {
            0 => Ok(AuthState::Pending),
            1 => Ok(AuthState::Failed),
            2 => Ok(AuthState::Success),
            _ => Err("Unknown AuthState tag"),
        }
    }
}

/// Holds the state of the authenticator thread
pub struct AuthenticatorState {
    pub auth_pipe: Option<Pipe>,
    pub app_state: ApplicationStatePtr,
    pub renderer_fd: RawFd,
    pub stop_flag: &'static AtomicBool,
    pub user: String,
}

impl AuthenticatorState {
    /// Create a new `AuthenticatorState` with a pointer to the global state.
    ///
    /// The global state pointer is held internally for future use.
    ///
    /// This function also obtains the username this process was run as for
    /// authentication.
    pub fn new(app_state: ApplicationStatePtr, stop_flag: &'static AtomicBool) -> Option<Self> {
        match get_current_username()?.into_string() {
            Ok(user) => {
                info!("Running authenticator for: '{}'", user);
                Some(Self {
                    auth_pipe: None,
                    app_state: app_state,
                    renderer_fd: -1,
                    stop_flag,
                    user,
                })
            }
            Err(os_str) => {
                error!("Failed to convert username string: {:?}", os_str);
                None
            }
        }
    }

    /// Send an AuthStateUpdate event to the renderer to indicate the
    /// authentication state has changed.
    ///
    /// This function can return an error if it fails to write an event.
    pub fn send_state_update(
        &mut self,
        state: AuthState,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let event = Event::new(
            EventType::AuthStateUpdate,
            EventParam::from(state as u64),
            EventParam::from(0u64),
        );
        let fd = unsafe { OwnedFd::from_raw_fd(self.renderer_fd) };
        event.write_to(&fd)
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
