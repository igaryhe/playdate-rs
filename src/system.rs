use anyhow::Result;
use core::ptr;
use cstr_core::CString;
use sys;

#[derive(Copy, Clone)]
pub struct System {
    system: *mut sys::playdate_sys,
}

impl System {
    pub fn new(system: *mut sys::playdate_sys) -> Self {
        System { system }
    }

    pub fn set_update_callback(
        &self,
        update: sys::PDCallbackFunction,
        userdata: *mut sys::cty::c_void,
    ) -> Result<()> {
        unsafe {
            (*self.system).setUpdateCallback.unwrap()(update, userdata);
        }
        Ok(())
    }

    pub fn realloc(
        &self,
        ptr: *mut sys::cty::c_void,
        size: sys::cty::c_ulong,
    ) -> *mut sys::cty::c_void {
        unsafe { (*self.system).realloc.unwrap()(ptr, size) }
    }

    pub fn log_to_console(&self, text: &str) {
        unsafe {
            if self.system != ptr::null_mut() {
                if let Ok(c_text) = CString::new(text) {
                    (*self.system).logToConsole.unwrap()(c_text.as_ptr() as *mut sys::cty::c_char);
                }
            }
        }
    }

    pub fn draw_fps(&self, x: i32, y: i32) -> Result<()> {
        unsafe {
            (*self.system).drawFPS.unwrap()(x, y);
        }
        Ok(())
    }
}
