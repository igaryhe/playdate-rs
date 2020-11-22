use anyhow::{Result, anyhow, ensure};
use core::{ptr, ops::RangeInclusive, slice};
use cstr_core::{CString, CStr};
use sys;

pub use sys::LCD_COLUMNS as COLUMNS;
pub use sys::LCD_ROWS as ROWS;
pub use sys::LCD_ROWSIZE as ROWSIZE;

pub use sys::LCDBitmapDrawMode as BitmapDrawMode;
pub use sys::LCDBitmapFlip as BitmapFlip;
pub use sys::LCDSolidColor as SolidColor;
pub use sys::LCDLineCapStyle as LineCapStyle;
pub use sys::LCDFontLanguage as FontLanguage;
pub use sys::PDStringEncoding as StringEncoding;
pub use sys::LCDRect as Rect;
pub use sys::LCDSprite as Sprite;

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
            let out_err = ptr::null_mut();
            let font = (*self.graphics).loadFont.unwrap()(c_path.as_ptr(), out_err);
            if out_err.is_null() {
                Ok(Font::new(font))
            } else {
                Err(anyhow!(CStr::from_ptr(*out_err).to_str().unwrap()))
            }
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
        target: Option<Bitmap>,
        stencil: Option<Bitmap>,
        text: &str,
        encoding: sys::PDStringEncoding,
        x: i32,
        y: i32,
        mode: sys::LCDBitmapDrawMode,
        tracking: i32,
        clip: sys::LCDRect,
    ) -> i32 {
        let len = text.len() as sys::cty::c_ulong;
        let c_str = CString::new(text).unwrap();
        unsafe {
            (*self.graphics).drawText.unwrap()(
                font.font,
                target.map_or(ptr::null_mut(), |b| b.bitmap),
                stencil.map_or(ptr::null_mut(), |b| b.bitmap),
                c_str.as_ptr() as *const sys::cty::c_void,
                len,
                encoding,
                x,
                y,
                mode,
                tracking,
                clip,
            )
        }
    }

    pub fn mark_updated_rows(&self, range: RangeInclusive<i32>) {
        let (start, end) = range.into_inner();
        unsafe {
            (*self.graphics).markUpdatedRows.unwrap()(start, end);
        }
    }

    pub fn get_frame(&self) -> Result<&'static mut [u8]> {
        unsafe {
            let ptr = (*self.graphics).getFrame.unwrap()();
            ensure!(!ptr.is_null(), {"Null pointer from get_frame"});
            let frame = slice::from_raw_parts_mut(ptr, (ROWSIZE * ROWS) as usize);
            Ok(frame)
        }
    }

    pub fn get_display_frame(&self) -> Result<&'static mut [u8]> {
        unsafe {
            let ptr = (*self.graphics).getDisplayFrame.unwrap()();
            ensure!(!ptr.is_null(), {"Null pointer from get_frame"});
            let frame = slice::from_raw_parts_mut(ptr, (ROWSIZE * ROWS) as usize);
            Ok(frame)
        }
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
    pub font: *mut sys::LCDFont,
}

impl Font {
    pub fn new(font: *mut sys::LCDFont) -> Self {
        Self { font }
    }

    // pub fn get_font_glyph(&self, c: u16) -> Result<(LCDBitmap, u32)> {
        
    // }
}

impl Default for Font {
    fn default() -> Self {
        Self { font: ptr::null_mut() }
    }
}

pub struct Bitmap {
    bitmap: *mut sys::LCDBitmap,
}

impl Bitmap {
    pub fn new(bitmap: *mut sys::LCDBitmap) -> Self {
        Bitmap { bitmap }
    }
}
