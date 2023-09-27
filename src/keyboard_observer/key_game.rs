use macroquad::input::KeyCode;

#[derive(Copy, Clone)] // Ajoutez cette dérivation pour le trait Copy
pub struct KeyGame {
    pub key: KeyCode,
    pressed: bool,
    down: bool,
}

impl KeyGame {
    pub fn new(key: KeyCode) -> Self {
        Self {
            key,
            pressed: false,
            down: false,
        }
    }

    pub fn update(&mut self, is_down: bool) {
        self.pressed = is_down && !self.down;
        self.down = is_down;
    }

    pub fn is_key_pressed(&self) -> bool {
        self.pressed
    }
}