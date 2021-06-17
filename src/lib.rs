#![no_std]
#![feature(alloc_error_handler, core_intrinsics)]

pub extern crate playdate_sys as sys;
use anyhow::Result;
use sys::PlaydateAPI;
pub mod display;
pub mod file;
pub mod graphics;
pub mod json;
pub mod sound;
pub mod sprite;
pub mod system;

extern crate alloc;
use core::alloc::{GlobalAlloc, Layout};
use core::ptr;
use sys::cty;

#[derive(Copy, Clone)]
pub struct Playdate {
    system: Option<system::System>,
    filesystem: Option<file::Filesystem>,
    graphics: Option<graphics::Graphics>,
    sprite: Option<sprite::PDSprite>,
    display: Option<display::Display>,
    sound: Option<sound::Sound>,
}

impl Playdate {
    pub fn playdate() -> Self {
        unsafe { PLAYDATE.clone() }
    }
    pub fn system(&self) -> system::System {
        self.system.unwrap().clone()
    }
    pub fn filesystem(&self) -> file::Filesystem {
        self.filesystem.unwrap().clone()
    }
    pub fn graphics(&self) -> graphics::Graphics {
        self.graphics.unwrap().clone()
    }
    pub fn sprite(&self) -> sprite::PDSprite {
        self.sprite.unwrap().clone()
    }
    pub fn display(&self) -> display::Display {
        self.display.unwrap().clone()
    }
    pub fn sound(&self) -> sound::Sound {
        self.sound.unwrap().clone()
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

    pub fn get_filesystem() -> file::Filesystem {
        unsafe { PLAYDATE.filesystem.unwrap().clone() }
    }

    pub fn get_sound() -> sound::Sound {
        unsafe { PLAYDATE.sound.unwrap().clone() }
    }

    pub fn get_sprite() -> sprite::PDSprite {
        unsafe { PLAYDATE.sprite.unwrap().clone() }
    }
}

static mut PLAYDATE: Playdate = Playdate {
    system: None,
    filesystem: None,
    graphics: None,
    sprite: None,
    display: None,
    sound: None,
};

impl Playdate {
    pub fn new(playdate: *mut PlaydateAPI) {
        unsafe {
            PLAYDATE = Playdate {
                system: Some(system::System::new((*playdate).system)),
                filesystem: Some(file::Filesystem::new((*playdate).file)),
                graphics: Some(graphics::Graphics::new((*playdate).graphics)),
                sprite: Some(sprite::PDSprite::new((*playdate).sprite)),
                display: Some(display::Display::new((*playdate).display)),
                sound: Some(sound::Sound::new((*playdate).sound)),
            }
        }
    }
}

pub trait Game {
    fn init(playdate: &mut Playdate) -> Self;
    fn update(&mut self, playdate: &mut Playdate) -> Result<()>;
}

#[macro_export]
macro_rules! start_game {
    ($state:tt) => {
        extern crate alloc;
        use alloc::boxed::Box;
        use sys::{cty, PDSystemEvent, PlaydateAPI};
        static mut STATE: Option<$state> = None;

        extern "C" fn update(_ud: *mut cty::c_void) -> cty::c_int {
            unsafe {
                STATE = STATE.map(|mut s| {
                    s.update(&mut Playdate::playdate()).unwrap();
                    s
                });
            }
            1
        }

        #[no_mangle]
        unsafe extern "C" fn eventHandler(
            playdate: *mut PlaydateAPI,
            event: PDSystemEvent,
            _arg: u32,
        ) -> cty::c_int {
            if event == PDSystemEvent::kEventInit {
                Playdate::new(playdate);
                Playdate::get_display().set_refresh_rate(20.0);
                Playdate::get_system().set_update_callback(Some(update));
                unsafe {
                    STATE = Some($state::init(&mut Playdate::playdate()));
                }
            }
            0
        }
    };
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
    unsafe { *p = 0 };
    core::intrinsics::abort()
}

use core::panic::PanicInfo;

#[panic_handler]
fn panic(panic_info: &PanicInfo) -> ! {
    use core::fmt::Write;
    use heapless::String;
    if let Some(location) = panic_info.location() {
        let mut output: String<1024> = String::new();
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
    #[cfg(target_arch = "x86_64")]
    {
        unsafe {
            core::intrinsics::breakpoint();
        }
    }
    abort_with_addr(0xdeadbeef)
}

#[cfg(target_arch = "x86_64")]
#[no_mangle]
pub unsafe extern "C" fn memcpy(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    let mut i = 0;
    while i < n {
        *dest.offset(i as isize) = *src.offset(i as isize);
        i += 1;
    }
    dest
}

#[cfg(target_arch = "x86_64")]
pub unsafe fn memset_internal(s: *mut u8, c: sys::cty::c_int, n: usize) -> *mut u8 {
    let mut i = 0;
    while i < n {
        *s.offset(i as isize) = c as u8;
        i += 1;
    }
    s
}

#[cfg(target_arch = "x86_64")]
#[no_mangle]
pub unsafe extern "C" fn memset(s: *mut u8, c: sys::cty::c_int, n: usize) -> *mut u8 {
    memset_internal(s, c, n)
}

#[cfg(target_os = "macos")]
#[no_mangle]
pub unsafe extern "C" fn memmove(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    if src < dest as *const u8 {
        // copy from end
        let mut i = n;
        while i != 0 {
            i -= 1;
            *dest.offset(i as isize) = *src.offset(i as isize);
        }
    } else {
        // copy from beginning
        let mut i = 0;
        while i < n {
            *dest.offset(i as isize) = *src.offset(i as isize);
            i += 1;
        }
    }
    dest
}

#[cfg(target_os = "macos")]
#[no_mangle]
pub unsafe extern "C" fn memcmp(s1: *const u8, s2: *const u8, n: usize) -> i32 {
    let mut i = 0;
    while i < n {
        let a = *s1.offset(i as isize);
        let b = *s2.offset(i as isize);
        if a != b {
            return a as i32 - b as i32;
        }
        i += 1;
    }
    0
}

#[cfg(target_os = "macos")]
#[no_mangle]
pub unsafe extern "C" fn bcmp(s1: *const u8, s2: *const u8, n: usize) -> i32 {
    memcmp(s1, s2, n)
}

#[cfg(target_os = "macos")]
#[no_mangle]
pub unsafe extern "C" fn __bzero(s: *mut u8, n: usize) {
    memset_internal(s, 0, n);
}

// windows specific config
#[cfg(target_os = "windows")]
#[used]
#[no_mangle]
static _fltused: i32 = 0;

#[cfg(target_os = "windows")]
#[no_mangle]
extern "system" fn _DllMainCRTStartup(_: *const u8, _: u32, _: *const u8) -> u32 {
    1
}

// arm specific config
#[cfg(target_arch = "arm")]
extern "C" {
    fn eventHandler(playdate: *mut PlaydateAPI, event: sys::PDSystemEvent, _arg: u32)
        -> cty::c_int;
    fn __bss_start__();
    fn __bss_end__();
}
#[cfg(target_arch = "arm")]
#[no_mangle]
#[link_section = ".capi_handler"]
pub static mut PD_eventHandler: unsafe extern "C" fn(
    *mut PlaydateAPI,
    sys::PDSystemEvent,
    u32,
) -> i32 = eventHandler;
#[cfg(target_arch = "arm")]
#[no_mangle]
#[link_section = ".bss_start"]
pub static mut _bss_start: unsafe extern "C" fn() = __bss_start__;
#[cfg(target_arch = "arm")]
#[no_mangle]
#[link_section = ".bss_end"]
pub static mut _bss_end: unsafe extern "C" fn() = __bss_end__;

#[macro_export]
macro_rules! pd_call {
    ($caller:expr, $func:ident) => { unsafe { (*$caller).$func.unwrap()() } };
    ($caller:expr, $func:ident, $($param:expr),*) => { unsafe { (*$caller).$func.unwrap()($($param,)*) } };
    ($caller:expr, $func:ident, $($param:expr),* => $val:expr) => {
        {
            let result = unsafe {(*$caller).$func.unwrap()($($param,)*)};
            ensure!(result >= 0, "Error {} from $func", result);
            Ok($val)
        }
    };
}