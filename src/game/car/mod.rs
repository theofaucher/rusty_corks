pub mod bot_car;
pub mod player_car;

use macroquad::prelude::*;

pub const PLAYER_CAR_HEIGHT: f32 = 73.0;
pub const BOT_CAR_WIDTH: f32 = 160.0;


pub trait Car {
    fn draw(&self);
}

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