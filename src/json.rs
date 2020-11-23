use sys;
use serde::{Serialize, Deserialize};
use serde::de::DeserializeOwned;
use serde_json;
use anyhow::{Result, Error};
use crate::Playdate;
use crate::file;

#[derive(Copy, Clone)]
pub struct Json {
    json: *mut sys::playdate_json,
}

impl Json {
    pub fn new(json: *mut sys::playdate_json) -> Self {
        Self { json }
    }
}

pub fn from_file<T: DeserializeOwned>(path: &str) -> Result<T> {
    let mut file = Playdate::get_filesystem()
        .open(path, file::FileOptions::kFileRead).map_err(Error::msg)?;
    let mut array: [u8; 200] = [0; 200];
    let len = file.read(&mut array)? as usize;
    let de: T = serde_json::from_slice(&array[0..len - 1]).map_err(Error::msg)?;
    Ok(de)
}

pub fn to_file<T: Serialize>(path: &str, obj: T) -> Result<()> {
    let mut file = Playdate::get_filesystem()
        .open(path, file::FileOptions::kFileRead).map_err(Error::msg)?;
    let ser = serde_json::to_string(&obj).map_err(Error::msg)?;
    file.write(ser.as_bytes())?;
    Ok(())
}
