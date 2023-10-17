use std::collections::HashMap;

use macroquad::audio;
use macroquad::audio::Sound;

use crate::utils::rusty_error::RustyResult;

pub enum SoundType {
    Menu,
    Game,
    GameOver,
}

const SOUND_FILE_FOR_SOUND_TYPE: [(usize, &str, f32); 3] = [
    (SoundType::Menu as usize, "assets/musics/music_menu.wav", 1.0),
    (SoundType::Game as usize, "assets/musics/music_menu.wav", 1.0),
    (SoundType::GameOver as usize, "assets/musics/game_over_sound.wav", 1.0),
];

pub struct SoundsManager {
    sounds: HashMap<usize, (Sound, f32)>,
}

impl SoundsManager {
    pub async fn new() -> RustyResult<SoundsManager> {
        let mut sounds: HashMap<usize, (Sound, f32)> = HashMap::new();
        for &(sound_type, sound_file, sound_volume) in &SOUND_FILE_FOR_SOUND_TYPE {
            let sound = audio::load_sound(sound_file).await?;
            audio::set_sound_volume(sound, sound_volume);
            sounds.insert(sound_type, (sound, sound_volume));
        }

        Ok(SoundsManager {
            sounds,
        })
    }

    pub fn play_sound(&mut self, sound_type: SoundType) {
        let sound = self.sounds.get(&(sound_type as usize));
        match sound {
            Some(sound) => {
                audio::play_sound(sound.0, audio::PlaySoundParams {
                    volume: sound.1,
                    looped: true,
                });
            }
            None => unreachable!(),
        }
    }

    pub fn stop_sound(&mut self, sound_type: SoundType) {
        let sound = self.sounds.get(&(sound_type as usize));
        match sound {
            Some(sound) => {
                audio::stop_sound(sound.0);
            }
            None => unreachable!(),
        }
    }
}