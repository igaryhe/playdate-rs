use anyhow::{Result, anyhow, ensure};
use core::{ptr, ops::RangeInclusive, slice};
use cstr_core::{CString, CStr};
use sys;

use crate::Playdate;

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
    graphics: *const sys::playdate_graphics,
}

impl Graphics {
    pub fn new(graphics: *const sys::playdate_graphics) -> Self {
        Graphics { graphics }
    }

    pub fn clear(&self, color: Color) {
        unsafe {
            (*self.graphics).clear.unwrap()(color.into());
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

    pub fn get_debug_bitmap(&self) -> Result<Bitmap> {
        unsafe {
            let ptr = (*self.graphics).getDebugBitmap.unwrap()();
            ensure!(!ptr.is_null(), {"Null pointer from get_debug_bitmap"});
            Ok(Bitmap { bitmap: ptr })
        }
    }

    pub fn get_frame_buffer_bitmap(&self) -> Result<Bitmap> {
        unsafe {
            let ptr = (*self.graphics).getFrameBufferBitmap.unwrap()();
            ensure!(!ptr.is_null(), {"Null pointer from get_frame_buffer_bitmap"});
            Ok(Bitmap { bitmap: ptr })
        }
    }

    pub fn set_background_color(&self, color: SolidColor) {
        unsafe {
            (*self.graphics).setBackgroundColor.unwrap()(color)
        }
    }

    pub fn display(&self) {
        unsafe {
            (*self.graphics).display.unwrap()()
        }
    }

    pub fn set_draw_offset(&self, dx: i32, dy: i32) {
        unsafe {
            (*self.graphics).setDrawOffset.unwrap()(dx, dy)
        }
    }

    pub fn push_context(&self, target: Bitmap) {
        unsafe {
            (*self.graphics).pushContext.unwrap()(target.bitmap)
        }
    }

     pub fn pop_context(&self) {
        unsafe {
            (*self.graphics).popContext.unwrap()()
        }
    }

    pub fn set_stencil(&self, stencil: Bitmap) {
        unsafe {
            (*self.graphics).setStencil.unwrap()(stencil.bitmap)
        }
    }

    pub fn set_draw_mode(&self, mode: BitmapDrawMode) {
        unsafe {
            (*self.graphics).setDrawMode.unwrap()(mode)
        }
    }

    pub fn set_clip_rect(&self, x: i32, y: i32, width: i32, height: i32) {
        unsafe {
            (*self.graphics).setClipRect.unwrap()(x, y, width, height)
        }
    }

    pub fn clear_clip_rect(&self) {
        unsafe {
            (*self.graphics).clearClipRect.unwrap()()
        }
    }

    pub fn set_line_cap_style(&self, end_cap_style: LineCapStyle) {
        unsafe {
            (*self.graphics).setLineCapSyle.unwrap()(end_cap_style)
        }
    }

    pub fn set_font(&self, font: Font) {
        unsafe {
            (*self.graphics).setFont.unwrap()(font.font)
        }
    }

    pub fn set_text_tracking(&self, tracking: i32) {
        unsafe {
            (*self.graphics).setTextTracking.unwrap()(tracking)
        }
    }

    pub fn draw_line(&self, x1: i32, y1: i32, x2: i32, y2: i32,
                     width: i32, color: Color) {
        unsafe {
            (*self.graphics).drawLine.unwrap()(x1, y1, x2, y2, width, color.into())
        }
    }

    pub fn fill_triangle(&self, x1: i32, y1: i32, x2: i32,
                         y2: i32, x3: i32, y3: i32, color: Color) {
        unsafe {
            (*self.graphics).fillTriangle.unwrap()(x1, y1, x2, y2, x3, y3, color.into())
        }
    }

    pub fn draw_rect(&self, x: i32, y: i32, width: i32, height: i32, color: Color) {
        unsafe {
            (*self.graphics).drawRect.unwrap()(x, y, width, height, color.into())
        }
    }

    pub fn fill_rect(&self, x: i32, y: i32, width: i32, height: i32, color: Color) {
        unsafe {
            (*self.graphics).fillRect.unwrap()(x, y, width, height, color.into())
        }
    }

    pub fn draw_ellipse(&self, x: i32, y: i32, width: i32,height: i32,
                     line_width: i32, start_angle: f32, end_angle: f32,
                     color: Color) {
        unsafe {
            (*self.graphics).drawEllipse.unwrap()(x, y, width, height,line_width,
                                         start_angle, end_angle, color.into())
        }
    }

    pub fn fill_ellipse(&self, x: i32, y: i32, width: i32,height: i32,
                     start_angle: f32, end_angle: f32, color: Color) {
        unsafe {
            (*self.graphics).fillEllipse.unwrap()(x, y, width, height,start_angle, end_angle, color.into())
        }
    }

    pub fn draw_text(&self,text: &str, encoding: sys::PDStringEncoding,
                     x: i32, y: i32) -> i32 {
        let len = text.len() as sys::cty::c_ulong;
        let c_str = CString::new(text).unwrap();
        unsafe {
            (*self.graphics).drawText.unwrap()(
                c_str.as_ptr() as *const sys::cty::c_void,
                len.into(), encoding, x, y)
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
    font: *mut sys::LCDFont,
}

impl Font {
    pub fn load(path: &str) -> Result<Self> {
        let c_path = CString::new(path).unwrap();
        unsafe {
            let graphics = Playdate::get_graphics().graphics;
            let out_err = ptr::null_mut();
            let font = (*graphics).loadFont.unwrap()(c_path.as_ptr(), out_err);
            if out_err.is_null() {
                Ok(Font {font})
            } else {
                Err(anyhow!(CStr::from_ptr(*out_err).to_str().unwrap()))
            }
        }
    }
}

pub struct Bitmap {
    pub bitmap: *mut sys::LCDBitmap,
}

impl Bitmap {
    pub fn new(width: u32, height: u32, bgcolor: Color) -> Result<Self> {
        unsafe {
            let graphics = Playdate::get_graphics().graphics;
            let ptr = (*graphics).newBitmap
                .unwrap()(width as i32, height as i32, bgcolor.into());
            ensure!(!ptr.is_null(), {"failed to create bitmap"});
            Ok(Self { bitmap: ptr })
        }
    }

    pub fn load(path: &str) -> Result<Self> {
        unsafe {
            let graphics = Playdate::get_graphics().graphics;
            let c_path = CString::new(path).unwrap();
            let outerr = ptr::null_mut();
            let ptr = (*graphics).loadBitmap.unwrap()(c_path.as_ptr(), outerr);
            ensure!(!ptr.is_null(), {"failed to load bitmap"});
            Ok(Self { bitmap: ptr })
        }
    }

    pub fn copy(bitmap: Bitmap) -> Result<Self> {
        unsafe {
            let graphics = Playdate::get_graphics().graphics;
            let ptr = (*graphics).copyBitmap.unwrap()(bitmap.bitmap);
            ensure!(!ptr.is_null(), {"failed to copy bitmap"});
            Ok(Self { bitmap: ptr })
        }
    }

    pub fn load_into(&mut self, path: &str) -> Result<()> {
         unsafe {
             let graphics = Playdate::get_graphics().graphics;
             let c_path = CString::new(path).unwrap();
             let outerr = ptr::null_mut();
             (*graphics).loadIntoBitmap
                 .unwrap()(c_path.as_ptr(), self.bitmap, outerr);
             ensure!(!outerr.is_null(), {"failed to load bitmap"});
             Ok(())
        }
    }

    pub fn clear(&mut self, bgcolor: Color) {
        unsafe {
            let graphics = Playdate::get_graphics().graphics;
            (*graphics).clearBitmap.unwrap()(self.bitmap, bgcolor.into())
        }
    }

    pub fn draw(&self, x: i32, y: i32, flip: BitmapFlip) {
        unsafe {
            let graphics = Playdate::get_graphics().graphics;
            (*graphics).drawBitmap.unwrap()(self.bitmap, x, y, flip)
        }
    }

    pub fn tile(&self, x: i32, y: i32, width: i32, height: i32, flip: BitmapFlip) {
        unsafe {
            let graphics = Playdate::get_graphics().graphics;
            (*graphics).tileBitmap
                .unwrap()(self.bitmap, x, y, width, height, flip)
        }
    }

    pub fn draw_scaled(&self, x: i32, y: i32, xscale: f32, yscale: f32) {
        unsafe {
            let graphics = Playdate::get_graphics().graphics;
            (*graphics).drawScaledBitmap
                .unwrap()(self.bitmap, x, y, xscale, yscale)
        }
    }
}

impl Drop for Bitmap {
    fn drop(&mut self) {
        unsafe {
            let graphics = Playdate::get_graphics().graphics;
            (*graphics).freeBitmap.unwrap()(self.bitmap);
        }
    }
}

pub struct BitmapTable {
    table: *mut sys::LCDBitmapTable,
}

impl BitmapTable {
    pub fn new(count: u32, width: u32, height: u32) -> Result<Self> {
        unsafe {
            let graphics = Playdate::get_graphics().graphics;
            let ptr = (*graphics).newBitmapTable
                .unwrap()(count as i32, width as i32, height as i32);
            ensure!(!ptr.is_null(), {"failed to create bitmap table"});
            Ok(Self { table: ptr })
        }
    }

    pub fn load(path: &str) -> Result<Self> {
        unsafe {
            let graphics = Playdate::get_graphics().graphics;
            let c_path = CString::new(path).unwrap();
            let outerr = ptr::null_mut();
            let ptr = (*graphics).loadBitmapTable
                .unwrap()(c_path.as_ptr(), outerr);
            ensure!(!outerr.is_null(), {"failed to load bitmap table"});
            Ok(Self { table: ptr })
        }
    }

    pub fn load_into(&mut self, path: &str) -> Result<()> {
        unsafe {
            let graphics = Playdate::get_graphics().graphics;
            let c_path = CString::new(path).unwrap();
            let outerr = ptr::null_mut();
            (*graphics).loadIntoBitmapTable
                .unwrap()(c_path.as_ptr(), self.table, outerr);
            ensure!(!outerr.is_null(), {"failed to load bitmap table"});
            Ok(())
        }
    }

    pub fn get_bitmap(&self, idx: usize) -> Result<Bitmap> {
         unsafe {
             let graphics = Playdate::get_graphics().graphics;
             let ptr = (*graphics).getTableBitmap
                 .unwrap()(self.table, idx as i32);
             ensure!(!ptr.is_null(), {"failed to get bitmap"});
             Ok(Bitmap { bitmap: ptr })
         }
    }
}

impl Drop for BitmapTable {
    fn drop(&mut self) {
        unsafe {
            let graphics = Playdate::get_graphics().graphics;
            (*graphics).freeBitmapTable.unwrap()(self.table);
        }
    }
}
