use std::sync::mpsc;

use macroquad::prelude::*;

use crate::game::game::Game;
use crate::keyboard::keyboard_observer::KeyboardObserver;
use crate::utils::rusty_error::RustyResult;

mod game;
mod keyboard;
mod utils;

#[macroquad::main(window_conf())]
async fn main() -> RustyResult<()> {
    let (sender, receiver) = mpsc::channel::<KeyCode>();
    let observer = KeyboardObserver::new(sender);
    observer.start_observer();

    let mut game = Game::new(receiver).await?;

    loop {
        game.run().await?;
        next_frame().await;
    }
}

// loop {
//     let delta_time = get_frame_time();
//
//     graphics_manager.background.update_position(delta_time);
//
//     match game.player_car.lock() {
//         Ok(player_car) => graphics_manager.draw_player_car(player_car),
//         Err(e) => {
//             println!("Error lock player car: {}", e);
//             break;
//         }
//     };
//
//     match game.score.lock() {
//         Ok(current_score) => graphics_manager.draw_score(*current_score),
//         Err(e) => {
//             println!("Error lock current score: {}", e);
//             break;
//         }
//     };
//
//     next_frame().await;
// }


pub fn window_conf() -> Conf {
    Conf {
        window_title: "Rusty Corks".to_string(),
        window_width: 1280,
        window_height: 720,
        window_resizable: false,
        ..Default::default()
    }
}