// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2025, Nathan Gill

/*
    main.rs:
        Main entry point, contains basic argument parsing, and initializes
        various state objects.
*/

pub mod shared;
pub mod wayland;

use std::{env, ffi::CString};

use crate::{
    shared::state::ApplicationState,
    wayland::{
        communication::manager::CommunicationManager, event::manager::EventManager,
        state::WaylandState,
    },
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: tlockr <qml path>");
        return Err("Invalid number of arguments".into());
    }

    let qml_path_cstring = CString::new(args[1].clone())?;
    let qml_path_raw = qml_path_cstring.into_raw();

    println!("Initializing Wayland interfaces...");

    let mut app_state = ApplicationState::new(qml_path_raw);
    let comm_manager = CommunicationManager::new();
    let mut event_manager = EventManager::new(&comm_manager);
    let mut state = WaylandState::new(&mut app_state as *mut ApplicationState);

    let mut event_queue = state.initialize()?;

    state.roundtrip(&mut event_queue)?;

    println!("Wayland interfaces initialized successfully.");

    event_manager.run_event_loop(&mut app_state as *mut ApplicationState)?;

    state.destroy_renderer();

    Ok(())
}
