// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2025, Nathan Gill

/*
    channel.rs:
        Pipe abstraction for one-way communication between threads.
*/

use crate::wayland::communication::event::Event;
use crate::wayland::communication::pipe::Pipe;
use std::os::fd::{AsRawFd, RawFd};

/// This structure represents a one-way communication channel using a pipe
///
/// It is intended that the owner of this object shares either the read
/// or write file descriptor with another thread. This allows one way
/// communication between the two threads using `Event` objects.
///
/// The pipe file descriptors held by this object are closed when this
/// object is dropped.
pub struct CommunicationChannel {
    pipe: Pipe,
}

impl CommunicationChannel {
    /// Create a new `CommunicationChannel`
    ///
    /// This function opens the pipe file descriptors that are used for
    /// communication.
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self { pipe: Pipe::new()? })
    }

    /// Write an event object into the event pipe
    pub fn send_event(&self, event: Event) -> Result<(), Box<dyn std::error::Error>> {
        let event_bytes = unsafe {
            std::slice::from_raw_parts(
                &event as *const Event as *const u8,
                std::mem::size_of::<Event>(),
            )
        };

        let write_fd = self.pipe.write_fd();

        let bytes_written = nix::unistd::write(write_fd, &event_bytes)?;
        if bytes_written != std::mem::size_of::<Event>() {
            return Err("Failed to write event bytes".into());
        }

        Ok(())
    }

    /// Read and return a single event from the event pipe
    pub fn receive_event(&self) -> Result<Event, Box<dyn std::error::Error>> {
        let mut event_bytes = [0u8; std::mem::size_of::<Event>()];
        let read_fd = self.pipe.read_fd();

        let bytes_read = nix::unistd::read(read_fd, &mut event_bytes)?;
        if bytes_read != std::mem::size_of::<Event>() {
            return Err("Failed to read event bytes".into());
        }

        let event = unsafe { std::ptr::read(event_bytes.as_ptr() as *const Event) };

        Ok(event)
    }

    /// Returns the raw file descriptor for the readable end of the pipe
    pub fn read_fd_raw(&self) -> RawFd {
        self.pipe.read_fd().as_raw_fd()
    }

    /// Returns the raw file descriptor for the writable end of the pipe
    pub fn write_fd_raw(&self) -> RawFd {
        self.pipe.write_fd().as_raw_fd()
    }
}
