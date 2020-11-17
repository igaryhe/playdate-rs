use sys;
// use core::ptr;
use anyhow::Result;

#[derive(Copy, Clone)]
pub struct System {
    system: *mut sys::playdate_sys,
}

// static mut SYSTEM: System = System { system: ptr::null_mut() };

impl System {
    pub fn new(system: *mut sys::playdate_sys) -> Self {
        System {
            system,
        }
    }

    pub fn set_update_callback(&self, update: sys::PDCallbackFunction, userdata: *mut sys::cty::c_void) -> Result<()> {
        unsafe {
            (*self.system).setUpdateCallback.unwrap()(update, userdata);
        }
        Ok(())
    }
}