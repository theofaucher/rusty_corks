use macroquad::audio;
use macroquad::audio::Sound;

use crate::utils::rusty_error::RustyResult;

pub struct RustySound {
    pub sound: Sound,
    pub volume: f32,
    pub playing_status: bool,
}

impl RustySound {
    pub async fn new(sound_path: &str, sound_volume: f32) -> RustyResult<RustySound> {
        let sound = audio::load_sound(sound_path).await?;
        audio::set_sound_volume(sound, sound_volume);
        Ok(RustySound {
            sound,
            volume: sound_volume,
            playing_status: false,
        })
    }
}