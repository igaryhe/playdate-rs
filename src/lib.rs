#![no_std]
#![feature(alloc_error_handler, core_intrinsics, link_args)]

pub extern crate playdate_sys as sys;
use sys::PlaydateAPI;
pub mod display;
pub mod file;
pub mod graphics;
pub mod json;
pub mod sound;
pub mod sprite;
pub mod system;
// pub mod alloc;

extern crate alloc;
use core::alloc::{GlobalAlloc, Layout};
use core::ptr;
use sys::cty;

#[derive(Copy, Clone)]
pub struct Playdate {
    system: Option<system::System>,
    filesystem: Option<file::Filesystem>,
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
    pub fn filesystem(&self) -> file::Filesystem { self.filesystem.unwrap().clone() }
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

    pub fn get_filesystem() -> file::Filesystem {
        unsafe { PLAYDATE.filesystem.unwrap().clone() }
    }
}

static mut PLAYDATE: Playdate = Playdate {
    system: None,
    filesystem: None,
    graphics: None,
    sprite: None,
    display: None,
    sound: None,
    json: None,
};

impl Playdate {
    pub fn new(playdate: *mut PlaydateAPI) {
        unsafe {
            PLAYDATE = Playdate {
                system: Some(system::System::new((*playdate).system)),
                filesystem: Some(file::Filesystem::new((*playdate).file)),
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
    ($state:tt) => (
        use sys::{cty, PDSystemEvent, PlaydateAPI};
        static mut STATE: $state = $state::default();
        
        extern "C" fn update(_ud: *mut cty::c_void) -> cty::c_int {
            unsafe{
                STATE.update(&mut Playdate::playdate());
            }
            1
        }

        #[no_mangle]
        extern "C" fn eventHandler(playdate: *mut PlaydateAPI,
            event: PDSystemEvent, _arg: u32,) -> cty::c_int {
            if event == PDSystemEvent::kEventInit {
                Playdate::new(playdate);
                Playdate::get_display().set_refresh_rate(20.0);
                Playdate::get_system().set_update_callback(Some(update));
                unsafe {
                    STATE.init(&mut Playdate::playdate());
                }
            }
            0
        }
    )
}

pub struct PlaydateAllocator;

unsafe impl Sync for PlaydateAllocator {}

unsafe impl GlobalAlloc for PlaydateAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        Playdate::get_system().realloc(ptr::null_mut(), layout.size() as cty::c_ulong) as *mut u8
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        Playdate::get_system().realloc(ptr as *mut cty::c_void, 0);
    }

    unsafe fn realloc(&self, ptr: *mut u8, _layout: Layout, new_size: usize) -> *mut u8 {
        Playdate::get_system().realloc(ptr as *mut cty::c_void, new_size as cty::c_ulong) as *mut u8
    }
}

#[global_allocator]
static mut A: PlaydateAllocator = PlaydateAllocator;

#[alloc_error_handler]
fn alloc_error(_layout: Layout) -> ! {
    Playdate::get_system().log_to_console("OOM\0");
    abort_with_addr(0xDEADFA11)
}

fn abort_with_addr(addr: usize) -> ! {
    let p = addr as *mut i32;
    unsafe {
        *p = 0;
    }
    core::intrinsics::abort()
}

use core::panic::PanicInfo;

#[panic_handler]
fn panic(panic_info: &PanicInfo) -> ! {
    use core::fmt::Write;
    use heapless::{consts::*, String};
    if let Some(location) = panic_info.location() {
        let mut output: String<U1024> = String::new();
        let payload = if let Some(payload) = panic_info.payload().downcast_ref::<&str>() {
            payload
        } else {
            "no payload"
        };
        write!(
            output,
            "panic: {} @ {}:{}\0",
            payload,
            location.file(),
            location.line()
        )
        .expect("write");
        Playdate::get_system().error(output.as_str());
    } else {
        Playdate::get_system().error("panic\0");
    }
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    {
        unsafe {
            core::intrinsics::breakpoint();
        }
    }
    abort_with_addr(0xdeadbeef)
}
