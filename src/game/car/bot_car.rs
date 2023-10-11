use macroquad::prelude::{load_texture, screen_width, Texture2D};
use rand::Rng;

use crate::game::car::{BOT_CAR_WIDTH, PLAYER_CAR_WIDTH, PLAYER_CAR_X_POSITION, Way};
use crate::game::car::player_car::PlayerCar;
use crate::utils::rusty_error::RustyResult;

#[derive(PartialEq, Copy)]
#[derive(Clone)]
pub struct BotCar {
    pub texture: Texture2D,
    pub way: Way,
    pub x_position: f32,
    speed: f32,
}

impl BotCar {
    pub async fn new(way: Way, speed: f32) -> RustyResult<BotCar> {
        let mut rng = rand::thread_rng();
        let path = match rng.gen_range(0..2) {
            0 => "assets/blackCar.png",
            1 => "assets/redCar.png",
            _ => "assets/redCar.png",
        };
        let car_texture = load_texture(path).await?;
        Ok(BotCar {
            texture: car_texture,
            way,
            speed,
            x_position: screen_width(),
        })
    }

    pub fn set_speed(&mut self, new_speed: f32) {
        {
            self.speed = new_speed;
        }
    }
    pub fn update_position(&mut self, delta_time: f32) {
        {
            self.x_position -= self.speed * delta_time;
        }
    }
    pub fn is_out_of_screen(&self) -> bool {
        self.x_position < -screen_width() - BOT_CAR_WIDTH
    }
    pub fn is_colliding(&self, player_car: &PlayerCar) -> bool {
        let mut ret = false;
        if player_car.way == self.way {
            ret = (self.x_position < (PLAYER_CAR_X_POSITION + PLAYER_CAR_WIDTH)) && (self.x_position > (PLAYER_CAR_X_POSITION - BOT_CAR_WIDTH));
        }
        ret
    }
}