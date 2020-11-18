use anyhow::Result;
use core::ptr;
use cstr_core::CString;
use sys;
#[derive(Copy, Clone)]
pub struct Graphics {
    graphics: *mut sys::playdate_graphics,
}

pub use sys::LCDSolidColor;

impl Graphics {
    pub fn new(graphics: *mut sys::playdate_graphics) -> Self {
        Graphics { graphics }
    }

    pub fn load_font(&self, path: &str) -> Result<*mut sys::LCDFont> {
        let c_path = CString::new(path).unwrap();
        unsafe {
            let font = (*self.graphics).loadFont.unwrap()(c_path.as_ptr(), ptr::null_mut());
            Ok(font)
        }
    }

    pub fn clear(&self, color: LCDColor) {
        unsafe {
            (*self.graphics).clear.unwrap()(color.into());
        }
    }

    pub fn draw_text(
        &self,
        font: *mut sys::LCDFont,
        target: *mut sys::LCDBitmap,
        stencil: *mut sys::LCDBitmap,
        text: &str,
        encoding: sys::PDStringEncoding,
        x: i32,
        y: i32,
        mode: sys::LCDBitmapDrawMode,
        tracking: i32,
        clip: sys::LCDRect,
    ) -> Result<i32> {
        let len = text.len() as sys::cty::c_ulong;
        let c_str = CString::new(text).unwrap();
        unsafe {
            (*self.graphics).drawText.unwrap()(
                font,
                target,
                stencil,
                c_str.as_ptr() as *const sys::cty::c_void,
                len,
                encoding,
                x,
                y,
                mode,
                tracking,
                clip,
            );
        }
        Ok(0)
    }
}

pub enum LCDColor {
    SolidColor(sys::LCDSolidColor),
    Pattern(sys::LCDPattern),
}

impl From<LCDColor> for usize {
    fn from(color: LCDColor) -> Self {
        match color {
            LCDColor::SolidColor(solid_color) => solid_color as usize,
            LCDColor::Pattern(pattern) => {
                let pattern_ptr = &pattern as *const u8;
                pattern_ptr as usize
            }
        }
    }
}
