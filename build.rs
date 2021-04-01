extern crate cmake;

fn main() {
    let dst = cmake::build("soil2");
    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=soil2");
}