use macroquad::prelude::{draw_texture, load_texture, screen_height, screen_width, Texture2D, WHITE};
use crate::game::car::{BOT_CAR_WIDTH, Car, PLAYER_CAR_HEIGHT, Way};
use crate::game::car::player_car::PlayerCar;

#[derive(PartialEq)]
pub struct BotCar {
    pub texture: Texture2D,
    pub way: Way,
    pub x_position: f32,
    speed: f32,
}

impl BotCar {
    pub async fn new(way: Way, speed: f32) -> Option<BotCar> {
        let background_texture = load_texture("assets/blackCar.png").await;
        match background_texture {
            Ok(texture) => Some(BotCar {
                texture,
                way,
                speed,
                x_position: screen_width()
            }),
            Err(e) => {
                println!("Error loading texture: {}", e);
                None
            }
        }
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
        let mut ret: bool = false;
        match self.way {
            Way::Upper => {
                if player_car.way == Way::Upper {
                    ret = true;
                }
            },
            Way::Center => {
                if player_car.way == Way::Center {
                    ret = true;
                }
            },
            Way::Lower => {
                if player_car.way == Way::Lower {
                    ret = true;
                }
            },
        }
        ret
    }
}

impl Car for BotCar {
    fn draw(&self) {
        match self.way {
            Way::Upper => {
                draw_texture(self.texture, self.x_position,screen_height() * (220.0/720.0) - PLAYER_CAR_HEIGHT/2.0  , WHITE);
            },
            Way::Center => {
                draw_texture(self.texture, self.x_position, screen_height()/2.0 - PLAYER_CAR_HEIGHT/2.0, WHITE);
            },
            Way::Lower => {
                draw_texture(self.texture, self.x_position, screen_height() * (500.0/720.0) - PLAYER_CAR_HEIGHT/2.0, WHITE);
            },
        }
    }
}