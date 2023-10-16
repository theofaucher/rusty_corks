use macroquad::prelude::{Color, draw_text, draw_texture, load_texture, screen_height, screen_width, Texture2D, WHITE};
use macroquad::shapes::draw_rectangle;
use macroquad::text::measure_text;

use crate::game::car::{Car, PLAYER_CAR_HEIGHT, Way};
use crate::game::car::bot_car::BotCar;
use crate::game::car::player_car::PlayerCar;
use crate::game::graphics::background::Background;
use crate::GAME_NAME;
use crate::utils::rusty_error::RustyResult;

const RUSTY_CORKS_TEXT_SIZE: f32 = 60.0;
const ENTER_TEXT_SIZE: f32 = 30.0;
const RESTART_MESSAGE: &str = "Press Space to start";
const COLLISION_SIZE: f32 = 50.0;

#[derive(Clone)]
pub struct GraphicsManager {
    pub background: Background,
    pub collision: Texture2D,
}

impl GraphicsManager {
    pub async fn new() -> RustyResult<GraphicsManager> {
        let background = Background::new().await?;
        let collision = load_texture("assets/collision.png").await?;
        Ok(GraphicsManager {
            background,
            collision,
        })
    }
    fn draw_depending_way(&self, texture: Texture2D, way: &Way, x: f32, height_soustraction: f32) {
        match way {
            Way::Upper => {
                draw_texture(texture, x, screen_height() * (220.0 / 720.0) - height_soustraction / 2.0, WHITE);
            }
            Way::Center => {
                draw_texture(texture, x, screen_height() / 2.0 - height_soustraction / 2.0, WHITE);
            }
            Way::Lower => {
                draw_texture(texture, x, screen_height() * (500.0 / 720.0) - height_soustraction / 2.0, WHITE);
            }
        }
    }

    pub fn draw_bot_car(&self, bot_car: &BotCar) {
        self.draw_depending_way(bot_car.get_texture(), &bot_car.get_way(), bot_car.x_position, PLAYER_CAR_HEIGHT);
    }

    pub fn draw_player_car(&self, player_car: &PlayerCar) {
        self.draw_depending_way(player_car.get_texture(), &player_car.get_way(), screen_width() / 4.0, PLAYER_CAR_HEIGHT);
    }

    pub fn draw_score(&self, score: u32) {
        let score_text = format!("Score: {}", score);
        draw_text(&score_text, 0.0, 60.0, 60.0, WHITE);
    }

    pub fn draw_game_over(&self, score: u32, session_record: u32) {
        draw_rectangle((screen_width() / 2.0) - (500.0 / 2.0),
                       (screen_height() / 2.0) - (250.0 / 2.0),
                       500.0,
                       250.0,
                       Color::new(0.5, 0.5, 0.5, 0.5));

        draw_text("Game Over",
                  (screen_width() / 2.0) - 120.0,
                  (screen_height() / 2.0) - 50.0,
                  60.0,
                  WHITE);

        let score_text = format!("Score: {} Session record: {}", score, session_record);
        draw_text(&score_text,
                  (screen_width() / 2.0) - 200.0,
                  (screen_height() / 2.0) - 10.0,
                  35.0,
                  WHITE);

        draw_text("Press Space to restart",
                  (screen_width() / 2.0) - 150.0,
                  (screen_height() / 2.0) + 100.0,
                  30.0,
                  WHITE);
    }

    pub fn draw_new_game(&self) {
        draw_rectangle(0.0, 0.0, screen_width(), screen_height(), Color::new(0.5, 0.5, 0.5, 0.5));

        let text_size = measure_text(GAME_NAME, None, RUSTY_CORKS_TEXT_SIZE as u16, 1.0);
        draw_text(GAME_NAME,
                  (screen_width() / 2.0) - (text_size.width / 2.0),
                  (screen_height() / 2.0) - (text_size.height / 2.0),
                  RUSTY_CORKS_TEXT_SIZE,
                  WHITE);

        let text_size = measure_text(RESTART_MESSAGE, None, ENTER_TEXT_SIZE as u16, 1.0);
        draw_text(RESTART_MESSAGE,
                  (screen_width() / 2.0) - (text_size.width / 2.0),
                  (screen_height() / 2.0) - (text_size.height / 2.0) + 20.0,
                  ENTER_TEXT_SIZE,
                  WHITE);
    }

    pub fn draw_collision(&self, way: Way, x: f32) {
        self.draw_depending_way(self.collision, &way, x - COLLISION_SIZE / 2.0, COLLISION_SIZE);
    }
}