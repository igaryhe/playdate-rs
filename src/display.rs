use crate::pd_call;
use sys::playdate_display;

#[derive(Copy, Clone)]
pub struct Display(*const playdate_display);

impl Display {
    pub fn new(display: *const playdate_display) -> Self {
        Display(display)
    }
    pub fn width(&self) -> i32 {
        pd_call!(self.0, getWidth)
    }
    pub fn height(&self) -> i32 {
        pd_call!(self.0, getHeight)
    }
    pub fn set_refresh_rate(&self, rate: f32) {
        pd_call!(self.0, setRefreshRate, rate)
    }
    pub fn set_inverted(&self, flag: i32) {
        pd_call!(self.0, setInverted, flag)
    }
    pub fn set_scale(&self, s: u32) {
        pd_call!(self.0, setScale, s)
    }
    pub fn set_mosaic(&self, x: u32, y: u32) {
        pd_call!(self.0, setMosaic, x, y)
    }
    pub fn set_flipped(&self, x: i32, y: i32) {
        pd_call!(self.0, setFlipped, x, y)
    }
    pub fn set_offset(&self, x: i32, y: i32) {
        pd_call!(self.0, setOffset, x, y)
    }
}