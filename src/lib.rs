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

const INIT_X: i32 = (400 - TEXT_WIDTH) / 2;
const INIT_Y: i32 = (240 - TEXT_HEIGHT) / 2;

const TEXT_WIDTH: i32 = 86;
const TEXT_HEIGHT: i32 = 16;

struct State {
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
}

impl State {
    pub fn init(&mut self) {
        self.x = INIT_X;
        self.y = INIT_Y;
        self.dx = 1;
        self.dy = 2;
    }

    pub fn update(&mut self) {
        unsafe {
            PLAYDATE.graphics.unwrap().clear(graphics::LCDColor::SolidColor(sys::LCDSolidColor::kColorWhite));
            PLAYDATE.graphics.unwrap().draw_text(FONT, ptr::null_mut(), ptr::null_mut(), "Hello World",
                                                 sys::PDStringEncoding::kASCIIEncoding, self.x, self.y,
                                                 sys::LCDBitmapDrawMode::kDrawModeCopy, 0,
                                                 sys::LCDRect { left: 0, right: 0, top: 0, bottom: 0}).unwrap();
        }
        self.x += self.dx;
        self.y += self.dy;
        if self.x < 0 || self.x > sys::LCD_COLUMNS as i32 - TEXT_WIDTH {
            self.dx = -self.dx;
        }
        if self.y < 0 || self.y > sys::LCD_ROWS as i32 - TEXT_HEIGHT {
            self.dy = -self.dy;
        }
    }
}

static mut STATE: State = State { x: INIT_X, y: INIT_Y, dx: 1, dy: 2 };

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
        STATE.update();
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
