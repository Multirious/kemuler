use crate::peripherals::{KeyCommon, KeyLayout, MouseButton};
use enigo::{
    Enigo as OriginalEnigo, Key as EnigoKey, KeyboardControllable, MouseButton as EnigoMouseButton,
    MouseControllable,
};

use super::{KeyboardBackend, MouseBackend};

pub struct General;

impl MouseBackend for General {
    fn mouse_move_to(&self, x: i32, y: i32) {
        MouseControllable::mouse_move_to(&mut OriginalEnigo, x, y);
    }

    fn mouse_move_by(&self, x: i32, y: i32) {
        MouseControllable::mouse_move_relative(&mut OriginalEnigo, x, y);
    }

    fn mouse_scroll(&self, x: i32, y: i32) {
        MouseControllable::mouse_scroll_x(&mut OriginalEnigo, x);
        MouseControllable::mouse_scroll_y(&mut OriginalEnigo, y);
    }

    fn mouse_down(&self, button: MouseButton) {
        MouseControllable::mouse_down(&mut OriginalEnigo, mouse_button_as_enigo(button));
    }

    fn mouse_up(&self, button: MouseButton) {
        MouseControllable::mouse_up(&mut OriginalEnigo, mouse_button_as_enigo(button));
    }
}

impl KeyboardBackend<KeyCommon> for General {
    fn key_down(&self, key: KeyCommon) {
        KeyboardControllable::key_down(&mut OriginalEnigo, keyboard_common_as_enigo(key));
    }

    fn key_up(&self, key: KeyCommon) {
        KeyboardControllable::key_up(&mut OriginalEnigo, keyboard_common_as_enigo(key));
    }
}

impl KeyboardBackend<KeyLayout> for General {
    fn key_down(&self, key: KeyLayout) {
        KeyboardControllable::key_down(&mut OriginalEnigo, keyboard_layout_as_enigo(key));
    }

    fn key_up(&self, key: KeyLayout) {
        KeyboardControllable::key_up(&mut OriginalEnigo, keyboard_layout_as_enigo(key));
    }
}

fn keyboard_layout_as_enigo(keyboard: KeyLayout) -> EnigoKey {
    EnigoKey::Layout(keyboard.0)
}

fn keyboard_common_as_enigo(keyboard: KeyCommon) -> EnigoKey {
    match keyboard {
        KeyCommon::A => EnigoKey::A,
        KeyCommon::B => EnigoKey::B,
        KeyCommon::C => EnigoKey::C,
        KeyCommon::D => EnigoKey::D,
        KeyCommon::E => EnigoKey::E,
        KeyCommon::F => EnigoKey::F,
        KeyCommon::G => EnigoKey::G,
        KeyCommon::H => EnigoKey::H,
        KeyCommon::I => EnigoKey::I,
        KeyCommon::J => EnigoKey::J,
        KeyCommon::K => EnigoKey::K,
        KeyCommon::L => EnigoKey::L,
        KeyCommon::M => EnigoKey::M,
        KeyCommon::N => EnigoKey::N,
        KeyCommon::O => EnigoKey::O,
        KeyCommon::P => EnigoKey::P,
        KeyCommon::Q => EnigoKey::Q,
        KeyCommon::R => EnigoKey::R,
        KeyCommon::S => EnigoKey::S,
        KeyCommon::T => EnigoKey::T,
        KeyCommon::U => EnigoKey::U,
        KeyCommon::V => EnigoKey::V,
        KeyCommon::W => EnigoKey::W,
        KeyCommon::X => EnigoKey::X,
        KeyCommon::Y => EnigoKey::Y,
        KeyCommon::Z => EnigoKey::Z,
        KeyCommon::Num0 => EnigoKey::Num0,
        KeyCommon::Num1 => EnigoKey::Num1,
        KeyCommon::Num2 => EnigoKey::Num2,
        KeyCommon::Num3 => EnigoKey::Num3,
        KeyCommon::Num4 => EnigoKey::Num4,
        KeyCommon::Num5 => EnigoKey::Num5,
        KeyCommon::Num6 => EnigoKey::Num6,
        KeyCommon::Num7 => EnigoKey::Num7,
        KeyCommon::Num8 => EnigoKey::Num8,
        KeyCommon::Num9 => EnigoKey::Num9,
        KeyCommon::Alt => EnigoKey::Alt,
        KeyCommon::LAlt => EnigoKey::Alt,
        KeyCommon::RAlt => EnigoKey::Alt,
        KeyCommon::Shift => EnigoKey::Shift,
        KeyCommon::RShift => EnigoKey::RShift,
        KeyCommon::LShift => EnigoKey::LShift,
        KeyCommon::Control => EnigoKey::Control,
        KeyCommon::RControl => EnigoKey::RControl,
        KeyCommon::LControl => EnigoKey::LControl,
        KeyCommon::F1 => EnigoKey::F1,
        KeyCommon::F2 => EnigoKey::F2,
        KeyCommon::F3 => EnigoKey::F3,
        KeyCommon::F4 => EnigoKey::F4,
        KeyCommon::F5 => EnigoKey::F5,
        KeyCommon::F6 => EnigoKey::F6,
        KeyCommon::F7 => EnigoKey::F7,
        KeyCommon::F8 => EnigoKey::F8,
        KeyCommon::F9 => EnigoKey::F9,
        KeyCommon::F10 => EnigoKey::F10,
        KeyCommon::F11 => EnigoKey::F11,
        KeyCommon::F12 => EnigoKey::F12,
        KeyCommon::CapsLock => EnigoKey::CapsLock,
        KeyCommon::End => EnigoKey::End,
        KeyCommon::Home => EnigoKey::Home,
        KeyCommon::PageUp => EnigoKey::PageUp,
        KeyCommon::PageDown => EnigoKey::PageDown,
        KeyCommon::Escape => EnigoKey::Escape,
        KeyCommon::Return => EnigoKey::Return,
        KeyCommon::Space => EnigoKey::Space,
        KeyCommon::Tab => EnigoKey::Tab,
        KeyCommon::Backspace => EnigoKey::Backspace,
        KeyCommon::Delete => EnigoKey::Delete,
        KeyCommon::UpArrow => EnigoKey::UpArrow,
        KeyCommon::DownArrow => EnigoKey::DownArrow,
        KeyCommon::LeftArrow => EnigoKey::LeftArrow,
        KeyCommon::RightArrow => EnigoKey::RightArrow,
    }
}

fn mouse_button_as_enigo(mouse_button: MouseButton) -> EnigoMouseButton {
    match mouse_button {
        MouseButton::Left => EnigoMouseButton::Left,
        MouseButton::Middle => EnigoMouseButton::Middle,
        MouseButton::Right => EnigoMouseButton::Right,
        MouseButton::X1 => EnigoMouseButton::Back,
        MouseButton::X2 => EnigoMouseButton::Forward,
    }
}
