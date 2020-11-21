extern crate alloc;

use core::alloc::{GlobalAlloc, Layout};
use core::ptr;
use sys::cty;
use crate::*;

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
    loop {}
    // abort_with_addr(0xdeadbeef)
}
