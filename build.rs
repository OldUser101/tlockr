use std::env;
use std::path::PathBuf;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    let dst = cmake::Config::new(".").out_dir(&out_dir).build();

    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-lib=dylib=tlockr_qt");

    println!("cargo:rustc-link-lib=stdc++");

    println!("cargo:rerun-if-changed=cpp/");
}
