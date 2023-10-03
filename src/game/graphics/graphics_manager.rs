use std::sync::MutexGuard;
use macroquad::prelude::{draw_texture, screen_height, screen_width, Texture2D, WHITE};
use crate::game::car::{PLAYER_CAR_HEIGHT, Way};
use crate::game::car::bot_car::BotCar;
use crate::game::car::player_car::PlayerCar;

use crate::game::graphics::background::Background;

pub struct GraphicsManager {
    pub background: Background,
}

impl GraphicsManager {
    pub async fn new() -> Option<GraphicsManager> {
        let background = Background::new().await;

        background.map(|background| GraphicsManager {
            background,
        })
    }

    fn draw_car(&self, texture: Texture2D, way: &Way, x: f32) {
        match way {
            Way::Upper => {
                draw_texture(texture, x, screen_height() * (220.0 / 720.0) - PLAYER_CAR_HEIGHT / 2.0, WHITE);
            }
            Way::Center => {
                draw_texture(texture, x, screen_height() / 2.0 - PLAYER_CAR_HEIGHT / 2.0, WHITE);
            }
            Way::Lower => {
                draw_texture(texture, x, screen_height() * (500.0 / 720.0) - PLAYER_CAR_HEIGHT / 2.0, WHITE);
            }
        }
    }

    pub fn draw_bot_car(&self, bot_car: &BotCar) {
        self.draw_car(bot_car.texture, &bot_car.way, bot_car.x_position);
    }

    pub fn draw_player_car(&self, player_car: MutexGuard<PlayerCar>) {
        self.draw_car(player_car.texture, &player_car.way, screen_width() / 4.0);
    }
}