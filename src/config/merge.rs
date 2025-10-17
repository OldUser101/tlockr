// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2025, Nathan Gill

/*
    merge.rs:
        `Merge` trait for configuration structs.
*/

pub trait Merge {
    fn merge(self, other: Self) -> Self;
}

impl<T: Merge> Merge for Option<T> {
    fn merge(self, other: Self) -> Self {
        match (self, other) {
            (Some(a), Some(b)) => Some(a.merge(b)),
            (None, Some(b)) => Some(b),
            (Some(a), None) => Some(a),
            (None, None) => None,
        }
    }
}
