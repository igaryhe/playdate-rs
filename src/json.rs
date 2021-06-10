use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json_core;
use anyhow::{Result, Error};
use crate::Playdate;
use crate::file;

pub fn from_file<T: DeserializeOwned>(path: &str) -> Result<T> {
    let mut file = Playdate::get_filesystem()
        .open(path, file::FileOptions::kFileRead).map_err(Error::msg)?;
    let mut array: [u8; 200] = [0; 200];
    let len = file.read(&mut array)? as usize;
    let de = serde_json_core::from_slice(&array[0..len - 1]).map_err(Error::msg)?;
    Ok(de.0)
}

pub fn to_file<T: Serialize>(path: &str, obj: T) -> Result<()> {
    let mut file = Playdate::get_filesystem()
        .open(path, file::FileOptions::kFileRead).map_err(Error::msg)?;
    let ser = serde_json_core::to_string::<T, 128>(&obj).map_err(Error::msg)?;
    file.write(ser.as_bytes())?;
    Ok(())
}
