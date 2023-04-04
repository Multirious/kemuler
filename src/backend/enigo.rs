use crate::peripherals::{KeyboardCommon, KeyboardLayout, MouseButton};
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

impl KeyboardBackend<KeyboardCommon> for General {
    fn key_down(&self, key: KeyboardCommon) {
        KeyboardControllable::key_down(&mut OriginalEnigo, keyboard_common_as_enigo(key));
    }

    fn key_up(&self, key: KeyboardCommon) {
        KeyboardControllable::key_up(&mut OriginalEnigo, keyboard_common_as_enigo(key));
    }
}

impl KeyboardBackend<KeyboardLayout> for General {
    fn key_down(&self, key: KeyboardLayout) {
        KeyboardControllable::key_down(&mut OriginalEnigo, keyboard_layout_as_enigo(key));
    }

    fn key_up(&self, key: KeyboardLayout) {
        KeyboardControllable::key_up(&mut OriginalEnigo, keyboard_layout_as_enigo(key));
    }
}

fn keyboard_layout_as_enigo(keyboard: KeyboardLayout) -> EnigoKey {
    EnigoKey::Layout(keyboard.0)
}

fn keyboard_common_as_enigo(keyboard: KeyboardCommon) -> EnigoKey {
    match keyboard {
        KeyboardCommon::A => EnigoKey::A,
        KeyboardCommon::B => EnigoKey::B,
        KeyboardCommon::C => EnigoKey::C,
        KeyboardCommon::D => EnigoKey::D,
        KeyboardCommon::E => EnigoKey::E,
        KeyboardCommon::F => EnigoKey::F,
        KeyboardCommon::G => EnigoKey::G,
        KeyboardCommon::H => EnigoKey::H,
        KeyboardCommon::I => EnigoKey::I,
        KeyboardCommon::J => EnigoKey::J,
        KeyboardCommon::K => EnigoKey::K,
        KeyboardCommon::L => EnigoKey::L,
        KeyboardCommon::M => EnigoKey::M,
        KeyboardCommon::N => EnigoKey::N,
        KeyboardCommon::O => EnigoKey::O,
        KeyboardCommon::P => EnigoKey::P,
        KeyboardCommon::Q => EnigoKey::Q,
        KeyboardCommon::R => EnigoKey::R,
        KeyboardCommon::S => EnigoKey::S,
        KeyboardCommon::T => EnigoKey::T,
        KeyboardCommon::U => EnigoKey::U,
        KeyboardCommon::V => EnigoKey::V,
        KeyboardCommon::W => EnigoKey::W,
        KeyboardCommon::X => EnigoKey::X,
        KeyboardCommon::Y => EnigoKey::Y,
        KeyboardCommon::Z => EnigoKey::Z,
        KeyboardCommon::Num0 => EnigoKey::Num0,
        KeyboardCommon::Num1 => EnigoKey::Num1,
        KeyboardCommon::Num2 => EnigoKey::Num2,
        KeyboardCommon::Num3 => EnigoKey::Num3,
        KeyboardCommon::Num4 => EnigoKey::Num4,
        KeyboardCommon::Num5 => EnigoKey::Num5,
        KeyboardCommon::Num6 => EnigoKey::Num6,
        KeyboardCommon::Num7 => EnigoKey::Num7,
        KeyboardCommon::Num8 => EnigoKey::Num8,
        KeyboardCommon::Num9 => EnigoKey::Num9,
        KeyboardCommon::Alt => EnigoKey::Alt,
        KeyboardCommon::Shift => EnigoKey::Shift,
        KeyboardCommon::RShift => EnigoKey::RShift,
        KeyboardCommon::LShift => EnigoKey::LShift,
        KeyboardCommon::Control => EnigoKey::Control,
        KeyboardCommon::RControl => EnigoKey::RControl,
        KeyboardCommon::LControl => EnigoKey::LControl,
        KeyboardCommon::F1 => EnigoKey::F1,
        KeyboardCommon::F2 => EnigoKey::F2,
        KeyboardCommon::F3 => EnigoKey::F3,
        KeyboardCommon::F4 => EnigoKey::F4,
        KeyboardCommon::F5 => EnigoKey::F5,
        KeyboardCommon::F6 => EnigoKey::F6,
        KeyboardCommon::F7 => EnigoKey::F7,
        KeyboardCommon::F8 => EnigoKey::F8,
        KeyboardCommon::F9 => EnigoKey::F9,
        KeyboardCommon::F10 => EnigoKey::F10,
        KeyboardCommon::F11 => EnigoKey::F11,
        KeyboardCommon::F12 => EnigoKey::F12,
        KeyboardCommon::CapsLock => EnigoKey::CapsLock,
        KeyboardCommon::End => EnigoKey::End,
        KeyboardCommon::Home => EnigoKey::Home,
        KeyboardCommon::PageUp => EnigoKey::PageUp,
        KeyboardCommon::PageDown => EnigoKey::PageDown,
        KeyboardCommon::Escape => EnigoKey::Escape,
        KeyboardCommon::Return => EnigoKey::Return,
        KeyboardCommon::Space => EnigoKey::Space,
        KeyboardCommon::Tab => EnigoKey::Tab,
        KeyboardCommon::Backspace => EnigoKey::Backspace,
        KeyboardCommon::Delete => EnigoKey::Delete,
        KeyboardCommon::UpArrow => EnigoKey::UpArrow,
        KeyboardCommon::DownArrow => EnigoKey::DownArrow,
        KeyboardCommon::LeftArrow => EnigoKey::LeftArrow,
        KeyboardCommon::RightArrow => EnigoKey::RightArrow,
    }
}

fn mouse_button_as_enigo(mouse_button: MouseButton) -> EnigoMouseButton {
    match mouse_button {
        MouseButton::Left => EnigoMouseButton::Left,
        MouseButton::Middle => EnigoMouseButton::Middle,
        MouseButton::Right => EnigoMouseButton::Right,
        MouseButton::Back => EnigoMouseButton::Back,
        MouseButton::Forward => EnigoMouseButton::Forward,
    }
}
