use macroquad::prelude::*;

use crate::WINDOW_WIDTH;

pub mod bot_car;
pub mod player_car;
pub mod bot_manager;

pub const PLAYER_CAR_HEIGHT: f32 = 73.0;
pub const PLAYER_CAR_WIDTH: f32 = 155.0;
pub const BOT_CAR_WIDTH: f32 = 140.0;

pub const PLAYER_CAR_X_POSITION: f32 = WINDOW_WIDTH as f32 / 4.0;


pub enum Direction {
    Up,
    Down,
}

#[derive(PartialEq, Hash, Eq, Clone, Copy)]
pub enum Way {
    Upper,
    Center,
    Lower,
}

pub trait Car {
    fn get_texture(&self) -> Texture2D;
    fn get_way(&self) -> Way;
}