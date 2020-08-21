use std::collections::HashMap;

use ggez::input::keyboard::KeyCode;

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::button::Button;

pub struct Keyboard {
    keys: HashMap<KeyboardKey, Button>
}

impl Keyboard {
    pub fn new() -> Keyboard {
        Keyboard{
            keys: {
                let mut key_hash_map: HashMap<KeyboardKey, Button> = HashMap::new();
                for i in KeyboardKey::iter() {
                    key_hash_map.insert(i, Button::new());
                }
                key_hash_map
            }
        }
    }

    pub fn key(&mut self, keyboard_key: KeyboardKey) -> &mut Button {
        self.keys.get_mut(&keyboard_key).unwrap()
    }

    pub fn update(&mut self) {
        for key in self.keys.values_mut() {
            key.update();
        }
    }
}

#[derive(EnumIter, PartialEq, Eq, Hash)]
pub enum KeyboardKey {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,

    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    F21,
    F22,
    F23,
    F24,

    Key0,
    Key1,
    Key2,
    Key3,
    Key4,
    Key5,
    Key6,
    Key7,
    Key8,
    Key9,

    Numpad0,
    Numpad1,
    Numpad2,
    Numpad3,
    Numpad4,
    Numpad5,
    Numpad6,
    Numpad7,
    Numpad8,
    Numpad9,

    Unknown
}

pub fn keyboard_key_from_ggez_key_code(key_code: KeyCode) -> KeyboardKey {
    match key_code {
        KeyCode::A => KeyboardKey::A,
        KeyCode::B => KeyboardKey::B,
        KeyCode::C => KeyboardKey::C,
        KeyCode::D => KeyboardKey::D,
        KeyCode::E => KeyboardKey::E,
        KeyCode::F => KeyboardKey::F,
        KeyCode::G => KeyboardKey::G,
        KeyCode::H => KeyboardKey::H,
        KeyCode::I => KeyboardKey::I,
        KeyCode::J => KeyboardKey::J,
        KeyCode::K => KeyboardKey::K,
        KeyCode::L => KeyboardKey::L,
        KeyCode::M => KeyboardKey::M,
        KeyCode::N => KeyboardKey::N,
        KeyCode::O => KeyboardKey::O,
        KeyCode::P => KeyboardKey::P,
        KeyCode::Q => KeyboardKey::Q,
        KeyCode::R => KeyboardKey::R,
        KeyCode::S => KeyboardKey::S,
        KeyCode::T => KeyboardKey::T,
        KeyCode::U => KeyboardKey::U,
        KeyCode::V => KeyboardKey::V,
        KeyCode::W => KeyboardKey::W,
        KeyCode::X => KeyboardKey::X,
        KeyCode::Y => KeyboardKey::Y,
        KeyCode::Z => KeyboardKey::Z,

        KeyCode::F1 => KeyboardKey::F1,
        KeyCode::F2 => KeyboardKey::F2,
        KeyCode::F3 => KeyboardKey::F3,
        KeyCode::F4 => KeyboardKey::F4,
        KeyCode::F5 => KeyboardKey::F5,
        KeyCode::F6 => KeyboardKey::F6,
        KeyCode::F7 => KeyboardKey::F7,
        KeyCode::F8 => KeyboardKey::F8,
        KeyCode::F9 => KeyboardKey::F9,
        KeyCode::F10 => KeyboardKey::F10,
        KeyCode::F11 => KeyboardKey::F11,
        KeyCode::F12 => KeyboardKey::F12,
        KeyCode::F13 => KeyboardKey::F13,
        KeyCode::F14 => KeyboardKey::F14,
        KeyCode::F15 => KeyboardKey::F15,
        KeyCode::F16 => KeyboardKey::F16,
        KeyCode::F17 => KeyboardKey::F17,
        KeyCode::F18 => KeyboardKey::F18,
        KeyCode::F19 => KeyboardKey::F19,
        KeyCode::F20 => KeyboardKey::F20,
        KeyCode::F21 => KeyboardKey::F21,
        KeyCode::F22 => KeyboardKey::F22,
        KeyCode::F23 => KeyboardKey::F23,
        KeyCode::F24 => KeyboardKey::F24,

        KeyCode::Key0 => KeyboardKey::Key0,
        KeyCode::Key1 => KeyboardKey::Key1,
        KeyCode::Key2 => KeyboardKey::Key2,
        KeyCode::Key3 => KeyboardKey::Key3,
        KeyCode::Key4 => KeyboardKey::Key4,
        KeyCode::Key5 => KeyboardKey::Key5,
        KeyCode::Key6 => KeyboardKey::Key6,
        KeyCode::Key7 => KeyboardKey::Key7,
        KeyCode::Key8 => KeyboardKey::Key8,
        KeyCode::Key9 => KeyboardKey::Key9,

        KeyCode::Numpad0 => KeyboardKey::Numpad0,
        KeyCode::Numpad1 => KeyboardKey::Numpad1,
        KeyCode::Numpad2 => KeyboardKey::Numpad2,
        KeyCode::Numpad3 => KeyboardKey::Numpad3,
        KeyCode::Numpad4 => KeyboardKey::Numpad4,
        KeyCode::Numpad5 => KeyboardKey::Numpad5,
        KeyCode::Numpad6 => KeyboardKey::Numpad6,
        KeyCode::Numpad7 => KeyboardKey::Numpad7,
        KeyCode::Numpad8 => KeyboardKey::Numpad8,
        KeyCode::Numpad9 => KeyboardKey::Numpad9,

        _ => KeyboardKey::Unknown

        //KeyCode::Escape => KeyboardKey::B,
        //KeyCode::Snapshot => KeyboardKey::B,
        //KeyCode::Scroll => KeyboardKey::B,
        //KeyCode::Pause => KeyboardKey::B,
        //KeyCode::Insert => KeyboardKey::B,
        //KeyCode::Home => KeyboardKey::B,
        //KeyCode::Delete => KeyboardKey::B,
        //KeyCode::End => KeyboardKey::B,
        //KeyCode::PageDown => KeyboardKey::B,
        //KeyCode::PageUp => KeyboardKey::B,
        //KeyCode::Left => KeyboardKey::B,
        //KeyCode::Up => KeyboardKey::B,
        //KeyCode::Right => KeyboardKey::B,
        //KeyCode::Down => KeyboardKey::B,
        //KeyCode::Back => KeyboardKey::B,
        //KeyCode::Return => KeyboardKey::B,
        //KeyCode::Space => KeyboardKey::B,
        //KeyCode::Compose => KeyboardKey::B,
        //KeyCode::Caret => KeyboardKey::B,
        //KeyCode::Numlock => KeyboardKey::B,
        //KeyCode::AbntC1 => KeyboardKey::B,
        //KeyCode::AbntC2 => KeyboardKey::B,
        //KeyCode::Add => KeyboardKey::B,
        //KeyCode::Apostrophe => KeyboardKey::B,
        //KeyCode::Apps => KeyboardKey::B,
        //KeyCode::At => KeyboardKey::B,
        //KeyCode::Ax => KeyboardKey::B,
        //KeyCode::Backslash => KeyboardKey::B,
        //KeyCode::Calculator => KeyboardKey::B,
        //KeyCode::Capital => KeyboardKey::B,
        //KeyCode::Colon => KeyboardKey::B,
        //KeyCode::Comma => KeyboardKey::B,
        //KeyCode::Convert => KeyboardKey::B,
        //KeyCode::Decimal => KeyboardKey::B,
        //KeyCode::Divide => KeyboardKey::B,
        //KeyCode::Equals => KeyboardKey::B,
        //KeyCode::Grave => KeyboardKey::B,
        //KeyCode::Kana => KeyboardKey::B,
        //KeyCode::Kanji => KeyboardKey::B,
        //KeyCode::LAlt => KeyboardKey::B,
        //KeyCode::LBracket => KeyboardKey::B,
        //KeyCode::LControl => KeyboardKey::B,
        //KeyCode::LShift => KeyboardKey::B,
        //KeyCode::LWin => KeyboardKey::B,
        //KeyCode::Mail => KeyboardKey::B,
        //KeyCode::MediaSelect => KeyboardKey::B,
        //KeyCode::MediaStop => KeyboardKey::B,
        //KeyCode::Minus => KeyboardKey::B,
        //KeyCode::Multiply => KeyboardKey::B,
        //KeyCode::Mute => KeyboardKey::B,
        //KeyCode::MyComputer => KeyboardKey::B,
        //KeyCode::NavigateForward => KeyboardKey::B,
        //KeyCode::NavigateBackward => KeyboardKey::B,
        //KeyCode::NextTrack => KeyboardKey::B,
        //KeyCode::NoConvert => KeyboardKey::B,
        //KeyCode::NumpadComma => KeyboardKey::B,
        //KeyCode::NumpadEnter => KeyboardKey::B,
        //KeyCode::NumpadEquals => KeyboardKey::B,
        //KeyCode::OEM102 => KeyboardKey::B,
        //KeyCode::Period => KeyboardKey::B,
        //KeyCode::PlayPause => KeyboardKey::B,
        //KeyCode::Power => KeyboardKey::B,
        //KeyCode::PrevTrack => KeyboardKey::B,
        //KeyCode::RAlt => KeyboardKey::B,
        //KeyCode::RBracket => KeyboardKey::B,
        //KeyCode::RControl => KeyboardKey::B,
        //KeyCode::RShift => KeyboardKey::B,
        //KeyCode::RWin => KeyboardKey::B,
        //KeyCode::Semicolon => KeyboardKey::B,
        //KeyCode::Slash => KeyboardKey::B,
        //KeyCode::Sleep => KeyboardKey::B,
        //KeyCode::Stop => KeyboardKey::B,
        //KeyCode::Subtract => KeyboardKey::B,
        //KeyCode::Sysrq => KeyboardKey::B,
        //KeyCode::Tab => KeyboardKey::B,
        //KeyCode::Underline => KeyboardKey::B,
        //KeyCode::Unlabeled => KeyboardKey::B,
        //KeyCode::VolumeDown => KeyboardKey::B,
        //KeyCode::VolumeUp => KeyboardKey::B,
        //KeyCode::Wake => KeyboardKey::B,
        //KeyCode::WebBack => KeyboardKey::B,
        //KeyCode::WebFavorites => KeyboardKey::B,
        //KeyCode::WebForward => KeyboardKey::B,
        //KeyCode::WebHome => KeyboardKey::B,
        //KeyCode::WebRefresh => KeyboardKey::B,
        //KeyCode::WebSearch => KeyboardKey::B,
        //KeyCode::WebStop => KeyboardKey::B,
        //KeyCode::Yen => KeyboardKey::B,
        //KeyCode::Copy => KeyboardKey::B,
        //KeyCode::Paste => KeyboardKey::B,
        //KeyCode::Cut => KeyboardKey::B
    }
}
