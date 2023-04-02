use crate::peripherals::{Keyboard, MouseButton};

use super::{KeyboardBackend, MouseBackend};

pub struct DefaultBackend;

cfg_if::cfg_if! {
    if #[cfg(feature = "enigo")] {

    }
}
impl MouseBackend for DefaultBackend {
    fn mouse_move_to(&self, x: i32, y: i32) {
        todo!()
    }

    fn mouse_move_by(&self, x: i32, y: i32) {
        todo!()
    }

    fn mouse_scroll(&self, x: i32, y: i32) {
        todo!()
    }

    fn mouse_down(&self, button: &MouseButton) {
        todo!()
    }

    fn mouse_up(&self, button: &MouseButton) {
        todo!()
    }
}

#[cfg(feature = "enigo")]
impl KeyboardBackend for DefaultBackend {
    fn key_down(&self, key: &Keyboard) {
        todo!()
    }

    fn key_up(&self, key: &Keyboard) {
        todo!()
    }
}
