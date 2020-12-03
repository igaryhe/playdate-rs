use sys;
use anyhow::{Result, ensure};
use crate::Playdate;
use crate::graphics::{Bitmap, BitmapFlip, BitmapDrawMode, Rect};
use core::ptr;

pub use sys::PDRect as PDRect;
pub use sys::SpriteCollisionResponseType as CollisionResponseType;

#[derive(Copy, Clone)]
pub struct PDSprite {
    sprite: *const sys::playdate_sprite,
}

impl PDSprite {
    pub fn new(sprite: *const sys::playdate_sprite) -> Self {
        Self { sprite }
    }
}

pub struct Sprite {
    sprite: *mut sys::LCDSprite,
}

impl Sprite {
    pub fn new() -> Result<Self> {
        unsafe {
            let spr = Playdate::get_sprite().sprite;
            let ptr = (*spr).newSprite.unwrap()();
            ensure!(!ptr.is_null(), "fail to create new sprite");
            Ok(Self {sprite: ptr})
        }
    }

    pub fn copy(sprite: &Sprite) -> Result<Self> {
        unsafe {
            let spr = Playdate::get_sprite().sprite;
            let ptr = (*spr).copy.unwrap()(sprite.sprite);
            ensure!(!ptr.is_null(), "fail to copy new sprite");
            Ok(Self {sprite: ptr})
        }
    }

    pub fn set_bounds(&mut self, bounds: PDRect) {
        unsafe {
            let spr = Playdate::get_sprite().sprite;
            (*spr).setBounds.unwrap()(self.sprite, bounds)
        }
    }

    pub fn bounds(&self) -> PDRect {
        unsafe {
            let spr = Playdate::get_sprite().sprite;
            (*spr).getBounds.unwrap()(self.sprite)
        }
    }

    pub fn move_to(&mut self, x: f32, y: f32) {
        unsafe {
            let spr = Playdate::get_sprite().sprite;
            (*spr).moveTo.unwrap()(self.sprite, x, y)
        }
    }

    pub fn move_by(&mut self, dx: f32, dy: f32) {
        unsafe {
            let spr = Playdate::get_sprite().sprite;
            (*spr).moveBy.unwrap()(self.sprite, dx, dy)
        }
    }

    pub fn get_position(&self) -> (f32, f32) {
        unsafe {
            let x = ptr::null_mut();
            let y = ptr::null_mut();
            let spr = Playdate::get_sprite().sprite;
            (*spr).getPosition.unwrap()(self.sprite, x, y);
            (*x, *y)
        }
    }

    pub fn set_image(&mut self, image: Bitmap, flip: BitmapFlip) {
        unsafe {
            let spr = Playdate::get_sprite().sprite;
            (*spr).setImage.unwrap()(self.sprite, image.bitmap, flip);
        }
    }

    pub fn image(&mut self) -> Result<Bitmap> {
        unsafe {
            let spr = Playdate::get_sprite().sprite;
            let ptr = (*spr).getImage.unwrap()(self.sprite);
            ensure!(!ptr.is_null(), "failed to retrieve bitmap");
            Ok(Bitmap { bitmap: ptr })
        }
    }

    pub fn set_size(&mut self, width: f32, height: f32) {
        unsafe {
            let spr = Playdate::get_sprite().sprite;
            (*spr).setSize.unwrap()(self.sprite, width, height)
        }
    }

    pub fn set_z_index(&mut self, z_index: i16) {
        unsafe {
            let spr = Playdate::get_sprite().sprite;
            (*spr).setZIndex.unwrap()(self.sprite, z_index)
        }
    }

    pub fn set_tag(&mut self, tag: u8) {
        unsafe {
            let spr = Playdate::get_sprite().sprite;
            (*spr).setTag.unwrap()(self.sprite, tag)
        }
    }

    pub fn tag(&mut self) -> u8 {
        unsafe {
            let spr = Playdate::get_sprite().sprite;
            (*spr).getTag.unwrap()(self.sprite)
        }
    }

    pub fn set_draw_mode(&mut self, mode: BitmapDrawMode) {
        unsafe {
            let spr = Playdate::get_sprite().sprite;
            (*spr).setDrawMode.unwrap()(self.sprite, mode)
        }
    }

    pub fn set_image_flip(&mut self, flip: BitmapFlip) {
        unsafe {
            let spr = Playdate::get_sprite().sprite;
            (*spr).setImageFlip.unwrap()(self.sprite, flip)
        }
    }

    pub fn image_flip(&mut self) -> BitmapFlip {
        unsafe {
            let spr = Playdate::get_sprite().sprite;
            (*spr).getImageFlip.unwrap()(self.sprite)
        }
    }

     pub fn set_stencil(&mut self, stencil: Bitmap) {
        unsafe {
            let spr = Playdate::get_sprite().sprite;
            (*spr).setStencil.unwrap()(self.sprite, stencil.bitmap)
        }
     }

    pub fn set_clip_rect(&mut self, clip_rect: Rect) {
        unsafe {
            let spr = Playdate::get_sprite().sprite;
            (*spr).setClipRect.unwrap()(self.sprite, clip_rect)
        }
    }

    pub fn clear_clip_rect(&mut self) {
        unsafe {
            let spr = Playdate::get_sprite().sprite;
            (*spr).clearClipRect.unwrap()(self.sprite)
        }
    }

    pub fn set_updates_enabled(&mut self, flag: bool) {
        unsafe {
            let spr = Playdate::get_sprite().sprite;
            (*spr).setUpdatesEnabled.unwrap()(self.sprite, flag as i32)
        }
    }

    pub fn updates_enabled(&mut self) -> bool {
        unsafe {
            let spr = Playdate::get_sprite().sprite;
            (*spr).updatesEnabled.unwrap()(self.sprite) != 0
        }
    }

    pub fn set_visible(&mut self, flag: bool) {
        unsafe {
            let spr = Playdate::get_sprite().sprite;
            (*spr).setVisible.unwrap()(self.sprite, flag as i32)
        }
    }

    pub fn is_visible(&mut self) -> bool {
        unsafe {
            let spr = Playdate::get_sprite().sprite;
            (*spr).isVisible.unwrap()(self.sprite) != 0
        }
    }

    pub fn set_opaque(&mut self, flag: bool) {
        unsafe {
            let spr = Playdate::get_sprite().sprite;
            (*spr).setOpaque.unwrap()(self.sprite, flag as i32)
        }
    }

    pub fn mark_dirty(&mut self) {
        unsafe {
            let spr = Playdate::get_sprite().sprite;
            (*spr).markDirty.unwrap()(self.sprite)
        }
    }

    pub fn set_ignores_draw_offset(&mut self, flag: bool) {
        unsafe {
            let spr = Playdate::get_sprite().sprite;
            (*spr).setIgnoresDrawOffset.unwrap()(self.sprite, flag as i32)
        }
    }

    pub fn set_update_function(&mut self, func: sys::LCDSpriteUpdateFunction) {
        unsafe {
            let spr = Playdate::get_sprite().sprite;
            (*spr).setUpdateFunction.unwrap()(self.sprite, func)
        }
    }
}

impl Drop for Sprite {
    fn drop(&mut self) {
        unsafe {
            let spr = Playdate::get_sprite().sprite;
            (*spr).freeSprite.unwrap()(self.sprite)
        }
    }
}

pub struct CollisionInfo {
    sprite: Sprite,
    other: Sprite,
    response_type: CollisionResponseType,
    overlap: u8,
    ti: f32,
    movement: Vec2<f32>,
    normal: Vec2<i32>,
    touch: Vec2<f32>,
    sprite_rect: PDRect,
    other_rect: PDRect,
}

pub struct QueryInfo {
    sprite: Sprite,
    ti1: f32,
    ti2: f32,
    entry_point: Vec2<f32>,
    exit_point: Vec2<f32>,
}

pub struct Vec2<T> {
    x: T,
    y: T,
}
