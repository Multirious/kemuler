use once_cell::sync::Lazy;
use std::sync::{RwLock, RwLockReadGuard};

use crate::simulate::{KeyCommon, KeyLayout, MouseButton};

use self::default::DefaultBackend;

pub mod default;
#[cfg(feature = "enigo")]
pub mod enigo;
#[cfg(feature = "inputbot")]
pub mod inputbot;

pub trait MouseBackend {
    fn mouse_move_to(&self, x: i32, y: i32);
    fn mouse_move_by(&self, x: i32, y: i32);
    fn mouse_scroll(&self, x: i32, y: i32);
    fn mouse_down(&self, button: MouseButton);
    fn mouse_up(&self, button: MouseButton);
}

pub trait KeyboardBackend<K> {
    fn key_down(&self, key: K);
    fn key_up(&self, key: K);
}

type StaticBackend<T> = Lazy<RwLock<Box<T>>>;
type BackendRead<T> = RwLockReadGuard<'static, Box<T>>;

macro_rules! define_backend_static {
    (
        $(
        static $static_ident:ident: $trait:ty;
        pub fn $fn_get:ident();
        pub fn $fn_set:ident();
        )*
    ) => {
        $(
            static $static_ident: StaticBackend<$trait> =
                Lazy::new(|| RwLock::new(Box::new(DefaultBackend)));

            pub fn $fn_get() -> BackendRead<$trait> {
                $static_ident.read().unwrap()
            }

            pub fn $fn_set(backend: Box<$trait>) {
                *$static_ident.write().unwrap() = backend;
            }
        )*
    };
}

define_backend_static! {
    static MOUSE_BACKEND: dyn MouseBackend + Send + Sync;
    pub fn get_mouse_backend();
    pub fn set_mouse_backend();

    static KEYBOARD_COMMON_BACKEND: dyn KeyboardBackend<KeyCommon> + Send + Sync;
    pub fn get_keyboard_common_backend();
    pub fn set_keyboard_common_backend();

    static KEYBOARD_LAYOUT_BACKEND: dyn KeyboardBackend<KeyLayout> + Send + Sync;
    pub fn get_keyboard_layout_backend();
    pub fn set_keyboard_layout_backend();
}
