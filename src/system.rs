use core::ptr;
use cstr_core::CString;
use sys;

pub use sys::PDPeripherals as Peripherals;
pub use sys::PDButtons as Buttons;
pub use sys::PDCallbackFunction as CallbackFunction;
pub use sys::PDLanguage as Language;

#[derive(Copy, Clone)]
pub struct System {
    system: *mut sys::playdate_sys,
}

impl System {
    pub fn new(system: *mut sys::playdate_sys) -> Self {
        System { system }
    }

    pub fn set_update_callback(&self, update: CallbackFunction) {
        unsafe {
            (*self.system).setUpdateCallback.unwrap()(update, ptr::null_mut());
        }
    }

    pub fn set_options_callback(&self, update: CallbackFunction) {
        unsafe {
            (*self.system).setOptionsCallback.unwrap()(update, ptr::null_mut());
        }
    }

    pub fn realloc(&self, ptr: *mut sys::cty::c_void,
                   size: sys::cty::c_ulong) -> *mut sys::cty::c_void {
        unsafe {
            (*self.system).realloc.unwrap()(ptr, size)
        }
    }

    pub fn log_to_console(&self, text: &str) {
        unsafe {
            if let Ok(c_text) = CString::new(text) {
                (*self.system).logToConsole
                    .unwrap()(c_text.as_ptr() as *mut sys::cty::c_char);
            }
        }
    }

    pub fn error(&self, text: &str) {
        unsafe {
            if let Ok(c_text) = CString::new(text) {
                (*self.system).error
                    .unwrap()(c_text.as_ptr() as *mut sys::cty::c_char);
            }
        }
    }

    pub fn draw_fps(&self, x: i32, y: i32) {
        unsafe {
            (*self.system).drawFPS.unwrap()(x, y);
        }
    }

    pub fn set_peripherals_enabled(&self, mask: Peripherals) {
        unsafe {
            (*self.system).setPeripheralsEnabled.unwrap()(mask);
        }
    }

    pub fn get_accelerometer(&self) -> (f32, f32, f32) {
        unsafe {
            let outx: *mut f32 = ptr::null_mut();
            let outy: *mut f32 = ptr::null_mut();
            let outz: *mut f32 = ptr::null_mut();
            (*self.system).getAccelerometer.unwrap()(outx, outy, outz);
            (*outx, *outy, *outz)
        }
    }

    pub fn get_crank_angle(&self) -> f32 {
        unsafe {
            (*self.system).getCrankAngle.unwrap()()
        }
    }

    pub fn get_crank_change(&self) -> f32 {
        unsafe {
            (*self.system).getCrankChange.unwrap()()
        }
    }

    pub fn is_crank_docked(&self) -> bool {
        unsafe {
            match (*self.system).isCrankDocked.unwrap()() {
                1 => true,
                _ => false,
            }
        }
    }

    pub fn get_button_state(&self) -> (Buttons, Buttons, Buttons) {
        let mut current: Buttons = Buttons(0);
        let mut pushed: Buttons = Buttons(0);
        let mut released: Buttons = Buttons(0);
        unsafe {
            (*self.system).getButtonState
                .unwrap()(&mut current, &mut pushed, &mut released);
            (current, pushed, released)
        }
    }

    pub fn get_language(&self) -> Language {
        unsafe {
            (*self.system).getLanguage.unwrap()()
        }
    }

    pub fn get_current_time_ms(&self) -> u32 {
        unsafe {
            (*self.system).getCurrentTimeMilliseconds
                .unwrap()()
        }
    }

    pub fn get_seconds_since_epoch(&self) -> (u32, u32) {
        let mut ms = 0;
        unsafe {
            let s = (*self.system).getSecondsSinceEpoch
                .unwrap()(&mut ms);
            (s, ms)
        }
    }

    pub fn get_flipped(&self) -> bool {
        unsafe {
            match (*self.system).getFlipped.unwrap()() {
                1 => true,
                _ => false,
            }
        }
    }
}
