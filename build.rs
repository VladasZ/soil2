extern crate cmake;
extern crate pkg_config;

use std::env;
use cmake::Config;

fn main() {
    let dst = cmake::build("soil2");

    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=soil2");
}