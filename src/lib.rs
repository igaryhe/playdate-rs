#![no_std]
#![feature(alloc_error_handler, core_intrinsics, lang_items)]

pub extern crate playdate_sys as sys;
use core::alloc::{GlobalAlloc, Layout};
use core::ptr;
use sys::cty;
pub mod display;
pub mod file;
pub mod graphics;
pub mod json;
pub mod sound;
pub mod sprite;
pub mod system;

pub struct Playdate {
    system: Option<system::System>,
    display: Option<display::Display>,
    graphics: Option<graphics::Graphics>,
}

impl Playdate {
    pub fn system() -> system::System {
        unsafe { PLAYDATE.system.unwrap().clone() }
    }

    pub fn display() -> display::Display {
        unsafe { PLAYDATE.display.unwrap().clone() }
    }

    pub fn graphics() -> graphics::Graphics {
        unsafe { PLAYDATE.graphics.unwrap().clone() }
    }
}

static mut PLAYDATE: Playdate = Playdate {
    system: None,
    display: None,
    graphics: None,
};

const INIT_X: i32 = (400 - TEXT_WIDTH) / 2;
const INIT_Y: i32 = (240 - TEXT_HEIGHT) / 2;

const TEXT_WIDTH: i32 = 86;
const TEXT_HEIGHT: i32 = 16;

struct State {
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
    font: *mut sys::LCDFont,
}

impl State {
    pub fn update(&mut self) {
        Playdate::graphics().clear(graphics::LCDColor::SolidColor(
            graphics::LCDSolidColor::kColorWhite,
        ));
        Playdate::graphics()
            .draw_text(
                self.font,
                ptr::null_mut(),
                ptr::null_mut(),
                "Hello World",
                sys::PDStringEncoding::kASCIIEncoding,
                self.x,
                self.y,
                sys::LCDBitmapDrawMode::kDrawModeCopy,
                0,
                sys::LCDRect {
                    left: 0,
                    right: 0,
                    top: 0,
                    bottom: 0,
                },
            )
            .unwrap();
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

static mut STATE: State = State {
    x: INIT_X,
    y: INIT_Y,
    dx: 1,
    dy: 2,
    font: ptr::null_mut(),
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

extern "C" fn update(_ud: *mut cty::c_void) -> cty::c_int {
    unsafe {
        STATE.update();
    }
    1
}

#[no_mangle]
extern "C" fn eventHandler(
    playdate: *mut sys::PlaydateAPI,
    event: sys::PDSystemEvent,
    _arg: u32,
) -> cty::c_int {
    if event == sys::PDSystemEvent::kEventInit {
        Playdate::new(playdate);
        Playdate::display().set_refresh_rate(20.0).unwrap();
        Playdate::system()
            .set_update_callback(Some(update), ptr::null_mut())
            .unwrap();
        unsafe {
            STATE.font = Playdate::graphics()
                .load_font("/System/Fonts/Asheville-Sans-14-Bold.pft")
                .unwrap();
        }
    }
    0
}

pub struct PlaydateAllocator;

unsafe impl Sync for PlaydateAllocator {}

unsafe impl GlobalAlloc for PlaydateAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        Playdate::system().realloc(ptr::null_mut(), layout.size() as sys::cty::c_ulong) as *mut u8
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        Playdate::system().realloc(ptr as *mut sys::cty::c_void, 0);
    }

    unsafe fn realloc(&self, ptr: *mut u8, _layout: Layout, new_size: usize) -> *mut u8 {
        Playdate::system().realloc(ptr as *mut sys::cty::c_void, new_size as sys::cty::c_ulong)
            as *mut u8
    }
}

#[global_allocator]
static mut ALLOCATOR: PlaydateAllocator = PlaydateAllocator;

#[alloc_error_handler]
fn alloc_error(_layout: Layout) -> ! {
    Playdate::system().log_to_console("OOM\0");
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
fn panic(#[allow(unused)] panic_info: &PanicInfo) -> ! {
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
        Playdate::system().log_to_console(output.as_str());
    } else {
        Playdate::system().log_to_console("panic\0");
    }
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    {
        unsafe {
            core::intrinsics::breakpoint();
        }
    }
    abort_with_addr(0xdeadbeef);
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[no_mangle]
pub unsafe extern "C" fn memcpy(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    let mut i = 0;
    while i < n {
        *dest.offset(i as isize) = *src.offset(i as isize);
        i += 1;
    }
    dest
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
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
