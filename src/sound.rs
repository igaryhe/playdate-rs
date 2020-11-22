use sys;

#[derive(Copy, Clone)]
pub struct Sound {
    sound: *mut sys::playdate_sound,
    channel: Channel,
    fileplayer: Fileplayer,
    sample: Sample,
    sampleplayer: Sampleplayer,
    synth: Synth,
    sequencer: Sequencer,
    effect: Effect,
    signal: Signal,
}

impl Sound {
    pub fn new(sound: *mut sys::playdate_sound) -> Self {
        unsafe {
            Self {
                sound,
                channel: Channel { channel: (*sound).channel },
                fileplayer: Fileplayer { fileplayer: (*sound).fileplayer },
                sample: Sample { sample: (*sound).sample },
                sampleplayer: Sampleplayer { sampleplayer: (*sound).sampleplayer },
                synth: Synth { synth: (*sound).synth },
                sequencer: Sequencer { sequencer: (*sound).sequencer },
                effect: Effect { effect: (*sound).effect },
                signal: Signal { signal: (*sound).signal },
            }   
        }
    }

    pub fn start(&self) {
        unsafe {
            (*self.sound).start.unwrap()()
        }
    }

    pub fn stop(&self) {
        unsafe {
            (*self.sound).stop.unwrap()()
        }
    }
}

#[derive(Copy, Clone)]
pub struct Fileplayer {
    fileplayer: *mut sys::playdate_sound_fileplayer,
}

#[derive(Copy, Clone)]
pub struct Sample {
    sample: *mut sys::playdate_sound_sample,
}

#[derive(Copy, Clone)]
pub struct Sampleplayer {
    sampleplayer: *mut sys::playdate_sound_sampleplayer,
}

#[derive(Copy, Clone)]
pub struct Channel {
    channel: *mut sys::playdate_sound_channel,
}

#[derive(Copy, Clone)]
pub struct Synth {
    synth: *mut sys::playdate_sound_synth,
}

#[derive(Copy, Clone)]
pub struct Sequencer {
    sequencer: *mut sys::playdate_sound_sequencer,
}

#[derive(Copy, Clone)]
pub struct Effect {
    effect: *mut sys::playdate_sound_effect,
}

#[derive(Copy, Clone)]
pub struct Signal {
    signal: *mut sys::playdate_sound_signal,
}
