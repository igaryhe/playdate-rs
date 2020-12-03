use sys;
use sys::cty;
use crate::Playdate;
use anyhow::{Result, Error, ensure};
use alloc::format;
use cstr_core::{CString, CStr};

pub use sys::FileOptions;
pub use sys::FileStat;

#[derive(Copy, Clone)]
pub struct Filesystem {
    fs: *const sys::playdate_file,
}

impl Filesystem {
    pub fn new(fs: *const sys::playdate_file) -> Self {
        Self { fs }
    }

    pub fn open(&self, path: &str, mode: FileOptions) -> Result<File> {
        let c_path = CString::new(path).map_err(Error::msg)?;
        unsafe {
            let file = (*self.fs).open.unwrap()(c_path.as_ptr(), mode);
            ensure!(!file.is_null(), "Error open {}", path);
            Ok(File { file })
        }
    }

    pub fn stat(&self, path: &str) -> Result<FileStat> {
        let c_path = CString::new(path).map_err(Error::msg)?;
        let mut file_stat = FileStat::default();
        unsafe {
            let result = (*self.fs).stat.unwrap()(c_path.as_ptr(), &mut file_stat);
            ensure!(result == 0, "Error {} from stat", result);
            Ok(file_stat)
        }
    }

    pub fn mkdir(&self, path: &str) -> Result<()> {
        let c_path = CString::new(path).map_err(Error::msg)?;
        unsafe {
            let result = (*self.fs).mkdir.unwrap()(c_path.as_ptr());
            ensure!(result == 0, "Error {} from mkdir", result);
        }
        Ok(())
    }

    pub fn unlink(&self, name: &str, recursive: bool) -> Result<()> {
        let c_name = CString::new(name).map_err(Error::msg)?;
        unsafe {
            let result = (*self.fs).unlink.unwrap()(c_name.as_ptr(), recursive as i32);
            ensure!(result == 0, "Error {} from unlink", result);
        }
        Ok(())
    }
    
    pub fn rename(&self, from: &str, to: &str) -> Result<()> {
        let c_from = CString::new(from).map_err(Error::msg)?;
        let c_to = CString::new(to).map_err(Error::msg)?;
        unsafe {
            let result = (*self.fs).rename.unwrap()(c_from.as_ptr(), c_to.as_ptr());
            ensure!(result == 0, "Error {} from rename", result);
        }
        Ok(())
    }

    pub fn get_err(&self) -> Result<&str> {
        unsafe {
            let ptr = (*self.fs).geterr.unwrap()();
            ensure!(!ptr.is_null(), "no previous error");
            Ok(CStr::from_ptr(ptr).to_str().map_err(Error::msg)?)
        }
    }
}

pub struct File {
    file: *mut sys::SDFile,
}

impl File {
    pub fn read(&mut self, buf: &mut [u8]) -> Result<i32> {
        let fs = Playdate::get_filesystem();
        unsafe {
            let result = (*fs.fs).read
                .unwrap()(self.file,
                        buf.as_mut_ptr() as *mut cty::c_void,
                        buf.len() as u32);
            ensure!(result >= 0, "Error {} from read", result);
            Ok(result)
        }
    }

    pub fn write(&mut self, buf: &[u8]) -> Result<i32> {
        let fs = Playdate::get_filesystem();
        unsafe {
            let result = (*fs.fs).write
                .unwrap()(self.file,
                          buf.as_ptr() as *mut cty::c_void,
                          buf.len() as u32);
            ensure!(result >= 0, "Error {} from write", result);
            Ok(result)
        }
    }

    pub fn flush(&mut self) -> Result<i32> {
        let fs = Playdate::get_filesystem();
        unsafe {
            let result = (*fs.fs).flush
                .unwrap()(self.file);
            ensure!(result >= 0, "Error {} from flush", result);
            Ok(result)
        }
    }

    pub fn tell(&mut self) -> Result<i32> {
        let fs = Playdate::get_filesystem();
        unsafe {
            let result = (*fs.fs).tell
                .unwrap()(self.file);
            ensure!(result >= 0, "Error {} from tell", result);
            Ok(result)
        }
    }

    pub fn seek(&mut self, pos: i32, whence: Whence) -> Result<i32> {
        let fs = Playdate::get_filesystem();
        unsafe {
            let result = (*fs.fs).seek
                .unwrap()(self.file, pos, whence as i32);
            ensure!(result >= 0, "Error {} from seek", result);
            Ok(result)
        }
    }
}

impl Drop for File {
    fn drop(&mut self) {
        let fs = Playdate::get_filesystem();
        unsafe {
            (*fs.fs).close.unwrap()(self.file);   
        }
    }
}

#[repr(i32)]
#[derive(Debug, Clone, Copy)]
pub enum Whence {
    Set = sys::SEEK_SET as i32,
    Cur = sys::SEEK_CUR as i32,
    End = sys::SEEK_END as i32,
}
