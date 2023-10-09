use macroquad::prelude::*;

use crate::game::game::Game;
use crate::game::graphics::graphics_manager::GraphicsManager;
use crate::keyboard::keyboard_observer::KeyboardObserver;

mod game;
mod keyboard;
mod utils;

struct Background {
    texture: Texture2D,
    position: Vec2,
    speed: f32,
}

#[macroquad::main(window_conf())]
async fn main() {


    let graphics_manager = GraphicsManager::new().await;
    let mut graphics_manager = match graphics_manager {
        Some(game_manager) => game_manager,
        None => return,
    };


    let (sender, receiver) = std::sync::mpsc::channel::<macroquad::input::KeyCode>();
    let observer = KeyboardObserver::new(sender);
    observer.start_observer();

    let game = Game::new(receiver).await;
    let mut game = match game {
        Some(game) => game,
        None => return,
    };

    game.start();

    loop {
        let delta_time = get_frame_time();

        graphics_manager.background.update_position(delta_time);

        let player_car_lock = game.player_car.lock();
        {
            let player_car = match player_car_lock {
                Ok(player_car) => player_car,
                Err(e) => {
                    println!("Error lock player car: {}", e);
                    break;
                }
            };
            graphics_manager.draw_player_car(player_car);
        }

        let current_score_lock = game.score.lock();
        {
            let current_score = match current_score_lock {
                Ok(current_score) => current_score,
                Err(e) => {
                    println!("Error lock current score: {}", e);
                    break;
                }
            };
            graphics_manager.draw_score(*current_score);
        }

        next_frame().await;
    }
}

pub fn window_conf() -> Conf {
    Conf {
        window_title: "Rusty Corks".to_string(),
        window_width: 1280,
        window_height: 720,
        window_resizable: false,
        ..Default::default()
    }
}