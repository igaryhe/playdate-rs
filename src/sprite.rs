use sys;

#[derive(Copy, Clone)]
pub struct Sprite {
    sprite: *mut sys::playdate_sprite,
}

impl Sprite {
    pub fn new(sprite: *mut sys::playdate_sprite) -> Self {
        Self { sprite }
    }
}
