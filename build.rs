extern crate pkg_config;

fn main() {
    println!("cargo:rustc-link-lib=c_api");
    println!("cargo:rustc-link-search=src/ffi_test");
}