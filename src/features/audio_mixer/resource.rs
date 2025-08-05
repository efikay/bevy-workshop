use std::collections::HashMap;

use bevy::prelude::*;
use bevy_seedling::{prelude::PoolLabel, sample::Sample};
use strum_macros::Display;

#[derive(Hash, PoolLabel, Debug, Reflect, PartialEq, Eq, Clone, Copy, Display)]
pub enum Stem {
    Instruments,
    Bass,
    Drums,
    Melody,
    Other,
}

#[derive(Default, Reflect, Clone, Copy)]
pub enum AudioMixerStatus {
    #[default]
    Idle,
    Playing,
    Paused,
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct AudioMixer {
    registered_stems: HashMap<Stem, Handle<Sample>>,
    status: AudioMixerStatus,
    audio_timer: Timer,
}
impl Default for AudioMixer {
    fn default() -> Self {
        Self {
            registered_stems: Default::default(),
            status: AudioMixerStatus::default(),
            audio_timer: Timer::default(),
        }
    }
}

impl AudioMixer {
    pub fn register_stem(
        &mut self,
        track: Handle<Sample>,
        stem: &Stem,
    ) -> std::result::Result<(), ()> {
        match self.has_stem(stem) {
            true => {
                log::warn!(
                    "AudioMixer::register_stem: Cannot register track \"{}\" ({}): Already exists",
                    track.path().unwrap(),
                    stem
                );

                Err(())
            }
            false => {
                self.registered_stems.insert(*stem, track.clone());

                log::info!(
                    "AudioMixer::register_stem: Successfully registered track \"{}\" ({})",
                    track.path().unwrap(),
                    stem
                );

                Ok(())
            }
        }
    }
    
    pub fn has_stem(&self, stem: &Stem) -> bool {
        self.registered_stems.contains_key(stem)
    }

    pub fn status(&self) -> AudioMixerStatus {
        self.status
    }
}
