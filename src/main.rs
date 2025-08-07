// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2025, Nathan Gill

/*
    main.rs:
        Main entry point, contains basic argument parsing, and initializes
        various state objects.
*/

pub mod shared;
pub mod wayland;

use crate::{shared::state::ApplicationState, wayland::state::WaylandState};
use std::{env, ffi::CString};
use tracing::{Level, debug, info};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: tlockr <qml path>");
        return Err("Invalid number of arguments".into());
    }

    tracing_subscriber::fmt()
        .with_timer(tracing_subscriber::fmt::time::uptime())
        .with_max_level(Level::DEBUG)
        .init();

    let now = chrono::Local::now();
    info!("tlockr started at {}", now.to_rfc3339());

    let qml_path_cstring = CString::new(args[1].clone())?;
    let qml_path_raw = qml_path_cstring.into_raw();

    debug!("Initializing Wayland interfaces...");

    let mut app_state = ApplicationState::new(qml_path_raw);
    let mut state = WaylandState::new(&mut app_state as *mut ApplicationState);

    let mut event_queue = state.initialize()?;

    state.roundtrip(&mut event_queue)?;

    debug!("Wayland interfaces initialized successfully.");

    state.run_event_loop(&mut event_queue)?;

    state.destroy_renderer();

    Ok(())
}
