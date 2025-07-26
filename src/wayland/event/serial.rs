// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2025, Nathan Gill

/*
    serial.rs:
        Atomic counter for event serials
*/

use std::sync::atomic::{AtomicU32, Ordering};

/// Serial counter value
static SERIAL_COUNTER: AtomicU32 = AtomicU32::new(0);

/// Get the next value from the atomic counter
///
/// This function is intended to be used for generating event serials.
pub fn next_serial() -> u32 {
    unsafe { _next_serial() }
}

/// Internal serial generation function
///
/// **This function should not be used in Rust code, use the safe `next_serial`**
///
/// This function should only be directly called by external code that needs to
/// use the events system, e.g. the C renderer.
unsafe extern "C" fn _next_serial() -> u32 {
    SERIAL_COUNTER.fetch_add(1, Ordering::SeqCst)
}
