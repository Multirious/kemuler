use once_cell::sync::Lazy;

use crate::simulate::{KeyCommon, KeyLayout, MouseButton};

use parking_lot::{MappedRwLockReadGuard, RwLock, RwLockReadGuard};

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

type StaticBackend<T> = Lazy<RwLock<Option<Box<T>>>>;
type BackendRead<T> = MappedRwLockReadGuard<'static, Box<T>>;

macro_rules! define_backend_static {
    (
        $(
        static $static_ident:ident: $trait:ty;
        pub fn $fn_get:ident(), expect_message = $expect_message:expr;
        pub fn $fn_set:ident();
        pub fn $fn_take:ident();
        )*
    ) => {
        $(
            static $static_ident: StaticBackend<$trait> =
                Lazy::new(|| RwLock::new(None));

            pub fn $fn_get() -> BackendRead<$trait> {
                RwLockReadGuard::map($static_ident.read(), |backend| {
                    backend.as_ref().expect($expect_message)
                })
            }

            pub fn $fn_set(backend: Box<$trait>) {
                let _ = $static_ident.write().insert(backend);
            }

            pub fn $fn_take() -> Option<Box<$trait>> {
                $static_ident.write().take()
            }
        )*
    };
}

define_backend_static! {
    static MOUSE_BACKEND: dyn MouseBackend + Send + Sync;
    pub fn get_mouse_backend(), expect_message = "No mouse backend";
    pub fn set_mouse_backend();
    pub fn take_mouse_backend();

    static KEYBOARD_COMMON_BACKEND: dyn KeyboardBackend<KeyCommon> + Send + Sync;
    pub fn get_keyboard_common_backend(), expect_message = "No keyboard common backend";
    pub fn set_keyboard_common_backend();
    pub fn take_keyboard_common_backend();

    static KEYBOARD_LAYOUT_BACKEND: dyn KeyboardBackend<KeyLayout> + Send + Sync;
    pub fn get_keyboard_layout_backend(), expect_message = "No keyboard layout backend";
    pub fn set_keyboard_layout_backend();
    pub fn take_keyboard_layout_backend();
}
