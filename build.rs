use std::env;
use std::path::PathBuf;

fn main() {
	let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

	let dst = cmake::Config::new("cpp").out_dir(&out_dir).build();

	println!("cargo:rustc-link-search=native={}/lib", dst.display());
	println!("cargo:rustc-link-lib=static=tlockr_qt");
	println!("cargo:rerun-if-changed=cpp/"); 
}
