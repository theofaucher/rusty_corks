use macroquad::prelude::{FileError, load_texture, Texture2D};

use crate::game::car::{Car, Way};

const PLAYER_CAR_PATH: &str = "assets/cars/playerCar.png";
#[derive(Clone)]
pub struct PlayerCar {
    texture: Texture2D,
    way: Way,
}

impl PlayerCar {
    pub async fn new() -> Result<PlayerCar, FileError> {
        let background_texture = load_texture(PLAYER_CAR_PATH).await?;
        Ok(PlayerCar {
            texture: background_texture,
            way: Way::Center,
        })
    }

    pub fn set_way(&mut self, way: Way) {
        self.way = way;
    }
}

impl Car for PlayerCar {
    fn get_texture(&self) -> Texture2D {
        self.texture
    }
    fn get_way(&self) -> Way {
        self.way
    }
}
