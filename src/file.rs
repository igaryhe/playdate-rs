use sys;

pub use sys::FileOptions;
pub use sys::FileStat;

#[derive(Copy, Clone)]
pub struct File {
    file: *mut sys::playdate_file,
}

impl File {
    pub fn new(file: *mut sys::playdate_file) -> Self {
        Self { file }
    }
}
