use once_cell::sync::Lazy;
use std::sync::{RwLock, RwLockReadGuard};

use crate::peripherals::{Keyboard, MouseButton};

use self::default::DefaultBackend;

mod default;
#[cfg(feature = "enigo")]
mod enigo;
#[cfg(feature = "rdev")]
mod rdev;

pub trait MouseBackend {
    fn mouse_move_to(&self, x: i32, y: i32);
    fn mouse_move_by(&self, x: i32, y: i32);
    fn mouse_scroll(&self, x: i32, y: i32);
    fn mouse_down(&self, button: &MouseButton);
    fn mouse_up(&self, button: &MouseButton);
}

pub trait KeyboardBackend {
    fn key_down(&self, key: &Keyboard);
    fn key_up(&self, key: &Keyboard);
}

type StaticBackend<T> = Lazy<RwLock<Box<T>>>;
type BackendRead<T> = RwLockReadGuard<'static, Box<T>>;

static MOUSE_BACKEND: StaticBackend<dyn MouseBackend + Send + Sync> =
    Lazy::new(|| RwLock::new(Box::new(DefaultBackend)));
static KEYBOARD_BACKEND: StaticBackend<dyn KeyboardBackend + Send + Sync> =
    Lazy::new(|| RwLock::new(Box::new(DefaultBackend)));

pub fn get_mouse_backend() -> BackendRead<(dyn MouseBackend + Send + Sync)> {
    MOUSE_BACKEND.read().unwrap()
}

pub fn set_mouse_backend(backend: Box<(dyn MouseBackend + Send + Sync)>) {
    *MOUSE_BACKEND.write().unwrap() = backend;
}

pub fn get_keyboard_backend() -> BackendRead<(dyn KeyboardBackend + Send + Sync)> {
    KEYBOARD_BACKEND.read().unwrap()
}

pub fn set_keyboard_backend(backend: Box<dyn KeyboardBackend + Send + Sync>) {
    *KEYBOARD_BACKEND.write().unwrap() = backend;
}
