use macroquad::prelude::*;

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

    loop {
        // Draw the background image
        draw_texture(
            background_texture,
            0.0,
            0.0,
            WHITE, // You can specify tint color if needed
        );

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