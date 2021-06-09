use std::env;
use std::fs;
use cc;

fn main() {
    let arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    let pd_sdk_path = env::var("PLAYDATE_SDK").unwrap();
    if arch == "arm" || arch == "aarch64" {
        // fs::copy(format!("{}/C_API/buildsupport/link_map.ld", pd_sdk_path),
        //          format!("{}/link_map.ld", env::var("OUT_DIR").unwrap())).unwrap();
        cc::Build::new()
            .file(format!("{}/C_API/buildsupport/setup.c", pd_sdk_path))
            .include(format!("{}/C_API", pd_sdk_path))
            .flag("-DTARGET_PLAYDATE=1")
            .flag("-DTARGET_EXTENSION=1")
            .out_dir("target/thumbv7em-none-eabihf/release/examples")
            .compile("setup");
        // println!("cargo:rustc-link-search=native=/usr/local/playdate/gcc-arm-none-eabi-9-2019-q4-major/arm-none-eabi/lib");
        // println!("cargo:rustc-link-search=native=/usr/local/playdate/gcc-arm-none-eabi-9-2019-q4-major/lib/gcc/arm-none-eabi/9.2.1");
        println!("cargo:rerun-if-changed=build.rs");
    }
}