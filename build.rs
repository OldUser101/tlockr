use std::env;
use std::path::PathBuf;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    let dst = cmake::Config::new(".").out_dir(&out_dir).build();

    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-lib=static=tlockr_qt");

    println!("cargo:rustc-link-lib=Qt6Quick");
    println!("cargo:rustc-link-lib=Qt6QmlMeta");
    println!("cargo:rustc-link-lib=Qt6QmlWorkerScript");
    println!("cargo:rustc-link-lib=Qt6QmlModels");
    println!("cargo:rustc-link-lib=Qt6Widgets");
    println!("cargo:rustc-link-lib=Qt6OpenGL");
    println!("cargo:rustc-link-lib=Qt6Gui");
    println!("cargo:rustc-link-lib=Qt6Qml");
    println!("cargo:rustc-link-lib=Qt6Network");
    println!("cargo:rustc-link-lib=Qt6Core");

    println!("cargo:rustc-link-lib=stdc++");
    println!("cargo:rustc-link-lib=GL");
    println!("cargo:rustc-link-lib=GLESv2");

    println!("cargo:rerun-if-changed=cpp/");
}
