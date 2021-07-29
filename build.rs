extern crate cmake;

fn main() {
    if option_env!("CARGO_CFG_TARGET_OS") == Some("android") {
        return;
    }

    let dst = cmake::build("soil2");
    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=soil2");
}
