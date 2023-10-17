use macroquad::prelude::KeyCode;

use crate::game::sounds::sounds_manager::SoundType;

// Window settings
pub const WINDOW_WIDTH: i32 = 1280;
pub const WINDOW_HEIGHT: i32 = 720;
pub const GAME_NAME: &str = "Rusty Corks";

// Game settings
pub const START_GAME_SPEED: f32 = 300.0;
pub const DISTANCE_BETWEEN_CARS: f32 = 2.5;

// Keyboard settings
pub const KEY_GAME: [(KeyCode, &str); 5] = [
    (KeyCode::Z, "Monter avec : {}"),
    (KeyCode::S, "Descendre avec : "),
    (KeyCode::Space, "Mettre en pause ou reprendre avec : "),
    (KeyCode::Escape, "Quitter avec : "),
    (KeyCode::M, "Désactiver le son avec : ")
];

// Sounds settings
pub const SOUND_FILE_FOR_SOUND_TYPE: [(usize, &str, f32); 3] = [
    (SoundType::Menu as usize, "assets/musics/menu_music.wav", 1.1),
    (SoundType::Game as usize, "assets/musics/game_music.wav", 0.7),
    (SoundType::GameOver as usize, "assets/musics/game_over_sound.wav", 1.0),
];