use std::sync::{Arc, Mutex};

use macroquad::prelude::{load_texture, screen_width, Texture2D};
use rand::Rng;

use crate::game::car::{BOT_CAR_WIDTH, Car, PLAYER_CAR_WIDTH, PLAYER_CAR_X_POSITION, Way};
use crate::game::car::player_car::PlayerCar;
use crate::utils::rusty_error::{LockError, RustyResult};
use crate::utils::rusty_error::RustyError::RustyLock;

#[derive(Clone)]
pub struct BotCar {
    texture: Texture2D,
    way: Way,
    pub x_position: f32,
    speed: Arc<Mutex<f32>>,
}

impl BotCar {
    pub async fn new(way: Way, speed: Arc<Mutex<f32>>) -> RustyResult<BotCar> {
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

    pub fn update_position(&mut self, delta_time: f32) -> RustyResult<()>
        {
            let current_speed = self.speed.lock().map_err(|e| RustyLock(LockError {
                message: format!("Impossible to lock the access to the current speed: {}", e),
            }))?;
            self.x_position -= *current_speed * delta_time;
            Ok(())
        }

    pub fn is_out_of_screen(&self) -> bool {
        self.x_position < -screen_width() - BOT_CAR_WIDTH
    }
    pub fn is_colliding(&self, player_car: &PlayerCar) -> bool {
        let mut ret = false;
        if player_car.get_way() == self.way {
            ret = (self.x_position < (PLAYER_CAR_X_POSITION + PLAYER_CAR_WIDTH)) && (self.x_position > (PLAYER_CAR_X_POSITION - BOT_CAR_WIDTH));
        }
        ret
    }
}

impl Car for BotCar {
    fn get_texture(&self) -> Texture2D {
        self.texture
    }

    fn get_way(&self) -> Way {
        self.way
    }
}