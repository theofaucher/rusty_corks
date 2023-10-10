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

pub fn window_conf() -> Conf {
    Conf {
        window_title: "Rusty Corks".to_string(),
        window_width: 1280,
        window_height: 720,
        window_resizable: false,
        ..Default::default()
    }
}