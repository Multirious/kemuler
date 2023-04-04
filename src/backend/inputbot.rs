use super::*;
use ::inputbot;
use inputbot::{KeybdKey, MouseButton as InputbotMouseButton};

pub struct General;

impl KeyboardBackend<KeyCommon> for General {
    fn key_down(&self, key: KeyCommon) {
        common_keyboard_as_inputbot(key).press()
    }

    fn key_up(&self, key: KeyCommon) {
        common_keyboard_as_inputbot(key).release()
    }
}

impl MouseBackend for General {
    fn mouse_move_to(&self, x: i32, y: i32) {
        inputbot::MouseCursor::move_abs(x, y);
    }

    fn mouse_move_by(&self, x: i32, y: i32) {
        inputbot::MouseCursor::move_rel(x, y);
    }

    fn mouse_scroll(&self, x: i32, y: i32) {
        inputbot::MouseWheel::scroll_ver(y);
        inputbot::MouseWheel::scroll_ver(x);
    }

    fn mouse_down(&self, button: MouseButton) {
        mouse_button_as_inputbot(button).press();
    }

    fn mouse_up(&self, button: MouseButton) {
        mouse_button_as_inputbot(button).release();
    }
}

fn mouse_button_as_inputbot(b: MouseButton) -> InputbotMouseButton {
    match b {
        MouseButton::Left => InputbotMouseButton::LeftButton,
        MouseButton::Middle => InputbotMouseButton::MiddleButton,
        MouseButton::Right => InputbotMouseButton::RightButton,
        MouseButton::X1 => InputbotMouseButton::X1Button,
        MouseButton::X2 => InputbotMouseButton::X2Button,
    }
}

fn common_keyboard_as_inputbot(k: KeyCommon) -> KeybdKey {
    match k {
        KeyCommon::A => KeybdKey::AKey,
        KeyCommon::B => KeybdKey::BKey,
        KeyCommon::C => KeybdKey::CKey,
        KeyCommon::D => KeybdKey::DKey,
        KeyCommon::E => KeybdKey::EKey,
        KeyCommon::F => KeybdKey::FKey,
        KeyCommon::G => KeybdKey::GKey,
        KeyCommon::H => KeybdKey::HKey,
        KeyCommon::I => KeybdKey::IKey,
        KeyCommon::J => KeybdKey::JKey,
        KeyCommon::K => KeybdKey::KKey,
        KeyCommon::L => KeybdKey::LKey,
        KeyCommon::M => KeybdKey::MKey,
        KeyCommon::N => KeybdKey::NKey,
        KeyCommon::O => KeybdKey::OKey,
        KeyCommon::P => KeybdKey::PKey,
        KeyCommon::Q => KeybdKey::QKey,
        KeyCommon::R => KeybdKey::RKey,
        KeyCommon::S => KeybdKey::SKey,
        KeyCommon::T => KeybdKey::TKey,
        KeyCommon::U => KeybdKey::UKey,
        KeyCommon::V => KeybdKey::VKey,
        KeyCommon::W => KeybdKey::WKey,
        KeyCommon::X => KeybdKey::XKey,
        KeyCommon::Y => KeybdKey::YKey,
        KeyCommon::Z => KeybdKey::ZKey,
        KeyCommon::Num0 => KeybdKey::Numrow0Key,
        KeyCommon::Num1 => KeybdKey::Numrow1Key,
        KeyCommon::Num2 => KeybdKey::Numrow2Key,
        KeyCommon::Num3 => KeybdKey::Numrow3Key,
        KeyCommon::Num4 => KeybdKey::Numrow4Key,
        KeyCommon::Num5 => KeybdKey::Numrow5Key,
        KeyCommon::Num6 => KeybdKey::Numrow6Key,
        KeyCommon::Num7 => KeybdKey::Numrow7Key,
        KeyCommon::Num8 => KeybdKey::Numrow8Key,
        KeyCommon::Num9 => KeybdKey::Numrow9Key,
        KeyCommon::Alt => KeybdKey::LAltKey,
        KeyCommon::LAlt => KeybdKey::LAltKey,
        KeyCommon::RAlt => KeybdKey::RAltKey,
        KeyCommon::Shift => KeybdKey::LShiftKey,
        KeyCommon::LShift => KeybdKey::RShiftKey,
        KeyCommon::RShift => KeybdKey::LShiftKey,
        KeyCommon::Control => KeybdKey::LControlKey,
        KeyCommon::LControl => KeybdKey::LControlKey,
        KeyCommon::RControl => KeybdKey::RControlKey,
        KeyCommon::F1 => KeybdKey::F1Key,
        KeyCommon::F2 => KeybdKey::F2Key,
        KeyCommon::F3 => KeybdKey::F3Key,
        KeyCommon::F4 => KeybdKey::F4Key,
        KeyCommon::F5 => KeybdKey::F5Key,
        KeyCommon::F6 => KeybdKey::F6Key,
        KeyCommon::F7 => KeybdKey::F7Key,
        KeyCommon::F8 => KeybdKey::F8Key,
        KeyCommon::F9 => KeybdKey::F9Key,
        KeyCommon::F10 => KeybdKey::F10Key,
        KeyCommon::F11 => KeybdKey::F11Key,
        KeyCommon::F12 => KeybdKey::F12Key,
        KeyCommon::CapsLock => KeybdKey::CapsLockKey,
        KeyCommon::End => KeybdKey::EndKey,
        KeyCommon::Home => KeybdKey::HomeKey,
        KeyCommon::PageUp => KeybdKey::PageUpKey,
        KeyCommon::PageDown => KeybdKey::PageDownKey,
        KeyCommon::Escape => KeybdKey::EscapeKey,
        KeyCommon::Return => KeybdKey::EnterKey,
        KeyCommon::Space => KeybdKey::SpaceKey,
        KeyCommon::Tab => KeybdKey::TabKey,
        KeyCommon::Backspace => KeybdKey::BackspaceKey,
        KeyCommon::Delete => KeybdKey::DeleteKey,
        KeyCommon::UpArrow => KeybdKey::UpKey,
        KeyCommon::DownArrow => KeybdKey::DownKey,
        KeyCommon::LeftArrow => KeybdKey::LeftKey,
        KeyCommon::RightArrow => KeybdKey::RightKey,
    }
}
