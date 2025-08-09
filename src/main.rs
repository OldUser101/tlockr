// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2025, Nathan Gill

/*
    main.rs:
        Main entry point, contains basic argument parsing, and initializes
        various state objects.
*/

pub mod auth;
pub mod shared;
pub mod wayland;

use crate::{
    auth::state::AuthenticatorState,
    shared::state::{ApplicationState, ApplicationStatePtr},
    wayland::state::WaylandState,
};
use nix::libc;
use std::{env, ffi::CString, fs::OpenOptions, os::fd::AsRawFd};
use tracing::{Level, debug, error, info};

fn suppress_stderr() {
    let devnull = OpenOptions::new().write(true).open("/dev/null").unwrap();
    unsafe {
        libc::dup2(devnull.as_raw_fd(), libc::STDERR_FILENO);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: tlockr <qml path>");
        return Err("Invalid number of arguments".into());
    }

    let qml_path_cstring = CString::new(args[1].clone())?;
    let qml_path_raw = qml_path_cstring.into_raw();

    debug!("Initializing Wayland interfaces...");

    let mut app_state = ApplicationState::new(qml_path_raw);
    let app_state_ptr = ApplicationStatePtr::new(&mut app_state as *mut ApplicationState);
    let mut state = WaylandState::new(&mut app_state as *mut ApplicationState);
    let mut auth_state = AuthenticatorState::new(app_state_ptr);

    let mut event_queue = state.initialize()?;

    auth_state.initialize(
        state
            .renderer_write_pipe
            .as_ref()
            .unwrap()
            .write_fd()
            .as_raw_fd(),
    )?;

    let auth_thread = std::thread::spawn(move || {
        info!("Authentication thread started");

        match auth_state.run_event_loop() {
            Err(e) => {
                error!("{:?}", e);
            }
            _ => {}
        }

        info!("Authentication thread exited");
    });

    state.roundtrip(&mut event_queue)?;

    debug!("Wayland interfaces initialized successfully.");

    state.run_event_loop(&mut event_queue)?;

    state.destroy_renderer();

    let _ = auth_thread.join();

    Ok(())
}

fn main() {
    // Supress stderr, since `tracing` is used instead
    suppress_stderr();

    tracing_subscriber::fmt()
        .with_timer(tracing_subscriber::fmt::time::uptime())
        .with_max_level(Level::DEBUG)
        .init();

    let now = chrono::Local::now();
    info!("tlockr started at {}", now.to_rfc3339());

    match run() {
        Err(e) => {
            error!("{:?}", e);
        }
        _ => {}
    }

    let now = chrono::Local::now();
    info!("tlockr exited at {}", now.to_rfc3339())
}
