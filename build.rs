extern crate cmake;
extern crate pkg_config;

fn main() {
    let dst = cmake::build("soil2");
    println!("cargo:rustc-link-search=native={}", dst.display());
}