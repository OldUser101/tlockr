// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2025, Nathan Gill

use crate::config::{Merge, RootConfig};

use std::path::PathBuf;
use dirs::config_dir;

pub const SYSTEM_CONFIG_DIR: &str = "/etc";
pub const CONFIG_SUB_DIR: &str = "tlockr";
pub const ROOT_CONFIG_NAME: &str = "tlockr.toml";

/// Load the tlockr root configuration from user and system config files
pub fn load_root_config() -> RootConfig {
    let mut root_config = RootConfig::default();

    // Merge system config
    let mut system_config_path = PathBuf::new();
    system_config_path.push(SYSTEM_CONFIG_DIR);
    system_config_path.push(CONFIG_SUB_DIR);
    system_config_path.push(ROOT_CONFIG_NAME);

    // TODO: Errors here should be propagated, if they relate to TOML formatting
    let system_config = RootConfig::read_from(system_config_path);
    if let Ok(cfg) = system_config {
        root_config = root_config.merge(cfg);
    }

    // Merge user config
    let opt_user_config_path = config_dir();
    if opt_user_config_path.is_none() {
        return root_config;
    }

    let mut user_config_path = opt_user_config_path.unwrap();
    user_config_path.push(CONFIG_SUB_DIR);
    user_config_path.push(ROOT_CONFIG_NAME);

    let user_config = RootConfig::read_from(user_config_path);
    if let Ok(cfg) = user_config {
        root_config = root_config.merge(cfg);
    }
    
    root_config
}
