use macroquad::prelude::*;

#[macroquad::main(window_conf())]
async fn main() {
    // Load the image to be used as a background
    let background_texture = load_texture("assets/road.png").await.expect("Failed to load image");

    loop {
        clear_background(BLACK);

        draw_text("RustyCorks", 20.0, 20.0, 30.0, DARKGRAY);

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
        window_title: "RustyCorks".to_string(),
        window_width: 1280 as i32,
        window_height: 720,
        window_resizable: false,
        ..Default::default()
    }
}