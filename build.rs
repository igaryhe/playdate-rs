use std::env;
use std::fs;
use cc;

fn main() {
    let arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    let config = fs::read_to_string(format!("{}/.Playdate/config", env::var("HOME").unwrap())).unwrap();
    let pd_sdk_path = config.trim_start_matches("SDKRoot\t").trim_end();
    if arch == "arm" || arch == "aarch64" {
        cc::Build::new()
            .file(format!("{}/C_API/buildsupport/setup.c", pd_sdk_path))
            .include(format!("{}/C_API", pd_sdk_path))
            .flag("-DTARGET_PLAYDATE=1")
            .flag("-DTARGET_EXTENSION=1")
            .out_dir("target/thumbv7em-none-eabihf/release")
            .compile("setup");
    }
}
