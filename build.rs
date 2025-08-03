// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2025, Nathan Gill

/*
    build.rs:
        Build script, used for building and linking with CMake.
*/

use std::env;
use std::path::PathBuf;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    let dst = cmake::Config::new(".").out_dir(&out_dir).build();

    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-lib=static=tlockr_qt");

    println!("cargo:rustc-link-lib=Qt6Quick");
    println!("cargo:rustc-link-lib=Qt6OpenGL");
    println!("cargo:rustc-link-lib=Qt6Gui");
    println!("cargo:rustc-link-lib=Qt6Qml");
    println!("cargo:rustc-link-lib=Qt6Core");

    println!("cargo:rustc-link-lib=stdc++");
    println!("cargo:rustc-link-lib=GLESv2");

    println!("cargo:rustc-link-lib=xkbcommon");

    println!("cargo:rerun-if-changed=cpp/");
}
