
use macroquad::prelude::*;

const PLAYER_CAR_HEIGHT: f32 = 73.0;
const BOT_CAR_WIDTH: f32 = 160.0;


pub trait Car {
    fn draw(&self);
}

pub enum Direction {
    Up,
    Down,
}

#[derive(PartialEq)]
pub enum Way {
    Upper,
    Center,
    Lower,
}

pub struct PlayerCar {
    texture: Texture2D,
    way: Way,
}

impl PlayerCar {
    pub fn new(texture : Texture2D) -> PlayerCar {
        PlayerCar { texture, way: Way::Center}
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
        self.draw();
    }
}

#[derive(PartialEq)]
pub struct BotCar {
    texture: Texture2D,
    way: Way,
    pub x_position: f32,
    speed: f32,
}

impl BotCar {
    pub fn new(texture : Texture2D, way: Way, speed: f32) -> BotCar {
        BotCar { texture, way, speed, x_position: screen_width()}
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



