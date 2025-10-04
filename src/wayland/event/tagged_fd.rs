// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2025, Nathan Gill

/*
    tagged_fd.rs:
        `TaggedFd` object for event sources and dispatcing.
*/

use std::os::fd::{AsFd, BorrowedFd, FromRawFd, OwnedFd, RawFd};

/// A file descriptor with a tag attached.
///
/// Intended to be used as part of event dispatching.
pub struct TaggedFd {
    tag: u64,
    fd: OwnedFd,
}

impl TaggedFd {
    /// Create a new `TaggedFd` with corresponding tag and file descriptor.
    pub fn new(tag: u64, fd: OwnedFd) -> Self {
        Self { tag, fd }
    }

    /// Create a new `TaggedFd` from a tag and raw file descriptor.
    ///
    /// The raw file descriptor stored in this object will be closed
    /// when this object is dropped.
    pub unsafe fn from_raw(tag: u64, fd: RawFd) -> Self {
        Self {
            tag,
            fd: unsafe { OwnedFd::from_raw_fd(fd) },
        }
    }

    /// Returns the u64 tag stored by this object
    pub fn tag(&self) -> u64 {
        self.tag
    }

    /// Returns the file desciptor stored by this object
    pub fn fd(&self) -> BorrowedFd<'_> {
        self.fd.as_fd()
    }
}
