use macroquad::prelude::{Color, draw_line, draw_text, draw_texture, load_texture, Texture2D, WHITE};
use macroquad::shapes::{draw_rectangle, draw_rectangle_lines};
use macroquad::text::measure_text;

use crate::{GAME_NAME, WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::config::KEY_GAME;
use crate::game::car::{Car, PLAYER_CAR_HEIGHT, Way};
use crate::game::car::bot_car::BotCar;
use crate::game::car::player_car::PlayerCar;
use crate::game::graphics::background::Background;
use crate::keyboard::{GameAction, get_action_description_from_game_action, get_key_code_from_game_action, get_str_from_key_code};
use crate::utils::rusty_error::RustyResult;

const FIRST_LANE_POSITION: f32 = WINDOW_HEIGHT * (220.0 / 720.0);
const SECOND_LANE_POSITION: f32 = WINDOW_HEIGHT / 2.0;
const THIRD_LANE_POSITION: f32 = WINDOW_HEIGHT * (500.0 / 720.0);

const RUSTY_CORKS_TEXT_SIZE: f32 = 60.0;
const ENTER_TEXT_SIZE: f32 = 35.0;

const PLAY_MESSAGE: &str = "Ready to play ? Press ";


const COLLISION_SIZE: f32 = 50.0;
const COLLISION_TEXTURE_PATH: &str = "assets/collision.png";

#[derive(Clone)]
pub struct GraphicsManager {
    pub background: Background,
    pub collision: Texture2D,
}

impl GraphicsManager {
    pub async fn new() -> RustyResult<GraphicsManager> {
        let background = Background::new().await?;
        let collision = load_texture(COLLISION_TEXTURE_PATH).await?;
        Ok(GraphicsManager {
            background,
            collision,
        })
    }

    fn draw_depending_way(&self, texture: Texture2D, way: &Way, x: f32, height_subtraction: f32) {
        match way {
            Way::Upper => {
                draw_texture(texture, x, FIRST_LANE_POSITION - height_subtraction / 2.0, WHITE);
            }
            Way::Center => {
                draw_texture(texture, x, SECOND_LANE_POSITION - height_subtraction / 2.0, WHITE);
            }
            Way::Lower => {
                draw_texture(texture, x, THIRD_LANE_POSITION - height_subtraction / 2.0, WHITE);
            }
        }
    }
    pub fn draw_bot_car(&self, bot_car: &BotCar) {
        self.draw_depending_way(bot_car.get_texture(), &bot_car.get_way(), bot_car.x_position, PLAYER_CAR_HEIGHT);
    }

    pub fn draw_player_car(&self, player_car: &PlayerCar) {
        self.draw_depending_way(player_car.get_texture(), &player_car.get_way(), WINDOW_WIDTH / 4.0, PLAYER_CAR_HEIGHT);
    }

    pub fn draw_score(&self, score: u32) {
        let score_text = format!("Score: {}", score);
        draw_text(&score_text, 0.0, 60.0, 60.0, WHITE);
    }

    pub fn draw_game_over(&self, score: u32, session_record: u32) {
        draw_rectangle((WINDOW_WIDTH / 2.0) - (500.0 / 2.0),
                       (WINDOW_HEIGHT / 2.0) - (250.0 / 2.0),
                       500.0,
                       250.0,
                       Color::new(0.5, 0.5, 0.5, 0.5));

        GraphicsManager::draw_centered_text("Game Over",
                                            (WINDOW_HEIGHT / 2.0) - 50.0,
                                            RUSTY_CORKS_TEXT_SIZE,
                                            WHITE);

        let score_text = format!("Score: {}", score);
        GraphicsManager::draw_centered_text(score_text.as_str(),
                                            (WINDOW_HEIGHT / 2.0) - 10.0,
                                            ENTER_TEXT_SIZE,
                                            WHITE);


        let score_record_text = format!("Session record: {}", session_record);
        GraphicsManager::draw_centered_text(score_record_text.as_str(),
                                            (WINDOW_HEIGHT / 2.0) + 30.0,
                                            ENTER_TEXT_SIZE,
                                            WHITE);

        GraphicsManager::draw_key_text(GameAction::PauseResume);
    }

    pub fn draw_pause(&self, session_record: u32) {
        draw_rectangle((WINDOW_WIDTH / 2.0) - (500.0 / 2.0),
                       (WINDOW_HEIGHT / 2.0) - (250.0 / 2.0),
                       500.0,
                       250.0,
                       Color::new(0.5, 0.5, 0.5, 0.5));

        GraphicsManager::draw_centered_text("Pause",
                                            (WINDOW_HEIGHT / 2.0) - 50.0,
                                            RUSTY_CORKS_TEXT_SIZE,
                                            WHITE);

        let score_record_text = format!("Session record: {}", session_record);
        GraphicsManager::draw_centered_text(score_record_text.as_str(),
                                            (WINDOW_HEIGHT / 2.0) - 10.0,
                                            ENTER_TEXT_SIZE,
                                            WHITE);

        GraphicsManager::draw_key_text(GameAction::PauseResume);
    }

    pub fn draw_new_game(&self) {
        draw_rectangle(0.0, 0.0, WINDOW_WIDTH, WINDOW_HEIGHT, Color::new(0.5, 0.5, 0.5, 0.5));

        GraphicsManager::draw_centered_text(GAME_NAME,
                                            (WINDOW_HEIGHT / 2.0) - 120.0,
                                            RUSTY_CORKS_TEXT_SIZE,
                                            WHITE);


        GraphicsManager::draw_key_text(GameAction::PauseResume);

        draw_rectangle_lines(WINDOW_WIDTH / 2.0 - (500.0 / 2.0), WINDOW_HEIGHT / 2.0 - (180.0 / 2.0), 500.0, 250.0, 5.0, Color::new(0.3, 0.3, 0.3, 0.8));

        GraphicsManager::draw_centered_text("Controls",
                                            (WINDOW_HEIGHT / 2.0) - 67.0,
                                            ENTER_TEXT_SIZE,
                                            WHITE);

        let text_size = measure_text(GAME_NAME, None, RUSTY_CORKS_TEXT_SIZE as u16, 1.0);
        draw_line(WINDOW_WIDTH / 2.0 - (text_size.width / 2.0),
                  WINDOW_HEIGHT / 2.0 - (text_size.height / 2.0) - 37.0,
                  WINDOW_WIDTH / 2.0 - (text_size.width / 2.0) + text_size.width,
                  WINDOW_HEIGHT / 2.0 - (text_size.height / 2.0) - 37.0,
                  3.0,
                  Color::new(1.0, 1.0, 1.0, 0.8));

        GraphicsManager::draw_key_binds(WINDOW_WIDTH / 2.0, WINDOW_HEIGHT / 2.0 - (text_size.height / 2.0), ENTER_TEXT_SIZE, WHITE);
    }

    pub fn draw_collision(&self, way: Way, x: f32) {
        self.draw_depending_way(self.collision, &way, x - COLLISION_SIZE / 2.0, COLLISION_SIZE);
    }

    fn draw_key_text(game_action: GameAction) {
        let key_for_new_game = get_key_code_from_game_action(game_action);
        match key_for_new_game {
            Some(key) => {
                let text_new_game = format!("{}{}", PLAY_MESSAGE, get_str_from_key_code(key));
                GraphicsManager::draw_centered_text(text_new_game.as_str(),
                                                    (WINDOW_HEIGHT / 2.0) + 100.0,
                                                    ENTER_TEXT_SIZE,
                                                    WHITE);
            },
            None => {
                unreachable!();
            }
        }
    }

    fn draw_centered_text(text: &str, y: f32, font_size: f32, color: Color) {
        let text_size = measure_text(text, None, font_size as u16, 1.0);
        draw_text(text, (WINDOW_WIDTH / 2.0) - (text_size.width / 2.0), y, font_size, color);
    }

    fn draw_key_binds(x: f32, y: f32, font_size: f32, color: Color) {
        let mut y_offset = 0.0;
        for key in KEY_GAME {
            let game_action = get_action_description_from_game_action(key.1);
            if let Some(game_action) = game_action {
                let text = format!("{}{}", game_action, get_str_from_key_code(key.0));
                let text_size = measure_text(&text, None, ENTER_TEXT_SIZE as u16, 1.0);
                draw_text(&text, x - (text_size.width / 2.0), y - (text_size.height / 2.0) + y_offset, font_size, color);
                y_offset += font_size;
            }
        }
    }
}