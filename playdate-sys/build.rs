extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=wrapper.h");
    let arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    let pd_sdk_path = env::var("PLAYDATE_SDK").expect("envvar PLAYDATE_SDK hasn't been set");
    let builder = bindgen::Builder::default()
        .use_core()
        .ctypes_prefix("cty")
        .clang_arg("-DTARGET_EXTENSION=1")
        .clang_arg(format!("-I{}/C_API", pd_sdk_path))
        .clang_arg("-fshort-enums")
        .header("wrapper.h")
        .default_enum_style(bindgen::EnumVariation::Rust {
            non_exhaustive: false,
        })
        .derive_default(true)
        .derive_eq(true)
        .bitfield_enum("FileOptions")
        .bitfield_enum("SoundFormat")
        .bitfield_enum("PDButtons")
        .bitfield_enum("PDPeripherals");
    let bindings = if arch == "arm" {
        let arm_gcc = env::var("ARM_GCC").expect("envvar ARM_GCC hasn't been set");
        builder
            .clang_arg("-DTARGET_PLAYDATE=1")
            .clang_arg(format!("-I{}/include", arm_gcc))
            .generate()
            .expect("unable to generate bindings")
    } else {
        builder
            .clang_arg("-DTARGET_SIMULATOR=1")
            .generate()
            .expect("unable to generate bindings")
    };
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
