#![no_std]
pub extern crate playdate_sys as sys;
pub use cty;
pub mod system;
pub mod file;
pub mod graphics;
pub mod sprite;
pub mod display;
pub mod sound;
pub mod json;

#[no_mangle]
extern "C" fn eventHandler(playdate: *mut PlaydateAPI, event: PDSystemEvent, arg: u32) -> cty::c_int {
    if event == PDSystemEvent::kEventInit {        
    }
    0
}
