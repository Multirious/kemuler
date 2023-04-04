use crate::{
    backend::{get_keyboard_common_backend, get_mouse_backend},
    combinator::Combine,
    emulatable::{EmulateAbsoluteValue, EmulateRelativeValue},
};

macro_rules! crate_impls {
    ($thing:ident) => {
        impl Combine for $thing {}

        impl<T> std::ops::Add<T> for $thing {
            type Output = crate::combinator::And<Self, T>;

            fn add(self, rhs: T) -> Self::Output {
                self.and(rhs)
            }
        }
    };
}

/// Layout independent key
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KeyCommon {
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

    Num0,
    Num1,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,

    /// alt key on Linux and Windows (option key on macOS)
    Alt,
    LAlt,
    RAlt,
    Shift,
    LShift,
    RShift,
    Control,
    LControl,
    RControl,

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

    CapsLock,

    End,
    Home,
    PageUp,
    PageDown,

    Escape,
    Return,
    Space,
    Tab,

    Backspace,
    Delete,

    UpArrow,
    DownArrow,
    LeftArrow,
    RightArrow,
    // AbntC1,
    // #[cfg(target_os = "windows")]
    // AbntC2,
    // #[cfg(target_os = "windows")]
    // Accept,
    // #[cfg(target_os = "windows")]
    // Add,

    // #[cfg(target_os = "windows")]
    // Apps,
    // #[cfg(target_os = "windows")]
    // Attn,
    // #[cfg(target_os = "linux")]
    // Begin,
    // #[cfg(target_os = "linux")]

    // #[cfg(target_os = "windows")]
    // BrowserBack,
    // #[cfg(target_os = "windows")]
    // BrowserFavorites,
    // #[cfg(target_os = "windows")]
    // BrowserForward,
    // #[cfg(target_os = "windows")]
    // BrowserHome,
    // #[cfg(target_os = "windows")]
    // BrowserRefresh,
    // #[cfg(target_os = "windows")]
    // BrowserSearch,
    // #[cfg(target_os = "windows")]
    // BrowserStop,
    // #[cfg(any(target_os = "windows", target_os = "linux"))]
    // Cancel,

    // #[cfg(any(target_os = "windows", target_os = "linux"))]
    // Clear,

    // #[cfg(target_os = "windows")]
    // Convert,
    // #[cfg(target_os = "windows")]
    // Crsel,
    // #[cfg(target_os = "windows")]
    // DBEAlphanumeric,
    // #[cfg(target_os = "windows")]
    // DBECodeinput,
    // #[cfg(target_os = "windows")]
    // DBEDetermineString,
    // #[cfg(target_os = "windows")]
    // DBEEnterDLGConversionMode,
    // #[cfg(target_os = "windows")]
    // DBEEnterIMEConfigMode,
    // #[cfg(target_os = "windows")]
    // DBEEnterWordRegisterMode,
    // #[cfg(target_os = "windows")]
    // DBEFlushString,
    // #[cfg(target_os = "windows")]
    // DBEHiragana,
    // #[cfg(target_os = "windows")]
    // DBEKatakana,
    // #[cfg(target_os = "windows")]
    // DBENoCodepoint,
    // #[cfg(target_os = "windows")]
    // DBENoRoman,
    // #[cfg(target_os = "windows")]
    // DBERoman,
    // #[cfg(target_os = "windows")]
    // DBESBCSChar,
    // #[cfg(target_os = "windows")]
    // DBESChar,
    // #[cfg(target_os = "windows")]
    // Decimal,

    // #[cfg(target_os = "windows")]
    // Divide,

    // #[cfg(target_os = "windows")]
    // Ereof,

    // #[cfg(any(target_os = "windows", target_os = "linux"))]
    // Execute,
    // #[cfg(target_os = "windows")]
    // Exsel,
    // /// F13 key
    // F13,
    // /// F14 key
    // F14,
    // /// F15 key
    // F15,
    // /// F16 key
    // F16,
    // /// F17 key
    // F17,
    // /// F18 key
    // F18,
    // /// F19 key
    // F19,
    // /// F20 key
    // F20,
    // #[cfg(any(target_os = "windows", target_os = "linux"))]
    // /// F21 key
    // F21,
    // #[cfg(any(target_os = "windows", target_os = "linux"))]
    // /// F22 key
    // F22,
    // #[cfg(any(target_os = "windows", target_os = "linux"))]
    // /// F23 key
    // F23,
    // #[cfg(any(target_os = "windows", target_os = "linux"))]
    // /// F24 key
    // F24,
    // #[cfg(target_os = "linux")]
    // F25,
    // #[cfg(target_os = "linux")]
    // F26,
    // #[cfg(target_os = "linux")]
    // F27,
    // #[cfg(target_os = "linux")]
    // F28,
    // #[cfg(target_os = "linux")]
    // F29,
    // #[cfg(target_os = "linux")]
    // F30,
    // #[cfg(target_os = "linux")]
    // F31,
    // #[cfg(target_os = "linux")]
    // F32,
    // #[cfg(target_os = "linux")]
    // F33,
    // #[cfg(target_os = "linux")]
    // F34,
    // #[cfg(target_os = "linux")]
    // F35,
    // #[cfg(target_os = "macos")]
    // Function,
    // #[cfg(target_os = "windows")]
    // Final,
    // #[cfg(target_os = "linux")]
    // Find,
    // #[cfg(target_os = "windows")]
    // GamepadA,
    // #[cfg(target_os = "windows")]
    // GamepadB,
    // #[cfg(target_os = "windows")]
    // GamepadDPadDown,
    // #[cfg(target_os = "windows")]
    // GamepadDPadLeft,
    // #[cfg(target_os = "windows")]
    // GamepadDPadRight,
    // #[cfg(target_os = "windows")]
    // GamepadDPadUp,
    // #[cfg(target_os = "windows")]
    // GamepadLeftShoulder,
    // #[cfg(target_os = "windows")]
    // GamepadLeftThumbstickButton,
    // #[cfg(target_os = "windows")]
    // GamepadLeftThumbstickDown,
    // #[cfg(target_os = "windows")]
    // GamepadLeftThumbstickLeft,
    // #[cfg(target_os = "windows")]
    // GamepadLeftThumbstickRight,
    // #[cfg(target_os = "windows")]
    // GamepadLeftThumbstickUp,
    // #[cfg(target_os = "windows")]
    // GamepadLeftTrigger,
    // #[cfg(target_os = "windows")]
    // GamepadMenu,
    // #[cfg(target_os = "windows")]
    // GamepadRightShoulder,
    // #[cfg(target_os = "windows")]
    // GamepadRightThumbstickButton,
    // #[cfg(target_os = "windows")]
    // GamepadRightThumbstickDown,
    // #[cfg(target_os = "windows")]
    // GamepadRightThumbstickLeft,
    // #[cfg(target_os = "windows")]
    // GamepadRightThumbstickRight,
    // #[cfg(target_os = "windows")]
    // GamepadRightThumbstickUp,
    // #[cfg(target_os = "windows")]
    // GamepadRightTrigger,
    // #[cfg(target_os = "windows")]
    // GamepadView,
    // #[cfg(target_os = "windows")]
    // GamepadX,
    // #[cfg(target_os = "windows")]
    // GamepadY,
    // #[cfg(target_os = "windows")]
    // Hangeul,
    // #[cfg(any(target_os = "windows", target_os = "linux"))]
    // Hangul,
    // #[cfg(any(target_os = "windows", target_os = "linux"))]
    // Hanja,
    // Help,

    // #[cfg(target_os = "windows")]
    // Ico00,
    // #[cfg(target_os = "windows")]
    // IcoClear,
    // #[cfg(target_os = "windows")]
    // IcoHelp,
    // #[cfg(target_os = "windows")]
    // IMEOff,
    // #[cfg(target_os = "windows")]
    // IMEOn,
    // #[cfg(any(target_os = "windows", target_os = "linux"))]
    // Insert,
    // #[cfg(target_os = "windows")]
    // Junja,
    // #[cfg(target_os = "windows")]
    // Kana,
    // #[cfg(any(target_os = "windows", target_os = "linux"))]
    // Kanji,
    // #[cfg(target_os = "windows")]
    // LaunchApp1,
    // #[cfg(target_os = "windows")]
    // LaunchApp2,
    // #[cfg(target_os = "windows")]
    // LaunchMail,
    // #[cfg(target_os = "windows")]
    // LaunchMediaSelect,
    // #[cfg(target_os = "macos")]
    // /// Opens launchpad
    // Launchpad,
    // #[cfg(target_os = "windows")]
    // LButton,

    // #[cfg(target_os = "linux")]
    // Linefeed,
    // #[cfg(any(target_os = "windows", target_os = "linux"))]
    // LMenu,

    // #[cfg(target_os = "windows")]
    // LWin,
    // #[cfg(target_os = "windows")]
    // MButton,
    // #[cfg(target_os = "windows")]
    // MediaNextTrack,
    // #[cfg(target_os = "windows")]
    // MediaPlayPause,
    // #[cfg(target_os = "windows")]
    // MediaPrevTrack,
    // #[cfg(target_os = "windows")]
    // MediaStop,
    // /// meta key (also known as "windows", "super", and "command")
    // Meta,
    // #[cfg(target_os = "macos")]
    // /// Opens mission control
    // MissionControl,
    // #[cfg(any(target_os = "windows", target_os = "linux"))]
    // ModeChange,
    // #[cfg(target_os = "windows")]
    // Multiply,
    // #[cfg(target_os = "windows")]
    // NavigationAccept,
    // #[cfg(target_os = "windows")]
    // NavigationCancel,
    // #[cfg(target_os = "windows")]
    // NavigationDown,
    // #[cfg(target_os = "windows")]
    // NavigationLeft,
    // #[cfg(target_os = "windows")]
    // NavigationMenu,
    // #[cfg(target_os = "windows")]
    // NavigationRight,
    // #[cfg(target_os = "windows")]
    // NavigationUp,
    // #[cfg(target_os = "windows")]
    // NavigationView,
    // #[cfg(target_os = "windows")]
    // NoName,
    // #[cfg(target_os = "windows")]
    // NonConvert,
    // #[cfg(target_os = "windows")]
    // None,
    // #[cfg(any(target_os = "windows", target_os = "linux"))]
    // Numlock,
    // #[cfg(target_os = "windows")]
    // Numpad0,
    // #[cfg(target_os = "windows")]
    // Numpad1,
    // #[cfg(target_os = "windows")]
    // Numpad2,
    // #[cfg(target_os = "windows")]
    // Numpad3,
    // #[cfg(target_os = "windows")]
    // Numpad4,
    // #[cfg(target_os = "windows")]
    // Numpad5,
    // #[cfg(target_os = "windows")]
    // Numpad6,
    // #[cfg(target_os = "windows")]
    // Numpad7,
    // #[cfg(target_os = "windows")]
    // Numpad8,
    // #[cfg(target_os = "windows")]
    // Numpad9,

    // #[cfg(target_os = "windows")]
    // OEM1,
    // #[cfg(target_os = "windows")]
    // OEM102,
    // #[cfg(target_os = "windows")]
    // OEM2,
    // #[cfg(target_os = "windows")]
    // OEM3,
    // #[cfg(target_os = "windows")]
    // OEM4,
    // #[cfg(target_os = "windows")]
    // OEM5,
    // #[cfg(target_os = "windows")]
    // OEM6,
    // #[cfg(target_os = "windows")]
    // OEM7,
    // #[cfg(target_os = "windows")]
    // OEM8,
    // #[cfg(target_os = "windows")]
    // OEMAttn,
    // #[cfg(target_os = "windows")]
    // OEMAuto,
    // #[cfg(target_os = "windows")]
    // OEMAx,
    // #[cfg(target_os = "windows")]
    // OEMBacktab,
    // #[cfg(target_os = "windows")]
    // OEMClear,
    // #[cfg(target_os = "windows")]
    // OEMComma,
    // #[cfg(target_os = "windows")]
    // OEMCopy,
    // #[cfg(target_os = "windows")]
    // OEMCusel,
    // #[cfg(target_os = "windows")]
    // OEMEnlw,
    // #[cfg(target_os = "windows")]
    // OEMFinish,
    // #[cfg(target_os = "windows")]
    // OEMFJJisho,
    // #[cfg(target_os = "windows")]
    // OEMFJLoya,
    // #[cfg(target_os = "windows")]
    // OEMFJMasshou,
    // #[cfg(target_os = "windows")]
    // OEMFJRoya,
    // #[cfg(target_os = "windows")]
    // OEMFJTouroku,
    // #[cfg(target_os = "windows")]
    // OEMJump,
    // #[cfg(target_os = "windows")]
    // OEMMinus,
    // #[cfg(target_os = "windows")]
    // OEMNECEqual,
    // #[cfg(target_os = "windows")]
    // OEMPA1,
    // #[cfg(target_os = "windows")]
    // OEMPA2,
    // #[cfg(target_os = "windows")]
    // OEMPA3,
    // #[cfg(target_os = "windows")]
    // OEMPeriod,
    // #[cfg(target_os = "windows")]
    // OEMPlus,
    // #[cfg(target_os = "windows")]
    // OEMReset,
    // #[cfg(target_os = "windows")]
    // OEMWsctrl,
    // /// option key on macOS (alt key on Linux and Windows)
    // Option,
    // #[cfg(target_os = "windows")]
    // PA1,
    // #[cfg(target_os = "windows")]
    // Packet,

    // #[cfg(any(target_os = "windows", target_os = "linux"))]
    // Pause,
    // #[cfg(target_os = "windows")]
    // Play,
    // #[cfg(any(target_os = "windows", target_os = "linux"))]
    // Print,
    // #[cfg(target_os = "windows")]
    // Processkey,
    // #[cfg(target_os = "windows")]
    // RButton,
    // #[cfg(target_os = "macos")]
    // RCommand,

    // #[cfg(target_os = "linux")]
    // Redo,

    // #[cfg(target_os = "windows")]
    // RMenu,
    // #[cfg(target_os = "macos")]
    // ROption,

    // #[cfg(target_os = "windows")]
    // RWin,
    // #[cfg(target_os = "windows")]
    // Scroll,
    // #[cfg(target_os = "linux")]
    // ScrollLock,
    // #[cfg(any(target_os = "windows", target_os = "linux"))]
    // Select,
    // #[cfg(target_os = "linux")]
    // ScriptSwitch,
    // #[cfg(target_os = "windows")]
    // Separator,

    // #[cfg(target_os = "linux")]
    // /// Lock shift key
    // ShiftLock,
    // #[cfg(target_os = "windows")]
    // Sleep,
    // #[cfg(target_os = "windows")]
    // Snapshot,

    // #[cfg(target_os = "windows")]
    // Subtract,
    // #[cfg(target_os = "linux")]
    // SysReq,

    // #[cfg(target_os = "linux")]
    // Undo,
    // #[cfg(any(target_os = "windows", target_os = "macos"))]
    // VolumeDown,
    // #[cfg(any(target_os = "windows", target_os = "macos"))]
    // VolumeMute,
    // #[cfg(any(target_os = "windows", target_os = "macos"))]
    // VolumeUp,
    // #[cfg(target_os = "windows")]
    // XButton1,
    // #[cfg(target_os = "windows")]
    // XButton2,
    // #[cfg(target_os = "windows")]
    // Zoom,

    // /// keyboard layout dependent key
    // Layout(char),
    // /// raw keycode eg 0x38
    // Raw(u16),
}

/// Layout dependent key
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct KeyLayout(pub char);

impl EmulateAbsoluteValue for KeyCommon {
    type Value = bool;

    fn change_to(&mut self, to: Self::Value) -> &mut Self {
        if to {
            get_keyboard_common_backend().key_down(*self);
        } else {
            get_keyboard_common_backend().key_up(*self);
        }
        self
    }
}

crate_impls! { KeyCommon }

#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MouseButton {
    Left,
    Middle,
    Right,
    X1,
    X2,
}

crate_impls! { MouseButton }

impl EmulateAbsoluteValue for MouseButton {
    type Value = bool;

    fn change_to(&mut self, to: Self::Value) -> &mut Self {
        if to {
            get_mouse_backend().mouse_down(*self);
        } else {
            get_mouse_backend().mouse_up(*self);
        }
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MousePosition;

crate_impls! { MousePosition }

impl EmulateAbsoluteValue for MousePosition {
    type Value = (i32, i32);

    fn change_to(&mut self, to: Self::Value) -> &mut Self {
        get_mouse_backend().mouse_move_to(to.0, to.1);
        self
    }
}

impl EmulateRelativeValue for MousePosition {
    type Value = (i32, i32);

    fn change_by(&mut self, by: Self::Value) -> &mut Self {
        get_mouse_backend().mouse_move_by(by.0, by.1);
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MouseScroll;

crate_impls! { MouseScroll }

impl EmulateRelativeValue for MouseScroll {
    type Value = (i32, i32);

    fn change_by(&mut self, by: Self::Value) -> &mut Self {
        get_mouse_backend().mouse_scroll(by.0, by.1);
        self
    }
}
