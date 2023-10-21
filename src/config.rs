use macroquad::prelude::KeyCode;

use crate::game::sounds::sounds_manager::SoundType;
use crate::keyboard::GameAction;

// Game settings
pub const START_GAME_SPEED: f32 = 500.0;
pub const SPEED_INCREASE_VALUE: f32 = 1.0;
pub const SPEED_INCREASE_TIMING: u16 = 100;

// More the value is high, more de distance is
pub const DISTANCE_BETWEEN_CARS: f32 = 0.4;

// Keyboard settings
pub const KEY_GAME: [(KeyCode, GameAction); 5] = [
    (KeyCode::Z, GameAction::Up),
    (KeyCode::S, GameAction::Down),
    (KeyCode::Space, GameAction::PauseResume),
    (KeyCode::Escape, GameAction::Quit),
    (KeyCode::M, GameAction::Mute)
];

// Sounds settings
pub const SOUND_FILE_FOR_SOUND_TYPE: [(SoundType, &str, f32); 3] = [
    (SoundType::Menu, "assets/musics/menu_music.wav", 1.1),
    (SoundType::Game, "assets/musics/game_music.wav", 0.7),
    (SoundType::GameOver, "assets/musics/game_over_sound.wav", 1.0),
];