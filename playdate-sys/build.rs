extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=wrapper.h");
    let bindings = build();
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
fn build() -> bindgen::Bindings {
    let pd_sdk_path = env::var("PLAYDATE_SDK_PATH").unwrap();
     bindgen::Builder::default()
        .use_core()
        .ctypes_prefix("cty")
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .clang_arg(format!("-I{}/C_API", pd_sdk_path))
        .default_enum_style(bindgen::EnumVariation::Rust{ non_exhaustive: false })
        .derive_default(true)
        .derive_eq(true)
        .bitfield_enum("FileOptions")
        .bitfield_enum("SoundFormat")
        .bitfield_enum("PDButtons")
        .bitfield_enum("PDPeripherals")
        .generate()
        .expect("Unable to generate bindings")
}

#[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
fn build() {
    let pd_sdk_path = env::var("PLAYDATE_SDK_PATH").unwrap();
    bindgen::Builder::default()
        .use_core()
        .ctypes_prefix("cty")
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .clang_arg(format!("-I{}/C_API", pd_sdk_path))
        .clang_arg("-I/usr/local/playdate/gcc-arm-none-eabi-9-2019-q4-major/arm-none-eabi/include/")
        .clang_arg("-fshort-enums")
        .bitfield_enum("FileOptions")
        .bitfield_enum("SoundFormat")
        .bitfield_enum("PDButtons")
        .bitfield_enum("PDPeripherals")
        .generate()
        .expect("Unable to generate bindings")
}
