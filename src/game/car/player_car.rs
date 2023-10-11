use macroquad::prelude::{FileError, load_texture, Texture2D};

use crate::game::car::{Car, Way};

#[derive(Clone)]
pub struct PlayerCar {
    pub texture: Texture2D,
    pub way: Way,
}

impl PlayerCar {
    pub async fn new() -> Result<PlayerCar, FileError> {
        let background_texture = load_texture("assets/playerCar.png").await?;
        Ok(PlayerCar {
            texture: background_texture,
            way: Way::Center,
        })
    }
}
