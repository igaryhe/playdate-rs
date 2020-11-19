use anyhow::Result;
use core::ptr;
use cstr_core::CString;
use sys;

pub use sys::LCD_COLUMNS as COLUMNS;
pub use sys::LCD_ROWS as ROWS;
pub use sys::LCDSolidColor as SolidColor;
pub use sys::LCDBitmapDrawMode as BitmapDrawMode;
pub use sys::PDStringEncoding;
pub use sys::LCDRect as Rect;

#[derive(Copy, Clone)]
pub struct Graphics {
    graphics: *mut sys::playdate_graphics,
}

impl Graphics {
    pub fn new(graphics: *mut sys::playdate_graphics) -> Self {
        Graphics { graphics }
    }

    pub fn load_font(&self, path: &str) -> Result<Font> {
        let c_path = CString::new(path).unwrap();
        unsafe {
            let font = (*self.graphics).loadFont.unwrap()(c_path.as_ptr(), ptr::null_mut());
            Ok(Font {font})
        }
    }

    pub fn clear(&self, color: Color) {
        unsafe {
            (*self.graphics).clear.unwrap()(color.into());
        }
    }

    pub fn draw_text(
        &self,
        font: &Font,
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
                font.font,
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

pub enum Color {
    SolidColor(SolidColor),
    Pattern(sys::LCDPattern),
}

impl From<Color> for usize {
    fn from(color: Color) -> Self {
        match color {
            Color::SolidColor(solid_color) => solid_color as usize,
            Color::Pattern(pattern) => {
                let pattern_ptr = &pattern as *const u8;
                pattern_ptr as usize
            }
        }
    }
}

#[derive(Copy, Clone)]
pub struct Font {
    font: *mut sys::LCDFont,
}

impl Font {
    pub fn new(font: *mut sys::LCDFont) -> Result<Self> {
        Ok(Self {font})
    }

    // pub fn get_font_glyph(&self, c: u16) -> Result<(LCDBitmap, u32)> {
        
    // }
}
