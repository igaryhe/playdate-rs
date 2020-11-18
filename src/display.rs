use sys;
// use core::ptr;
use anyhow::Result;

#[derive(Copy, Clone)]
pub struct Display {
    display: *mut sys::playdate_display,
}

impl Display {
    pub fn new(display: *mut sys::playdate_display) -> Self {
        Display { display }
    }

    pub fn set_refresh_rate(&self, rate: f32) -> Result<()> {
        unsafe {
            (*self.display).setRefreshRate.unwrap()(rate);
        }
        Ok(())
    }
}
