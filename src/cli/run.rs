// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2025, Nathan Gill

/*
    run.rs:
        Functions and configuration for running the screen locker
*/

use crate::auth::AuthenticatorState;
use crate::config::{RunConfig, ThemeRoot, resolve_theme};
use crate::shared::{ApplicationState, ApplicationStatePtr};
use crate::wayland::WaylandState;

use nix::libc;
use std::os::unix::ffi::OsStrExt;
use std::{
    ffi::CString,
    fs::OpenOptions,
    os::fd::AsRawFd,
    path::PathBuf,
    sync::atomic::{AtomicBool, Ordering},
};
use tracing::{debug, error, info};

/// Flag that signals the authentication thread to exit
static AUTH_STOP_FLAG: AtomicBool = AtomicBool::new(false);

pub const DEFAULT_THEME_NAME: &str = "default";
pub const DEFAULT_QML_NAME: &str = "main.qml";

/// Redirect stderr to /dev/null
pub fn suppress_stderr() {
    let devnull = OpenOptions::new().write(true).open("/dev/null").unwrap();
    unsafe {
        libc::dup2(devnull.as_raw_fd(), libc::STDERR_FILENO);
    }
}

/// Run tlockr
///
/// This function is called by main to allow for better handling of errors
pub fn run_lock(config: &RunConfig) -> Result<(), Box<dyn std::error::Error>> {
    let default_theme_name = DEFAULT_THEME_NAME.to_string();
    let theme_name = config.theme.unwrap_or(&default_theme_name);

    debug!("Loading theme '{}'", theme_name);

    let theme_config_path = resolve_theme(theme_name, config.develop)?;
    let theme_root_meta = ThemeRoot::read_from(&theme_config_path)?;

    let theme_config_dir = theme_config_path
        .parent()
        .ok_or("Failed to get theme root directory.")?;
    nix::unistd::chdir(theme_config_dir)?; // Change into the theme's directory

    let mut theme_qml_path = PathBuf::new();
    theme_qml_path.push(theme_config_dir);
    theme_qml_path.push(DEFAULT_QML_NAME);

    let theme_qml = theme_root_meta.qml.unwrap_or_default();
    if theme_qml.main.is_some() {
        theme_qml_path = theme_qml.main.unwrap();
    }

    let qml_path_cstring = CString::new(theme_qml_path.as_os_str().as_bytes())?;
    let qml_path_raw = qml_path_cstring.into_raw();

    let mut app_state = ApplicationState::new(qml_path_raw);
    let app_state_ptr = ApplicationStatePtr::new(&mut app_state as *mut ApplicationState);

    debug!("Initializing Wayland interfaces...");

    let mut state = WaylandState::new(&mut app_state as *mut ApplicationState, config.develop);
    let mut auth_state =
        AuthenticatorState::new(app_state_ptr, &AUTH_STOP_FLAG)
            .ok_or::<Box<dyn std::error::Error>>("Failed to create authenticator state".into())?;

    let mut event_queue = state.initialize()?;

    // TODO: Store pipe fd in ApplicationState
    auth_state.initialize(state.renderer_write_pipe.as_ref().unwrap().write_fd())?;

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

    AUTH_STOP_FLAG.store(true, Ordering::Relaxed);

    let _ = auth_thread.join();

    Ok(())
}
