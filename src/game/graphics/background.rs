use macroquad::prelude::{draw_texture, FileError, load_texture, screen_width, Texture2D, Vec2, WHITE};

const ROAD_TEXTURE_PATH: &str = "assets/road.png";

#[derive(Clone)]
pub struct Background {
    pub texture: Texture2D,
    position: Vec2,
    pub speed: f32,
}

impl Background {
    pub async fn new() -> Result<Background, FileError> {
        let background_texture = load_texture(ROAD_TEXTURE_PATH).await?;
        Ok(Background {
            texture: background_texture,
            position: Vec2::new(0.0, 0.0),
            speed: 300.0,
        })
    }

    pub fn draw(&self) {
        draw_texture(self.texture, self.position.x, self.position.y, WHITE);
        draw_texture(self.texture, self.position.x + screen_width(), self.position.y, WHITE);
    }
    pub fn move_texture(&mut self, delta_time: f32) {
        // Update background position
        self.position.x -= self.speed * delta_time;
        // Create a continuous loop effect
        if self.position.x < -screen_width() {
            self.position.x = 0.0;
        }

        self.draw();
    }

    pub fn set_speed(&mut self, speed: f32) {
        self.speed = speed;
    }
}