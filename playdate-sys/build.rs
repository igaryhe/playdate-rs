extern crate bindgen;

use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=wrapper.h");
    let arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    let bindings = if arch == "arm" || arch == "aarch64" {
        build_arm()
    } else {
        build_x86()
    };
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

fn build_arm() -> bindgen::Bindings {
    let config =
        fs::read_to_string(format!("{}/.Playdate/config", env::var("HOME").unwrap())).unwrap();
    let pd_sdk_path = config.trim_start_matches("SDKRoot\t").trim_end();
    bindgen::Builder::default()
        .use_core()
        .ctypes_prefix("cty")
        .clang_arg("-DTARGET_EXTENSION=1")
        .clang_arg("-DTARGET_PLAYDATE=1")
        .clang_arg(format!("-I{}/C_API", pd_sdk_path))
        .header("wrapper.h")
        .default_enum_style(bindgen::EnumVariation::Rust {
            non_exhaustive: false,
        })
        .derive_default(true)
        .derive_eq(true)
        .bitfield_enum("FileOptions")
        .bitfield_enum("SoundFormat")
        .bitfield_enum("PDButtons")
        .bitfield_enum("PDPeripherals")
        .clang_arg("-I/usr/local/playdate/gcc-arm-none-eabi-9-2019-q4-major/arm-none-eabi/include/")
        .clang_arg("-fshort-enums")
        .generate()
        .expect("Unable to generate bindings")
}

fn build_x86() -> bindgen::Bindings {
    let config =
        fs::read_to_string(format!("{}/.Playdate/config", env::var("HOME").unwrap())).unwrap();
    let pd_sdk_path = config.trim_start_matches("SDKRoot\t").trim_end();
    bindgen::Builder::default()
        .use_core()
        .ctypes_prefix("cty")
        .clang_arg("-DTARGET_EXTENSION=1")
        .clang_arg("-DTARGET_SIMULATOR=1")
        .clang_arg(format!("-I{}/C_API", pd_sdk_path))
        .header("wrapper.h")
        .default_enum_style(bindgen::EnumVariation::Rust {
            non_exhaustive: false,
        })
        .derive_default(true)
        .derive_eq(true)
        .bitfield_enum("FileOptions")
        .bitfield_enum("SoundFormat")
        .bitfield_enum("PDButtons")
        .bitfield_enum("PDPeripherals")
        .generate()
        .expect("Unable to generate bindings")
}
