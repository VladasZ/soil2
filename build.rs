extern crate cmake;
extern crate pkg_config;

use std::env;
use cmake::Config;

fn main() {
    let dst = cmake::build("soil2");
    println!("cargo:rustc-link-search=native={}", dst.display());


    let mut config = Config::new("soil2");

    let dst = config
        .profile("Release")
        .build();

    let out_dir = env::var("OUT_DIR").unwrap();
    println!("cargo:rustc-link-search=native={}/lib", dst.display());
   // println!("cargo:rustc-link-lib=static=freetype");
    println!("cargo:outdir={}", out_dir);
}