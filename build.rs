extern crate cmake;
use std::env;

fn main() {
    if env!("CARGO_CFG_TARGET_OS") == "android" {
        return;
    }

    let dst = cmake::build("soil2");
    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=soil2");
}
