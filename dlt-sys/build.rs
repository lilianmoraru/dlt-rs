extern crate gcc;
extern crate cmake;

use std::env;
use std::path::{ Path, PathBuf };

fn main() {
    // Just some hacks currently
    let build_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
//    println!("cargo:rustc-link-lib=static=dlt"); // issues with fPIC
    println!("cargo:rustc-link-lib=dlt");
    println!("cargo:rustc-link-search=native={}/../../../native", build_dir.display());
}
