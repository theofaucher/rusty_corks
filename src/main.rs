
use macroquad::prelude::*;
use crate::game::car::{BotCar, Car, PlayerCar, Way};
use crate::keyboard_observer::KeyboardObserver;

mod keyboard_observer;
mod game;

struct Background {
    texture: Texture2D,
    position: Vec2,
    speed: f32,
}

#[macroquad::main(window_conf())]
async fn main() {

    let graphics_manager = game::graphics_manager::GraphicsManager::new().await;
    let mut game_manager = match graphics_manager {
        Some(game_manager) => game_manager,
        None => return,
    };


    let player_car = PlayerCar::new().await;
    let player_car = match player_car {
        Some(player_car) => player_car,
        None => return,
    };

    let red_car = BotCar::new( Way::Upper, game_manager.background.speed + 100.0).await;
    let mut red_car = match red_car {
        Some(red_car) => red_car,
        None => return,
    };

    let (sender, _receiver) = std::sync::mpsc::channel::<macroquad::input::KeyCode>();
    let observer = KeyboardObserver::new(sender);
    observer.start_observer();

    loop {
        let delta_time = get_frame_time();

        game_manager.background.update_position(delta_time);

        // Mettez à jour la position de la voiture rouge
        red_car.update_position(delta_time);

        if red_car.x_position > -screen_width() {
            game_manager.draw_bot_car(&red_car);
        }

        game_manager.draw_player_car(&player_car);

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