#![no_std]
#![feature(alloc_error_handler, core_intrinsics, link_args)]

pub extern crate playdate_sys as sys;
pub mod display;
pub mod file;
pub mod graphics;
pub mod json;
pub mod sound;
pub mod sprite;
pub mod system;
pub mod alloc;

#[derive(Copy, Clone)]
pub struct Playdate {
    system: Option<system::System>,
    display: Option<display::Display>,
    graphics: Option<graphics::Graphics>,
}

impl Playdate {
    pub fn playdate() -> Self {
        unsafe { PLAYDATE.clone() }
    }
    pub fn system(&self) -> system::System {
        self.system.unwrap().clone()
    }

    pub fn display(&self) -> display::Display {
        self.display.unwrap().clone()
    }

    pub fn graphics(&self) -> graphics::Graphics {
        self.graphics.unwrap().clone()
    }

    pub fn get_system() -> system::System {
        unsafe { PLAYDATE.system.unwrap().clone() }
    }

    pub fn get_graphics() -> graphics::Graphics {
        unsafe { PLAYDATE.graphics.unwrap().clone() }
    }

    pub fn get_display() -> display::Display {
        unsafe { PLAYDATE.display.unwrap().clone() }
    }
}

static mut PLAYDATE: Playdate = Playdate {
    system: None,
    display: None,
    graphics: None,
};

impl Playdate {
    pub fn new(playdate: *mut sys::PlaydateAPI) {
        unsafe {
            PLAYDATE = Playdate {
                system: Some(system::System::new((*playdate).system)),
                display: Some(display::Display::new((*playdate).display)),
                graphics: Some(graphics::Graphics::new((*playdate).graphics)),
            }
        }
    }
}

pub trait Game {
    fn init(&mut self, playdate: &mut Playdate);
    fn update(&mut self, playdate: &mut Playdate);
}

#[macro_export]
macro_rules! start_game {
    ($state:tt, $default:expr) => (
        static mut STATE: $state = $default;
        extern "C" fn update(_ud: *mut sys::cty::c_void) -> sys::cty::c_int {
            unsafe{
                STATE.update(&mut Playdate::playdate());
            }
            1
        }

        #[no_mangle]
        extern "C" fn eventHandler(playdate: *mut sys::PlaydateAPI,
            event: sys::PDSystemEvent, _arg: u32,) -> sys::cty::c_int {
            if event == sys::PDSystemEvent::kEventInit {
                Playdate::new(playdate);
                Playdate::get_display().set_refresh_rate(20.0).unwrap();
                Playdate::get_system()
                    .set_update_callback(Some(update), ptr::null_mut())
                    .unwrap();
                unsafe {
                    STATE.init(&mut Playdate::playdate());
                }
            }
            0
        }
    )
}
