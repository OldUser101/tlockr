// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2025, Nathan Gill

/*
    pipe.rs:
        Contains a structure for holding pipe file descriptors used
        in Wayland <-> Renderer communication
*/

use std::os::fd::OwnedFd;

/// Structure to hold a pair of pipe file descriptors
///
/// File descriptors are automatically closed when this object is dropped.
pub struct Pipe {
    read_fd: OwnedFd,
    write_fd: OwnedFd,
}

impl Pipe {
    /// Creates a new `Pipe` object, with a pair of pipe file descriptors open.
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let (read_fd, write_fd) = nix::unistd::pipe()?;

        Ok(Self { read_fd, write_fd })
    }

    /// Returns a reference to the read file descriptor for the pipe
    pub fn read_fd(&self) -> &OwnedFd {
        &self.read_fd
    }

    /// Returns a reference to the write file descriptor for the pipe
    pub fn write_fd(&self) -> &OwnedFd {
        &self.write_fd
    }
}
