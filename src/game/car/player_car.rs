use macroquad::prelude::{draw_texture, load_texture, screen_height, screen_width, Texture2D, WHITE};
use crate::game::car::{Car, Direction, PLAYER_CAR_HEIGHT, Way};

pub struct PlayerCar {
    pub texture: Texture2D,
    pub way: Way,
}

impl PlayerCar {
    pub async fn new() -> Option<PlayerCar> {
        let background_texture = load_texture("assets/playerCar.png").await;
        match background_texture {
            Ok(texture) => Some(PlayerCar {
                texture,
                way: Way::Center
            }),
            Err(e) => {
                println!("Error loading texture: {}", e);
                None
            }
        }
    }

    pub fn move_car(&mut self, dir: Direction) {
        match dir {
            Direction::Up => {
                match self.way {
                    Way::Upper => self.way = Way::Upper,
                    Way::Center => self.way = Way::Upper,
                    Way::Lower => self.way = Way::Center,
                }
            },
            Direction::Down => {
                match self.way {
                    Way::Upper => self.way = Way::Center,
                    Way::Center => self.way = Way::Lower,
                    Way::Lower => self.way = Way::Lower,
                }
            },
        }
    }
}



impl Car for PlayerCar {
    fn draw(&self) {
        match self.way {
            Way::Upper => {
                draw_texture(self.texture, screen_width()/4.0,screen_height() * (220.0/720.0) - PLAYER_CAR_HEIGHT/2.0  , WHITE);
            },
            Way::Center => {
                draw_texture(self.texture, screen_width()/4.0, screen_height()/2.0 - PLAYER_CAR_HEIGHT/2.0, WHITE);
            },
            Way::Lower => {
                draw_texture(self.texture, screen_width()/4.0, screen_height() * (500.0/720.0) - PLAYER_CAR_HEIGHT/2.0, WHITE);
            },
        }
    }
}