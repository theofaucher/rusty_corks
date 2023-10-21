use macroquad::input::KeyCode;

use crate::config::KEY_GAME;

pub mod key_game;
pub mod keyboard_observer;

#[derive(PartialEq, Copy, Clone)]
pub enum GameAction {
    Up,
    Down,
    Mute,
    PauseResume,
    Quit,
    None,
}

pub const GAME_ACTION: [(GameAction, &str); 5] = [
    (GameAction::Up, "Go Up: "),
    (GameAction::Down, "Go Down: "),
    (GameAction::Mute, "Mute: "),
    (GameAction::PauseResume, "Pause and resume: "),
    (GameAction::Quit, "Quit: ")
];

const KEY_CODE_WITH_STRING: [(KeyCode, &str); 121] = [
    (KeyCode::Space, "Space"),
    (KeyCode::Apostrophe, "Apostrophe"),
    (KeyCode::Comma, "Comma"),
    (KeyCode::Minus, "Minus"),
    (KeyCode::Period, "Period"),
    (KeyCode::Slash, "Slash"),
    (KeyCode::Key0, "Key0"),
    (KeyCode::Key1, "Key1"),
    (KeyCode::Key2, "Key2"),
    (KeyCode::Key3, "Key3"),
    (KeyCode::Key4, "Key4"),
    (KeyCode::Key5, "Key5"),
    (KeyCode::Key6, "Key6"),
    (KeyCode::Key7, "Key7"),
    (KeyCode::Key8, "Key8"),
    (KeyCode::Key9, "Key9"),
    (KeyCode::Semicolon, "Semicolon"),
    (KeyCode::Equal, "Equal"),
    (KeyCode::A, "A"),
    (KeyCode::B, "B"),
    (KeyCode::C, "C"),
    (KeyCode::D, "D"),
    (KeyCode::E, "E"),
    (KeyCode::F, "F"),
    (KeyCode::G, "G"),
    (KeyCode::H, "H"),
    (KeyCode::I, "I"),
    (KeyCode::J, "J"),
    (KeyCode::K, "K"),
    (KeyCode::L, "L"),
    (KeyCode::M, "M"),
    (KeyCode::N, "N"),
    (KeyCode::O, "O"),
    (KeyCode::P, "P"),
    (KeyCode::Q, "Q"),
    (KeyCode::R, "R"),
    (KeyCode::S, "S"),
    (KeyCode::T, "T"),
    (KeyCode::U, "U"),
    (KeyCode::V, "V"),
    (KeyCode::W, "W"),
    (KeyCode::X, "X"),
    (KeyCode::Y, "Y"),
    (KeyCode::Z, "Z"),
    (KeyCode::LeftBracket, "LeftBracket"),
    (KeyCode::Backslash, "Backslash"),
    (KeyCode::RightBracket, "RightBracket"),
    (KeyCode::GraveAccent, "GraveAccent"),
    (KeyCode::World1, "World1"),
    (KeyCode::World2, "World2"),
    (KeyCode::Escape, "Escape"),
    (KeyCode::Enter, "Enter"),
    (KeyCode::Tab, "Tab"),
    (KeyCode::Backspace, "Backspace"),
    (KeyCode::Insert, "Insert"),
    (KeyCode::Delete, "Delete"),
    (KeyCode::Right, "Right"),
    (KeyCode::Left, "Left"),
    (KeyCode::Down, "Down"),
    (KeyCode::Up, "Up"),
    (KeyCode::PageUp, "PageUp"),
    (KeyCode::PageDown, "PageDown"),
    (KeyCode::Home, "Home"),
    (KeyCode::End, "End"),
    (KeyCode::CapsLock, "CapsLock"),
    (KeyCode::ScrollLock, "ScrollLock"),
    (KeyCode::NumLock, "NumLock"),
    (KeyCode::PrintScreen, "PrintScreen"),
    (KeyCode::Pause, "Pause"),
    (KeyCode::F1, "F1"),
    (KeyCode::F2, "F2"),
    (KeyCode::F3, "F3"),
    (KeyCode::F4, "F4"),
    (KeyCode::F5, "F5"),
    (KeyCode::F6, "F6"),
    (KeyCode::F7, "F7"),
    (KeyCode::F8, "F8"),
    (KeyCode::F9, "F9"),
    (KeyCode::F10, "F10"),
    (KeyCode::F11, "F11"),
    (KeyCode::F12, "F12"),
    (KeyCode::F13, "F13"),
    (KeyCode::F14, "F14"),
    (KeyCode::F15, "F15"),
    (KeyCode::F16, "F16"),
    (KeyCode::F17, "F17"),
    (KeyCode::F18, "F18"),
    (KeyCode::F19, "F19"),
    (KeyCode::F20, "F20"),
    (KeyCode::F21, "F21"),
    (KeyCode::F22, "F22"),
    (KeyCode::F23, "F23"),
    (KeyCode::F24, "F24"),
    (KeyCode::F25, "F25"),
    (KeyCode::Kp0, "Kp0"),
    (KeyCode::Kp1, "Kp1"),
    (KeyCode::Kp2, "Kp2"),
    (KeyCode::Kp3, "Kp3"),
    (KeyCode::Kp4, "Kp4"),
    (KeyCode::Kp5, "Kp5"),
    (KeyCode::Kp6, "Kp6"),
    (KeyCode::Kp7, "Kp7"),
    (KeyCode::Kp8, "Kp8"),
    (KeyCode::Kp9, "Kp9"),
    (KeyCode::KpDecimal, "KpDecimal"),
    (KeyCode::KpDivide, "KpDivide"),
    (KeyCode::KpMultiply, "KpMultiply"),
    (KeyCode::KpSubtract, "KpSubtract"),
    (KeyCode::KpAdd, "KpAdd"),
    (KeyCode::KpEnter, "KpEnter"),
    (KeyCode::KpEqual, "KpEqual"),
    (KeyCode::LeftShift, "LeftShift"),
    (KeyCode::LeftControl, "LeftControl"),
    (KeyCode::LeftAlt, "LeftAlt"),
    (KeyCode::LeftSuper, "LeftSuper"),
    (KeyCode::RightShift, "RightShift"),
    (KeyCode::RightControl, "RightControl"),
    (KeyCode::RightAlt, "RightAlt"),
    (KeyCode::RightSuper, "RightSuper"),
    (KeyCode::Menu, "Menu"),
    (KeyCode::Unknown, "Unknown"),
];

pub fn get_action_description_from_game_action(game_action: GameAction) -> Option<String> {
    let mut return_value: Option<String> = None;
    for &(code, description) in &GAME_ACTION {
        if code == game_action {
            return_value = Some(description.to_string());
        }
    }
    return return_value;
}

fn get_game_action_from_key_code(key_code: KeyCode) -> Option<GameAction> {
    let mut return_value: Option<GameAction> = None;
    for &(code, action) in &KEY_GAME {
        if code == key_code {
            return_value = Some(action);
        }
    }
    return return_value;
}

pub fn get_key_code_from_game_action(game_action: GameAction) -> Option<KeyCode> {
    let mut return_value: Option<KeyCode> = None;
    for &(code, action) in &KEY_GAME {
        if action == game_action {
            return_value = Some(code);
        }
    }
    return return_value;
}

pub fn get_str_from_key_code(key_code: KeyCode) -> String {
    let mut result: String = "".to_string();
    for (key, value) in KEY_CODE_WITH_STRING.iter() {
        if *key == key_code {
            result = value.to_string();
        }
    };
    result
}