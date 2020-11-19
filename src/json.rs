use sys;

#[derive(Copy, Clone)]
pub struct Json {
    json: *mut sys::playdate_json,
}

impl Json {
    pub fn new(json: *mut sys::playdate_json) -> Self {
        Self { json }
    }
}
