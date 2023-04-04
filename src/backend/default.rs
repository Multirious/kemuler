use crate::peripherals::{KeyCommon, KeyLayout, MouseButton};

use super::{KeyboardBackend, MouseBackend};

pub struct DefaultBackend;

cfg_if::cfg_if! {
    if #[cfg(feature = "enigo")] {
        use super::enigo;
        impl MouseBackend for DefaultBackend {
            fn mouse_move_to(&self, x: i32, y: i32) {
                enigo::General.mouse_move_to(x, y);
            }

            fn mouse_move_by(&self, x: i32, y: i32) {
                enigo::General.mouse_move_by(x, y);
            }

            fn mouse_scroll(&self, x: i32, y: i32) {
                enigo::General.mouse_scroll(x, y);
            }

            fn mouse_down(&self, button: MouseButton) {
                enigo::General.mouse_down(button);
            }

            fn mouse_up(&self, button: MouseButton) {
                enigo::General.mouse_up(button);
            }
        }

        impl KeyboardBackend<KeyCommon> for DefaultBackend {
            fn key_down(&self, key: KeyCommon) {
                enigo::General.key_down(key);
            }

            fn key_up(&self, key: KeyCommon) {
                enigo::General.key_up(key);
            }
        }

        impl KeyboardBackend<KeyLayout> for DefaultBackend {
            fn key_down(&self, key: KeyLayout) {
                enigo::General.key_down(key);
            }

            fn key_up(&self, key: KeyLayout) {
                enigo::General.key_up(key);
            }
        }
    } else {
        impl MouseBackend for DefaultBackend {
            fn mouse_move_to(&self, x: i32, y: i32) {
                panic!("No default emulate mouse input implementation")
            }

            fn mouse_move_by(&self, x: i32, y: i32) {
                panic!("No default emulate mouse input implementation")
            }

            fn mouse_scroll(&self, x: i32, y: i32) {
                panic!("No default emulate mouse input implementation")
            }

            fn mouse_down(&self, button: MouseButton) {
                panic!("No default emulate mouse input implementation")
            }

            fn mouse_up(&self, button: MouseButton) {
                panic!("No default emulate mouse input implementation")
            }
        }

        impl KeyboardBackend<KeyCommon> for DefaultBackend {
            fn key_down(&self, key: KeyCommon) {
                panic!("No default keyboard common input implementation")
            }

            fn key_up(&self, key: KeyCommon) {
                panic!("No default keyboard common input implementation")
            }
        }

        impl KeyboardBackend<KeyLayout> for DefaultBackend {
            fn key_down(&self, key: KeyLayout) {
                panic!("No default keyboard layout input implementation")
            }

            fn key_up(&self, key: KeyLayout) {
                panic!("No default keyboard layout input implementation")
            }
        }
    }
}
