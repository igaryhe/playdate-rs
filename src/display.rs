use sys::playdate_display;

#[derive(Copy, Clone)]
pub struct Display {
    display: *const playdate_display,
}

impl Display {
    pub fn new(display: *const playdate_display) -> Self {
        Display { display }
    }

    pub fn get_width(&self) -> i32 {
        unsafe {
            (*self.display).getWidth.unwrap()()
        }
    }

    pub fn get_height(&self) -> i32 {
        unsafe {
            (*self.display).getHeight.unwrap()()
        }
    }

    pub fn set_inverted(&self, flag: i32) {
        unsafe {
            (*self.display).setInverted.unwrap()(flag)
        }
    }

    pub fn set_scale(&self, s: u32) {
        unsafe {
            (*self.display).setScale.unwrap()(s)
        }
    }

    pub fn set_mosaic(&self, x: u32, y: u32) {
        unsafe {
            (*self.display).setMosaic.unwrap()(x, y)
        }
    }

    pub fn set_refresh_rate(&self, rate: f32) {
        unsafe {
            (*self.display).setRefreshRate.unwrap()(rate);
        }
    }

    pub fn set_offset(&self, x: i32, y: i32) {
        unsafe {
            (*self.display).setOffset.unwrap()(x, y)
        }
    }

    pub fn set_flipped(&self, x: i32, y: i32) {
        unsafe {
            (*self.display).setFlipped.unwrap()(x, y)
        }
    }
}
