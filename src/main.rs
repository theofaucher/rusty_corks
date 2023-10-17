use std::sync::mpsc;

use macroquad::prelude::*;

use crate::config::{GAME_NAME, WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::game::game::Game;
use crate::keyboard::keyboard_observer::KeyboardObserver;
use crate::utils::rusty_error::RustyResult;

mod game;
mod keyboard;
mod utils;
mod config;

#[macroquad::main(window_conf())]
async fn main() -> RustyResult<()> {
    let mut quit_game = false;
    let (sender, receiver) = mpsc::channel::<KeyCode>();
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
        window_width: WINDOW_WIDTH,
        window_height: WINDOW_HEIGHT,
        window_resizable: false,
        ..Default::default()
    }
}