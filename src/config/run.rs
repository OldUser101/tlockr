// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2025, Nathan Gill

/// Configuration struct for running the screen locker
pub struct RunConfig<'a> {
    pub theme: Option<&'a String>,
    pub develop: bool,
}
