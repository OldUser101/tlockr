// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2025, Nathan Gill

/*
    theme.rs:
        Structures for serializing/deserializing theme metadata files
*/

use dirs::config_dir;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::config::{CONFIG_SUB_DIR, SYSTEM_CONFIG_DIR};

#[derive(Serialize, Deserialize)]
pub struct ThemeMeta {
    pub name: String,
    pub version: Option<String>,
    pub author: Option<Vec<String>>,
    pub license: Option<String>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct ThemeQml {
    pub main: Option<PathBuf>,
}

#[derive(Serialize, Deserialize)]
pub struct ThemeRoot {
    pub theme: ThemeMeta,
    pub qml: Option<ThemeQml>,
}

pub const THEME_SUB_DIR: &str = "themes";
pub const THEME_CONFIG_NAME: &str = "theme.toml";

/// Resolve a theme name to a theme configuration path
pub fn resolve_theme(theme: &String, develop: bool) -> Result<PathBuf, Box<dyn std::error::Error>> {
    if develop {
        // In develop mode, themes can be loaded from any directory
        let mut path = PathBuf::new();
        path.push(theme);
        path.push(THEME_CONFIG_NAME);

        if path.is_file() {
            return Ok(path);
        }
    }

    if let Some(mut user_config_path) = config_dir() {
        user_config_path.push(CONFIG_SUB_DIR);
        user_config_path.push(THEME_SUB_DIR);
        user_config_path.push(theme);
        user_config_path.push(THEME_CONFIG_NAME);

        if user_config_path.is_file() {
            return Ok(user_config_path);
        }
    }

    let mut system_config_path = PathBuf::new();
    system_config_path.push(SYSTEM_CONFIG_DIR);
    system_config_path.push(CONFIG_SUB_DIR);
    system_config_path.push(THEME_SUB_DIR);
    system_config_path.push(theme);
    system_config_path.push(THEME_CONFIG_NAME);

    if system_config_path.is_file() {
        return Ok(system_config_path);
    }

    Err(format!("Failed to locate theme '{theme}'.").into())
}

impl ThemeRoot {
    /// Given a theme path, load the theme root metadata
    pub fn read_from(theme_path: &PathBuf) -> Result<ThemeRoot, Box<dyn std::error::Error>> {
        let s = std::fs::read_to_string(theme_path)?;
        let c: ThemeRoot = toml::from_str(&s)?;
        Ok(c)
    }
}
