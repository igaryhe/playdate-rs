use sys;
use crate::Playdate;
use core::{ptr, slice};
use cstr_core::CString;
use anyhow::{Result, ensure};

pub use sys::sndCallbackProc as Callback;
pub use sys::SoundFormat as Format;

#[derive(Copy, Clone)]
pub struct Sound {
    sound: *const sys::playdate_sound,
}

impl Sound {
    pub fn new(sound: *const sys::playdate_sound) -> Self {
        Self { sound }
    }

    unsafe fn get_fileplayer(&self) -> *const sys::playdate_sound_fileplayer {
            (*self.sound).fileplayer
    }

    unsafe fn get_sampleplayer(&self) -> *const sys::playdate_sound_sampleplayer {
            (*self.sound).sampleplayer
    }

    unsafe fn get_sample(&self) -> *const sys::playdate_sound_sample {
            (*self.sound).sample
    }
}

pub trait Player {
    fn set_volume(&mut self, left: f32, right: f32);
    fn volume(&self) -> (f32, f32);
    fn length(&self) -> f32;
    fn set_offset(&mut self, offset: f32);
    fn offset(&self) -> f32;
    fn set_rate(&mut self, rate: f32);
    fn rate(&self) -> f32;
    fn is_playing(&self) -> bool;
    fn set_finish_callback(&mut self, callback: Callback);
    fn set_loop_callback(&mut self, callback: Callback);
    fn stop(&mut self);
}

pub struct FilePlayer {
    fp: *mut sys::FilePlayer,
}

impl Drop for FilePlayer {
    fn drop(&mut self) {
        unsafe {
            let fp = *Playdate::get_sound().get_fileplayer();
            fp.freePlayer.unwrap()(self.fp);
        }
    }
}

impl FilePlayer {
    fn new(buffer_size: i32) -> Result<FilePlayer> {
        unsafe {
            let fp = *Playdate::get_sound().get_fileplayer();
            let result = fp.newPlayer.unwrap()();
            ensure!(result != ptr::null_mut(), "FilePlayer failed to create");
            Ok(FilePlayer { fp: result })
        }
    }
    
    pub fn load_into_player(&mut self, path: &str) {
        unsafe {
            let fp = *Playdate::get_sound().get_fileplayer();
            let c_path = CString::new(path).unwrap();
            fp.loadIntoPlayer.unwrap()(self.fp, c_path.as_ptr());
        }
    }

    pub fn set_buffer_length(&mut self, buffer_len: f32) {
        unsafe {
            let fp = *Playdate::get_sound().get_fileplayer();
            fp.setBufferLength.unwrap()(self.fp, buffer_len);
        }
    }

    pub fn play(&mut self, repeat: u32) -> i32 {
        unsafe {
            let fp = *Playdate::get_sound().get_fileplayer();
            fp.play.unwrap()(self.fp, repeat as i32)
        }
    }

    pub fn pause(&mut self) {
        unsafe {
            let fp = *Playdate::get_sound().get_fileplayer();
            fp.pause.unwrap()(self.fp);
        }
    }

    pub fn did_underrun(&self) -> bool {
        unsafe {
            let fp = *Playdate::get_sound().get_fileplayer();
            match fp.didUnderrun.unwrap()(self.fp) {
                1 => true,
                _ => false,
            }
        }
    }

    pub fn set_stop_on_underrun(&self, flag: i32) {
        unsafe {
            let fp = *Playdate::get_sound().get_fileplayer();
            fp.setStopOnUnderrun.unwrap()(self.fp, flag);
        }
    }
    
    fn set_loop_range(&mut self, start: f32, end: f32) {
        unsafe {
            let fp = *Playdate::get_sound().get_fileplayer();
            fp.setLoopRange.unwrap()(self.fp, start, end);
        }
    }
}

impl Player for FilePlayer {
    fn is_playing(&self) -> bool {
        unsafe {
            let fp = *Playdate::get_sound().get_fileplayer();
            match fp.isPlaying.unwrap()(self.fp) {
                1 => true,
                _ => false,
            }
        }
    }
    fn stop(&mut self) {
        unsafe {
            let fp = *Playdate::get_sound().get_fileplayer();
            fp.stop.unwrap()(self.fp);
        }
    }

    fn set_volume(&mut self, left: f32, right: f32) {
        unsafe {
            let fp = *Playdate::get_sound().get_fileplayer();
            fp.setVolume.unwrap()(self.fp, left, right);
        }
    }

    fn volume(&self) -> (f32, f32) {
        unsafe {
            let left = ptr::null_mut();
            let right = ptr::null_mut();
            let fp = *Playdate::get_sound().get_fileplayer();
            fp.getVolume.unwrap()(self.fp, left, right);
            (*left, *right)
        }
    }

    fn length(&self) -> f32 {
        unsafe {
            let fp = *Playdate::get_sound().get_fileplayer();
            fp.getLength.unwrap()(self.fp)
        }
    }

    fn set_offset(&mut self, offset: f32) {
        unsafe {
            let fp = *Playdate::get_sound().get_fileplayer();
            fp.setOffset.unwrap()(self.fp, offset);
        }
    }

    fn offset(&self) -> f32 {
        unsafe {
            let fp = *Playdate::get_sound().get_fileplayer();
            fp.getOffset.unwrap()(self.fp)
        }
    }
    
    fn set_rate(&mut self, rate: f32) {
        unsafe {
            let fp = *Playdate::get_sound().get_fileplayer();
            fp.setRate.unwrap()(self.fp, rate);
        }
    }

    fn rate(&self) -> f32 {
        unsafe {
            let fp = *Playdate::get_sound().get_fileplayer();
            fp.getRate.unwrap()(self.fp)
        }
    }

    fn set_finish_callback(&mut self, callback: Callback) {
        unsafe {
            let fp = *Playdate::get_sound().get_fileplayer();
            fp.setFinishCallback.unwrap()(self.fp, callback);
        }
    }

    fn set_loop_callback(&mut self, callback: Callback) {
        unsafe {
            let fp = *Playdate::get_sound().get_fileplayer();
            fp.setLoopCallback.unwrap()(self.fp, callback);
        }
    }
}

pub struct SamplePlayer {
    sp: *mut sys::SamplePlayer,
}

impl Drop for SamplePlayer {
    fn drop(&mut self) {
        unsafe {
            let sp = *Playdate::get_sound().get_sampleplayer();
            sp.freePlayer.unwrap()(self.sp);
        }
    }
}

impl SamplePlayer {
    fn new() -> Result<SamplePlayer> {
        unsafe {
            let sp = *Playdate::get_sound().get_sampleplayer();
            let result = sp.newPlayer.unwrap()();
            ensure!(result != ptr::null_mut(), "SamplePlayer failed to create");
            Ok(SamplePlayer { sp: result })
        }
    }

    fn set_sample(&mut self, sample: Sample) {
        unsafe {
            let sp = *Playdate::get_sound().get_sampleplayer();
            sp.setSample.unwrap()(self.sp, sample.sample);
        }
    }

    fn play(&mut self, repeat: i32, rate: f32) -> i32 {
        unsafe {
            let sp = *Playdate::get_sound().get_sampleplayer();
            sp.play.unwrap()(self.sp, repeat, rate)
        }
    }
}

impl Player for SamplePlayer {
    fn is_playing(&self) -> bool {
        unsafe {
            let sp = *Playdate::get_sound().get_sampleplayer();
            match sp.isPlaying.unwrap()(self.sp) {
                1 => true,
                _ => false,
            }
        }
    }
    fn stop(&mut self) {
        unsafe {
            let sp = *Playdate::get_sound().get_sampleplayer();
            sp.stop.unwrap()(self.sp);
        }
    }

    fn set_volume(&mut self, left: f32, right: f32) {
        unsafe {
            let sp = *Playdate::get_sound().get_sampleplayer();
            sp.setVolume.unwrap()(self.sp, left, right);
        }
    }

    fn volume(&self) -> (f32, f32) {
        unsafe {
            let left = ptr::null_mut();
            let right = ptr::null_mut();
            let sp = *Playdate::get_sound().get_sampleplayer();
            sp.getVolume.unwrap()(self.sp, left, right);
            (*left, *right)
        }
    }

    fn length(&self) -> f32 {
        unsafe {
            let sp = *Playdate::get_sound().get_sampleplayer();
            sp.getLength.unwrap()(self.sp)
        }
    }

    fn set_offset(&mut self, offset: f32) {
        unsafe {
            let sp = *Playdate::get_sound().get_sampleplayer();
            sp.setOffset.unwrap()(self.sp, offset);
        }
    }

    fn offset(&self) -> f32 {
        unsafe {
            let sp = *Playdate::get_sound().get_sampleplayer();
            sp.getOffset.unwrap()(self.sp)
        }
    }
    
    fn set_rate(&mut self, rate: f32) {
        unsafe {
            let sp = *Playdate::get_sound().get_sampleplayer();
            sp.setRate.unwrap()(self.sp, rate);
        }
    }

    fn rate(&self) -> f32 {
        unsafe {
            let sp = *Playdate::get_sound().get_sampleplayer();
            sp.getRate.unwrap()(self.sp)
        }
    }

    fn set_finish_callback(&mut self, callback: Callback) {
        unsafe {
            let sp = *Playdate::get_sound().get_sampleplayer();
            sp.setFinishCallback.unwrap()(self.sp, callback);
        }
    }

    fn set_loop_callback(&mut self, callback: Callback) {
        unsafe {
            let sp = *Playdate::get_sound().get_sampleplayer();
            sp.setLoopCallback.unwrap()(self.sp, callback);
        }
    }
}

pub struct Sample {
    sample: *mut sys::AudioSample,
}

impl Sample {
    pub fn new(byte_count: i32) -> Result<Sample> {
        unsafe {
            let sample = *Playdate::get_sound().get_sample();
            let result = sample.newSampleBuffer.unwrap()(byte_count);
            ensure!(result != ptr::null_mut(), "Fail to create new Sample");
            Ok( Sample { sample: result })
        }
    }

    pub fn load(path: &str) -> Result<Sample> {
        unsafe {
            let c_path = CString::new(path).unwrap();
            let sample = *Playdate::get_sound().get_sample();
            let result = sample.load.unwrap()(c_path.as_ptr());
            ensure!(result != ptr::null_mut(), "Fail to load Sample");
            Ok(Sample { sample: result })
        }
    }

    pub fn new_sample_from_data(data: &mut [u8], format: Format, sample_rate: u32) -> Result<Sample> {
        unsafe {
            let sample = *Playdate::get_sound().get_sample();
            let result = sample.newSampleFromData.unwrap()(data.as_mut_ptr(), format, sample_rate, data.len() as i32);
            ensure!(result != ptr::null_mut(), "Fail to load Sample from data");
            Ok(Sample { sample: result })
        }
    }

    pub fn load_into_sample(&mut self, path: &str) -> i32 {
        unsafe {
            let c_path = CString::new(path).unwrap();
            let sample = *Playdate::get_sound().get_sample();
            sample.loadIntoSample.unwrap()(self.sample, c_path.as_ptr())
        }
    }

    pub fn get_data(&self) -> Result<(&'static mut [u8], Format, u32)> {
        unsafe {
            let raw_data = ptr::null_mut();
            let format = ptr::null_mut();
            let sample_rate = ptr::null_mut();
            let byte_length = ptr::null_mut();
            let sample = *Playdate::get_sound().get_sample();
            sample.getData.unwrap()(self.sample, raw_data, format, sample_rate, byte_length);
            ensure!(!raw_data.is_null(), "Fail to get data");
            let data = slice::from_raw_parts_mut(*raw_data, byte_length as usize);
            Ok((data, *format, *sample_rate))
        }
    }

    pub fn length(&self) -> f32 {
        unsafe {
            let sample = *Playdate::get_sound().get_sample();
            sample.getLength.unwrap()(self.sample)
        }
    }
}

impl Drop for Sample {
    fn drop(&mut self) {
        unsafe {
            let sample = *Playdate::get_sound().get_sample();
            sample.freeSample.unwrap()(self.sample);
        }
    }
}
