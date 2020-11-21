use sys::playdate_display;

#[derive(Copy, Clone)]
pub struct Display {
    display: *mut playdate_display,
}

impl Display {
    pub fn new(display: *mut playdate_display) -> Self {
        Display { display }
    }

    pub fn set_refresh_rate(&self, rate: f32) {
        unsafe {
            (*self.display).setRefreshRate.unwrap()(rate);
        }
    }
}
