// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2025, Nathan Gill

/*
    main.rs:
        Entry point for the application.
*/

pub mod auth;
pub mod buffer;
pub mod cli;
pub mod event;
pub mod ffi;
pub mod graphics;
pub mod input;
pub mod lock;
pub mod registry;
pub mod shared;
pub mod wayland;

use cli::start;

fn main() {
    start();
}
