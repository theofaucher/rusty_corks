use std::sync::mpsc;

use macroquad::prelude::*;

use crate::game::game::Game;
use crate::keyboard::GameAction;
use crate::keyboard::keyboard_observer::KeyboardObserver;
use crate::utils::rusty_error::RustyResult;

mod game;
mod keyboard;
mod utils;
mod config;

// Window settings
pub const WINDOW_WIDTH: f32 = 1280.0;
pub const WINDOW_HEIGHT: f32 = 720.0;
pub const GAME_NAME: &str = "Rusty Corks";

#[macroquad::main(window_conf())]
async fn main() -> RustyResult<()> {
    let mut quit_game = false;
    let (sender, receiver) = mpsc::channel::<GameAction>();
    let mut observer = KeyboardObserver::new(sender);

    observer.start_observer();
    let mut game = Game::new(receiver).await?;

    while !quit_game {
        quit_game = game.run().await?;
        next_frame().await;
    }

    observer.stop_observer();
    Ok(())
}

pub fn window_conf() -> Conf {
    Conf {
        window_title: GAME_NAME.to_string(),
        window_width: WINDOW_WIDTH as i32,
        window_height: WINDOW_HEIGHT as i32,
        window_resizable: false,
        ..Default::default()
    }
}