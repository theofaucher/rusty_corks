use std::path::Path;

use macroquad::prelude::{load_texture, screen_width, Texture2D};
use rand::Rng;

use crate::game::car::{BOT_CAR_WIDTH, Car, PLAYER_CAR_WIDTH, PLAYER_CAR_X_POSITION, Way};
use crate::game::car::player_car::PlayerCar;
use crate::utils::rusty_error::RustyResult;

const BOT_CAR_TEXTURE_PATH: &str = "assets/cars/bots/";

#[derive(Clone)]
pub struct BotCar {
    texture: Texture2D,
    way: Way,
    pub x_position: f32,
    speed: f32,
}

impl BotCar {
    pub async fn new(way: Way) -> RustyResult<BotCar> {
        let mut png_path = Vec::new();
        let directory = Path::new(BOT_CAR_TEXTURE_PATH);

        // Code complexe mais nous n'avons pas le choix
        if directory.is_dir() {
            for entry in (std::fs::read_dir(directory)?).flatten() {
                let file = entry.path();
                if file.is_file() {
                    if let Some(extension) = file.extension() {
                        if extension.to_string_lossy().to_lowercase() == "png" {
                            png_path.push(file.as_path().to_string_lossy().to_string());
                        }
                    }
                }
            }
        }

        let mut rng = rand::thread_rng();
        let texture_rng = png_path.get(rng.gen_range(0..png_path.len()));

        if let Some(texture_rng) = texture_rng {
            let car_texture = load_texture(texture_rng).await?;
            Ok(BotCar {
                texture: car_texture,
                way,
                speed: 0.0,
                x_position: screen_width(),
            })
        } else {
            unreachable!();
        }
    }

    pub fn update_position(&mut self, delta_time: f32) {
        self.x_position -= self.speed * delta_time;
    }

    pub fn is_out_of_screen(&self) -> bool {
        self.x_position < -screen_width() - BOT_CAR_WIDTH
    }
    pub fn is_colliding(&self, player_car: &PlayerCar) -> Option<(Way, f32)> {
        let mut ret = None;
        if player_car.get_way() == self.way {
            let bot_car_back_x = self.x_position;
            let bot_car_front_x = self.x_position + PLAYER_CAR_WIDTH;
            let player_car_back_x = PLAYER_CAR_X_POSITION;
            let player_car_front_x = PLAYER_CAR_X_POSITION + PLAYER_CAR_WIDTH;

            // Vérifier s'il y a une intersection en X
            if !(bot_car_front_x <= player_car_back_x || player_car_front_x <= bot_car_back_x) {
                // Calculer la position de l'intersection en X
                let overlap_back = bot_car_back_x.max(player_car_back_x);
                let overlap_front = bot_car_front_x.min(player_car_front_x);

                ret = Some((self.way, overlap_back + (overlap_front - overlap_back) / 2.0))
            }
        }
        ret
    }

    pub fn set_speed(&mut self, speed: f32) {
        self.speed = speed;
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