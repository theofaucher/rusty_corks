use macroquad::prelude::*;

#[derive(Debug)]
struct Background {
    texture: Texture2D,
    position: Vec2,
    speed: f32,
}

#[macroquad::main(window_conf())]
async fn main() {
    // Load the image to be used as a background
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
        speed: 500.0, // Réglez la vitesse de déplacement selon vos préférences
    };

    loop {
        draw_text("RustyCorks", 20.0, 20.0, 30.0, DARKGRAY);

        let delta_time = get_frame_time();

        // Mettez à jour la position du fond
        background.position.x -= background.speed * delta_time;

        // Créez un effet de boucle continue
        if background.position.x < -screen_width() {
            background.position.x = 0.0;
        }

        // Dessinez le fond
        clear_background(BLACK);
        draw_texture(background.texture, background.position.x, background.position.y, WHITE);
        draw_texture(background.texture, background.position.x + screen_width(), background.position.y, WHITE);

        next_frame().await
    }
}

pub fn window_conf() -> Conf {
    Conf {
        window_title: "Rusty Corks".to_string(),
        window_width: 1280 as i32,
        window_height: 720,
        window_resizable: false,
        ..Default::default()
    }
}