#![no_std]
pub extern crate playdate_sys as sys;
use sys::cty;
use core::ptr;
pub mod system;
pub mod file;
pub mod graphics;
pub mod sprite;
pub mod display;
pub mod sound;
pub mod json;

pub struct Playdate {
    // playdate: *mut sys::PlaydateAPI,
    system: Option<system::System>,
    display: Option<display::Display>,
    graphics: Option<graphics::Graphics>,
}

static mut PLAYDATE: Playdate = Playdate {
    // playdate: ptr::null_mut(),
    system: None,
    display: None,
    graphics: None,
};

static mut FONT: *mut sys::LCDFont = ptr::null_mut();

impl Playdate {
    pub fn new(playdate: *mut sys::PlaydateAPI) {
        unsafe {
            PLAYDATE = Playdate {
                // playdate,
                system: Some(system::System::new((*playdate).system)),
                display: Some(display::Display::new((*playdate).display)),
                graphics: Some(graphics::Graphics::new((*playdate).graphics)),
            }
        }
    }
}

extern "C" fn update(_ud: *mut cty::c_void) -> cty::c_int {
    unsafe {
        PLAYDATE.graphics.unwrap().clear(graphics::LCDColor::SolidColor(sys::LCDSolidColor::kColorWhite));
        PLAYDATE.graphics.unwrap().draw_text(FONT, ptr::null_mut(), ptr::null_mut(), "Hello World", sys::PDStringEncoding::kASCIIEncoding, 150, 100, sys::LCDBitmapDrawMode::kDrawModeCopy, 0, sys::LCDRect { left: 0, right: 0, top: 0, bottom: 0}).unwrap();
    }
    1
}

#[no_mangle]
extern "C" fn eventHandler(playdate: *mut sys::PlaydateAPI, event: sys::PDSystemEvent, _arg: u32) -> cty::c_int {
    if event == sys::PDSystemEvent::kEventInit {
        Playdate::new(playdate);
        unsafe {
            PLAYDATE.display.unwrap().set_refresh_rate(20.0).unwrap();
            PLAYDATE.system.unwrap().set_update_callback(Some(update), ptr::null_mut()).unwrap();
            FONT = PLAYDATE.graphics.unwrap().load_font("/System/Fonts/Asheville-Sans-14-Bold.pft").unwrap();
        }
    }
    0
}
