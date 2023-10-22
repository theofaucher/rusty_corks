use std::collections::HashMap;

use macroquad::audio;

use crate::config::SOUND_FILE_FOR_SOUND_TYPE;
use crate::game::sounds::rusty_sound::RustySound;
use crate::utils::rusty_error::RustyResult;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum SoundType {
    Menu,
    Game,
    GameOver,
}

pub struct SoundsManager {
    // Contains the sound type, the sound object
    sounds: HashMap<SoundType, RustySound>,
    sounds_muted: bool,
}

impl SoundsManager {
    pub async fn new() -> RustyResult<SoundsManager> {
        let mut sounds: HashMap<SoundType, RustySound> = HashMap::new();
        for &(sound_type, sound_file, sound_volume) in &SOUND_FILE_FOR_SOUND_TYPE {
            // Save the sound type and the sound object in a hashmap
            // to be able to find the sound object by the sound type
            sounds.insert(sound_type, RustySound::new(sound_file, sound_volume).await?);
        }

        Ok(SoundsManager {
            sounds,
            sounds_muted: false,
        })
    }

    pub fn play_sound(&mut self, sound_type: SoundType, play_loop: bool) {
        // Get the sound object from the hashmap by the sound type
        let sound = self.sounds.get_mut(&(sound_type));
        match sound {
            Some(sound) => {
                // If the sounds are muted, the volume is 0.0
                let volume = if self.sounds_muted { 0.0 } else { sound.volume };
                sound.playing_status = true;
                audio::play_sound(sound.sound, audio::PlaySoundParams {
                    volume,
                    looped: play_loop,
                });
            }
            None => unreachable!(),
        }
    }

    pub fn stop_sound(&mut self, sound_type: SoundType) {
        // Get the sound object from the hashmap by the sound type
        let sound = self.sounds.get_mut(&(sound_type));
        match sound {
            Some(sound) => {
                sound.playing_status = false;
                audio::stop_sound(sound.sound);
            }
            None => unreachable!(),
        }
    }

    pub fn set_mute_songs(&mut self) {
        for sound in self.sounds.values_mut() {
            self.sounds_muted = !self.sounds_muted;
            // If the sounds are not muted, the volume is the original volume if the sound is playing
            if self.sounds_muted {
                audio::set_sound_volume(sound.sound, 0.0);
            } else if !self.sounds_muted && sound.playing_status {
                audio::set_sound_volume(sound.sound, sound.volume);
            }
        }
    }
}