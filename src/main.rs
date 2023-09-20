use macroquad::prelude::*;

#[macroquad::main(window_conf())]
async fn main() {
    loop {
        clear_background(RED);

        draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);

        draw_text("RustyCorks", 20.0, 20.0, 30.0, DARKGRAY);

        next_frame().await
    }
}

pub fn window_conf() -> Conf {
    Conf {
        window_title: "RustyCorks".to_string(),
        window_width: 1024,
        window_height: 512,
        window_resizable: false,
        ..Default::default()
    }
}