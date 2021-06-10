use std::env;
use std::fs;
use cc;

fn main() {
    let arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    let pd_sdk_path = env::var("PLAYDATE_SDK").unwrap();
    let out_dir = "target/thumbv7em-none-eabihf/release/examples";
    if arch == "arm" || arch == "aarch64" {
        cc::Build::new()
            .file(format!("{}/C_API/buildsupport/setup.c", pd_sdk_path))
            .include(format!("{}/C_API", pd_sdk_path))
            .flag("-DTARGET_PLAYDATE=1")
            .flag("-DTARGET_EXTENSION=1")
            .flag("-Wall")
            .flag("-Wno-unknown-pragmas")
            .flag("-O2")
            .flag("-mthumb")
            .flag("-mcpu=cortex-m7")
            .flag("-mfloat-abi=hard")
            .flag("-mfpu=fpv4-sp-d16")
            .flag("-D__FPU_USED=1")
            .flag("-falign-functions=16")
            .flag("-fomit-frame-pointer")
            .flag("-gdwarf-2")
            .flag("-fverbose-asm")
            .flag("-ffunction-sections")
            .flag("-fdata-sections")
            .out_dir(out_dir.clone())
            .compile("setup");
        println!("cargo:rerun-if-changed=build.rs");
        println!("cargo:rustc-link-search=native={}", out_dir);
        println!("cargo:rustc-link-lib=static=setup");
    }
}