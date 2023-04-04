use rdev::{simulate, EventType, Key as RdevKey};

use super::{KeyboardBackend, MouseBackend};
use crate::peripherals::{KeyboardCommon, MouseButton};

pub struct General;
pub struct PhysicalQwertyLayout;

impl KeyboardBackend<KeyboardCommon> for PhysicalQwertyLayout {
    fn key_down(&self, key: KeyboardCommon) {
        let _ = simulate(&EventType::KeyPress(keyboard_to_rdev_key(key)));
    }

    fn key_up(&self, key: KeyboardCommon) {
        let _ = simulate(&EventType::KeyPress(keyboard_to_rdev_key(key)));
    }
}

impl MouseBackend for General {
    fn mouse_move_to(&self, x: i32, y: i32) {
        let _ = simulate(&EventType::MouseMove { x: x as f64, y: y as f64 })
    }

    fn mouse_move_by(&self, x: i32, y: i32) {
        let _ = simulate(&EventType:: { x: x as f64, y: y as f64 }
    }

    fn mouse_scroll(&self, x: i32, y: i32) {
        todo!()
    }

    fn mouse_down(&self, button: MouseButton) {
        todo!()
    }

    fn mouse_up(&self, button: MouseButton) {
        todo!()
    }
}

fn keyboard_to_rdev_key(key: KeyboardCommon) -> RdevKey {
    match key {
        KeyboardCommon::A => todo!(),
        KeyboardCommon::B => todo!(),
        KeyboardCommon::C => todo!(),
        KeyboardCommon::D => todo!(),
        KeyboardCommon::E => todo!(),
        KeyboardCommon::F => todo!(),
        KeyboardCommon::G => todo!(),
        KeyboardCommon::H => todo!(),
        KeyboardCommon::I => todo!(),
        KeyboardCommon::J => todo!(),
        KeyboardCommon::K => todo!(),
        KeyboardCommon::L => todo!(),
        KeyboardCommon::M => todo!(),
        KeyboardCommon::N => todo!(),
        KeyboardCommon::O => todo!(),
        KeyboardCommon::P => todo!(),
        KeyboardCommon::Q => todo!(),
        KeyboardCommon::R => todo!(),
        KeyboardCommon::S => todo!(),
        KeyboardCommon::T => todo!(),
        KeyboardCommon::U => todo!(),
        KeyboardCommon::V => todo!(),
        KeyboardCommon::W => todo!(),
        KeyboardCommon::X => todo!(),
        KeyboardCommon::Y => todo!(),
        KeyboardCommon::Z => todo!(),
        KeyboardCommon::Num0 => todo!(),
        KeyboardCommon::Num1 => todo!(),
        KeyboardCommon::Num2 => todo!(),
        KeyboardCommon::Num3 => todo!(),
        KeyboardCommon::Num4 => todo!(),
        KeyboardCommon::Num5 => todo!(),
        KeyboardCommon::Num6 => todo!(),
        KeyboardCommon::Num7 => todo!(),
        KeyboardCommon::Num8 => todo!(),
        KeyboardCommon::Num9 => todo!(),
        KeyboardCommon::Alt => todo!(),
        KeyboardCommon::Shift => todo!(),
        KeyboardCommon::RShift => todo!(),
        KeyboardCommon::LShift => todo!(),
        KeyboardCommon::Control => todo!(),
        KeyboardCommon::RControl => todo!(),
        KeyboardCommon::LControl => todo!(),
        KeyboardCommon::F1 => todo!(),
        KeyboardCommon::F2 => todo!(),
        KeyboardCommon::F3 => todo!(),
        KeyboardCommon::F4 => todo!(),
        KeyboardCommon::F5 => todo!(),
        KeyboardCommon::F6 => todo!(),
        KeyboardCommon::F7 => todo!(),
        KeyboardCommon::F8 => todo!(),
        KeyboardCommon::F9 => todo!(),
        KeyboardCommon::F10 => todo!(),
        KeyboardCommon::F11 => todo!(),
        KeyboardCommon::F12 => todo!(),
        KeyboardCommon::CapsLock => todo!(),
        KeyboardCommon::End => todo!(),
        KeyboardCommon::Home => todo!(),
        KeyboardCommon::PageUp => todo!(),
        KeyboardCommon::PageDown => todo!(),
        KeyboardCommon::Escape => todo!(),
        KeyboardCommon::Return => todo!(),
        KeyboardCommon::Space => todo!(),
        KeyboardCommon::Tab => todo!(),
        KeyboardCommon::Backspace => todo!(),
        KeyboardCommon::Delete => todo!(),
        KeyboardCommon::UpArrow => todo!(),
        KeyboardCommon::DownArrow => todo!(),
        KeyboardCommon::LeftArrow => todo!(),
        KeyboardCommon::RightArrow => todo!(),
    }
}
