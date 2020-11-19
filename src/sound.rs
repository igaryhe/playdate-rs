use sys;

#[derive(Copy, Clone)]
pub struct Sound {
    sound: *mut sys::playdate_sound,
}

impl Sound {
    pub fn new(sound: *mut sys::playdate_sound) -> Self {
        Self { sound }
    }
}
