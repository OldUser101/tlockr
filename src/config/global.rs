// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2025, Nathan Gill

/*
    global.rs:
        Configuration structs for tlockr configuration files.
*/

use std::path::PathBuf;

use crate::config::Merge;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct StoredRunConfig {
    pub theme: Option<String>,
}

impl Merge for StoredRunConfig {
    fn merge(self, other: Self) -> Self {
        Self {
            theme: other.theme.or(self.theme),
        }
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct GlobalConfig {
    pub editor: Option<String>,
}

impl Merge for GlobalConfig {
    fn merge(self, other: Self) -> Self {
        Self {
            editor: other.editor.or(self.editor),
        }
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct RootConfig {
    pub run: Option<StoredRunConfig>,
    pub global: Option<GlobalConfig>,
}

impl Merge for RootConfig {
    fn merge(self, other: Self) -> Self {
        Self {
            run: self.run.merge(other.run),
            global: self.global.merge(other.global),
        }
    }
}

impl RootConfig {
    /// Read a `RootConfig` object from a TOML formatted file
    pub fn read_from(path: PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        let s = std::fs::read_to_string(path)?;
        let c: RootConfig = toml::from_str(&s)?;
        Ok(c)
    }
}
