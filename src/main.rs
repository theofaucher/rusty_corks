mod game;

use macroquad::prelude::*;
use crate::keyboard_observer::KeyboardObserver;

mod keyboard_observer;

use game::car::*;

struct Background {
    texture: Texture2D,
    position: Vec2,
    speed: f32,
}

#[macroquad::main(window_conf())]
async fn main() {
    let background_texture: Result<Texture2D, FileError> = load_texture("assets/road.png").await;
    let background_texture = match background_texture {
        Ok(background_texture) => background_texture,
        Err(e) => {
            println!("Error loading texture: {}", e);
            return;
        }
    };
    let mut background = Background {
        texture: background_texture,
        position: Vec2::new(0.0, 0.0),
        speed: 300.0, // Réglez la vitesse de déplacement selon vos préférences
    };

    let player_car_texture: Result<Texture2D, FileError> = load_texture("assets/playerCar.png").await;
    let player_car_texture = match player_car_texture {
        Ok(player_car_texture) => player_car_texture,
        Err(e) => {
            println!("Error loading texture: {}", e);
            return;
        }
    };

    let black_car_texture: Result<Texture2D, FileError> = load_texture("assets/blackCar.png").await;
    let bot_car_texture = match black_car_texture {
        Ok(black_car_texture) => black_car_texture,
        Err(e) => {
            println!("Error loading texture: {}", e);
            return;
        }
    };

    let player_car = PlayerCar::new(player_car_texture);
    let mut red_car = BotCar::new(bot_car_texture, Way::Upper, background.speed + 100.0);

    let (sender, _receiver) = std::sync::mpsc::channel::<macroquad::input::KeyCode>();
    let observer = KeyboardObserver::new(sender);
    observer.start_observer();

    loop {
        draw_text("RustyCorks", 20.0, 20.0, 30.0, DARKGRAY);

        let delta_time = get_frame_time();

        // Mettez à jour la position du fond
        background.position.x -= background.speed * delta_time;

        // Mettez à jour la position de la voiture rouge
        red_car.update_position(delta_time);

        // Créez un effet de boucle continue
        if background.position.x < -screen_width() {
            background.position.x = 0.0;
        }

        // Dessinez le fond
        clear_background(BLACK);
        draw_texture(background.texture, background.position.x, background.position.y, WHITE);
        draw_texture(background.texture, background.position.x + screen_width(), background.position.y, WHITE);

        if red_car.x_position > -screen_width() {
            red_car.draw();
        }

        player_car.draw();

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