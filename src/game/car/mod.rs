use macroquad::prelude::*;

use crate::WINDOW_WIDTH;

pub mod bot_car;
pub mod player_car;
pub mod bot_manager;

pub const PLAYER_CAR_HEIGHT: f32 = 73.0;
pub const PLAYER_CAR_WIDTH: f32 = 155.0;
pub const BOT_CAR_WIDTH: f32 = 140.0;
pub const PLAYER_CAR_X_POSITION: f32 = WINDOW_WIDTH / 4.0;

#[derive(PartialEq, Hash, Eq, Clone, Copy)]
pub enum Way {
    Upper,
    Center,
    Lower,
}

/// Trait for all cars, but it is not a entire generic type
/// to separate the player car and the bot car
pub trait Car {
    fn get_texture(&self) -> Texture2D;
    fn get_way(&self) -> Way;
}