use macroquad::prelude::{draw_texture, FileError, load_texture, screen_width, Texture2D, Vec2, WHITE};

#[derive(Clone)]
pub struct Background {
    pub texture: Texture2D,
    position: Vec2,
    pub speed: f32,
}

impl Background {
    pub async fn new() -> Result<Background, FileError> {
        let background_texture = load_texture("assets/road.png").await?;
        Ok(Background {
            texture: background_texture,
            position: Vec2::new(0.0, 0.0),
            speed: 300.0, // Réglez la vitesse de déplacement selon vos préférences
        })
    }

    pub fn move_texture(&mut self, delta_time: f32) {
        // Mettez à jour la position du fond
        self.position.x -= self.speed * delta_time;
        // Créez un effet de boucle continue
        if self.position.x < -screen_width() {
            self.position.x = 0.0;
        }
        draw_texture(self.texture, self.position.x, self.position.y, WHITE);
        draw_texture(self.texture, self.position.x + screen_width(), self.position.y, WHITE);
    }
}