use sys;
use sys::cty;
use crate::Playdate;
use anyhow::{Result, Error, ensure};
use alloc::format;
use cstr_core::{CString, CStr};

pub use sys::FileOptions;
pub use sys::FileStat;
use crate::pd_call;

macro_rules! fs_call {
    ($func:ident, $($param:expr),*) => {
        {
            let fs = Playdate::get_filesystem();
            let result = unsafe {(*fs.0).$func.unwrap()($($param,)*)};
            ensure!(result >= 0, "Error {} from $func", result);
            Ok(result)
        }
    };
}

#[derive(Copy, Clone)]
pub struct Filesystem(*const sys::playdate_file);

impl Filesystem {
    pub fn new(fs: *const sys::playdate_file) -> Self { Self(fs) }

    pub fn open(&self, path: &str, mode: FileOptions) -> Result<File> {
        let c_path = CString::new(path).map_err(Error::msg)?;
        let file = pd_call!(self.0, open, c_path.as_ptr(), mode);
        ensure!(!file.is_null(), "Error open {}", path);
        Ok(File(file))
    }

    pub fn stat(&self, path: &str) -> Result<FileStat> {
        let mut file_stat = FileStat::default();
        pd_call!(self.0, stat, str_to_ptr(path), &mut file_stat => file_stat)
    }

    pub fn mkdir(&self, path: &str) -> Result<()> {
        pd_call!(self.0, mkdir, str_to_ptr(path) => ())
    }

    pub fn unlink(&self, name: &str, recursive: bool) -> Result<()> {
        pd_call!(self.0, unlink, str_to_ptr(name), recursive as i32 => ())
    }
    
    pub fn rename(&self, from: &str, to: &str) -> Result<()> {
        pd_call!(self.0, rename, str_to_ptr(from), str_to_ptr(to) => ())
    }

    pub fn get_err(&self) -> Result<&str> {
        unsafe {
            let ptr = (*self.0).geterr.unwrap()();
            ensure!(!ptr.is_null(), "no previous error");
            Ok(CStr::from_ptr(ptr).to_str().map_err(Error::msg)?)
        }
    }
}

pub struct File(*mut sys::SDFile);

impl File {
    pub fn read(&mut self, buf: &mut [u8]) -> Result<i32> {
        fs_call!(read, self.0, buf.as_mut_ptr() as *mut cty::c_void, buf.len() as u32)
    }

    pub fn write(&mut self, buf: &[u8]) -> Result<i32> {
        fs_call!(write, self.0, buf.as_ptr() as *const cty::c_void, buf.len() as u32)
    }

    pub fn flush(&mut self) -> Result<i32> {
        fs_call!(flush, self.0)
    }

    pub fn tell(&mut self) -> Result<i32> {
        fs_call!(tell, self.0)
    }

    pub fn seek(&mut self, pos: i32, whence: Whence) -> Result<()> {
        pd_call!(Playdate::get_filesystem().0, seek, self.0, pos, whence as i32 => ())
    }
}

impl Drop for File {
    fn drop(&mut self) { pd_call!(Playdate::get_filesystem().0, close, self.0); }
}

#[repr(i32)]
#[derive(Debug, Clone, Copy)]
pub enum Whence {
    Set = sys::SEEK_SET as i32,
    Cur = sys::SEEK_CUR as i32,
    End = sys::SEEK_END as i32,
}

fn str_to_ptr(string: &str) -> *const cty::c_char {
    CString::new(string).map_err(Error::msg).unwrap().as_ptr()
}