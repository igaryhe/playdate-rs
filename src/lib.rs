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
    file: Option<file::File>,
    graphics: Option<graphics::Graphics>,
    sprite: Option<sprite::Sprite>,
    display: Option<display::Display>,
    sound: Option<sound::Sound>,
    json: Option<json::Json>,
}

impl Playdate {
    pub fn playdate() -> Self {
        unsafe { PLAYDATE.clone() }
    }
    pub fn system(&self) -> system::System { self.system.unwrap().clone() }
    pub fn file(&self) -> file::File { self.file.unwrap().clone() }
    pub fn graphics(&self) -> graphics::Graphics { self.graphics.unwrap().clone() }
    pub fn sprite(&self) -> sprite::Sprite { self.sprite.unwrap().clone() }
    pub fn display(&self) -> display::Display { self.display.unwrap().clone() }
    pub fn sound(&self) -> sound::Sound { self.sound.unwrap().clone() }
    pub fn json(&self) -> json::Json { self.json.unwrap().clone() }

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
    file: None,
    graphics: None,
    sprite: None,
    display: None,
    sound: None,
    json: None,
};

impl Playdate {
    pub fn new(playdate: *mut sys::PlaydateAPI) {
        unsafe {
            PLAYDATE = Playdate {
                system: Some(system::System::new((*playdate).system)),
                file: Some(file::File::new((*playdate).file)),
                graphics: Some(graphics::Graphics::new((*playdate).graphics)),
                sprite: Some(sprite::Sprite::new((*playdate).sprite)),
                display: Some(display::Display::new((*playdate).display)),
                sound: Some(sound::Sound::new((*playdate).sound)),
                json: Some(json::Json::new((*playdate).json)),
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
