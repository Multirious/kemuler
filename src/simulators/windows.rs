use crate::{
    combinator::{AndThen, Combine},
    inputs::common,
    simulatable::{ChangeBy, SetTo},
    simulator::Simulate,
};

mod inner;

macro_rules! virtual_key_enum {
    (
        $(
            $(#[$attr:meta])*
            $variant:ident => $og_ident:ident
        )*
    ) => {
        /// https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes
        pub enum VirtualKey {
            $(
                $(#[$attr])*
                #[doc = ""]
                #[doc = "`"]
                #[doc = stringify!($og_ident)
]                #[doc = "`"]
                $variant,
            )*
        }

        impl VirtualKey {
            pub fn code(&self) -> inner::VirtualKeyCode {
                match self {
                    $(
                        VirtualKey::$variant => inner::VirtualKeyCode::$og_ident,
                    )*
                }
            }
        }
    };
}

virtual_key_enum! {
    Num0 => VK_0
    Num1 => VK_1
    Num2 => VK_2
    Num3 => VK_3
    Num4 => VK_4
    Num5 => VK_5
    Num6 => VK_6
    Num7 => VK_7
    Num8 => VK_8
    Num9 => VK_9
    A => VK_A
    B => VK_B
    C => VK_C
    D => VK_D
    E => VK_E
    F => VK_F
    G => VK_G
    H => VK_H
    I => VK_I
    J => VK_J
    K => VK_K
    L => VK_L
    M => VK_M
    N => VK_N
    O => VK_O
    P => VK_P
    Q => VK_Q
    R => VK_R
    S => VK_S
    T => VK_T
    U => VK_U
    V => VK_V
    W => VK_W
    X => VK_X
    Y => VK_Y
    Z => VK_Z
    AbntC1 => VK_ABNT_C1
    AbntC2 => VK_ABNT_C2
    DbeAlphanumeric => VK_DBE_ALPHANUMERIC
    DbeCodeInput => VK_DBE_CODEINPUT
    DbeDbcsChar => VK_DBE_DBCSCHAR
    DbeDetermineString => VK_DBE_DETERMINESTRING
    DbeEnterDlgConversionMode => VK_DBE_ENTERDLGCONVERSIONMODE
    DbeEnterImeConfigMode => VK_DBE_ENTERIMECONFIGMODE
    DbeEnterWordRegisterMode => VK_DBE_ENTERWORDREGISTERMODE
    DbeFlushString => VK_DBE_FLUSHSTRING
    DbeHiragana => VK_DBE_HIRAGANA
    DbeKatakana => VK_DBE_KATAKANA
    DbeNoCodeInput => VK_DBE_NOCODEINPUT
    DbeNoRoman => VK_DBE_NOROMAN
    DbeRoman => VK_DBE_ROMAN
    DbeSbcsChar => VK_DBE_SBCSCHAR
    _none_ => VK__none_
    /// Left mouse button
    LButton => VK_LBUTTON
    /// Right mouse button
    RButton => VK_RBUTTON
    /// Control-break processing
    Cancel => VK_CANCEL
    /// Middle mouse button (three-button mouse)
    MButton => VK_MBUTTON
    /// X1 mouse button
    XButton1 => VK_XBUTTON1
    /// X2 mouse button
    XButton2 => VK_XBUTTON2
    /// BACKSPACE key
    Back => VK_BACK
    /// TAB key
    Tab => VK_TAB
    /// CLEAR key
    Clear => VK_CLEAR
    /// ENTER key
    Return => VK_RETURN
    /// SHIFT key
    Shift => VK_SHIFT
    /// CTRL key
    Control => VK_CONTROL
    /// ALT key
    Menu => VK_MENU
    /// PAUSE key
    Pause => VK_PAUSE
    /// CAPS LOCK key
    Capital => VK_CAPITAL
    /// IME Kana mode
    Kana => VK_KANA
    /// IME Hanguel mode (maintained for compatibility; use VK_HANGUL)
    Hangeul => VK_HANGEUL
    /// IME Hangul mode
    Hangul => VK_HANGUL
    /// IME On
    ImeOn => VK_IME_ON
    /// IME Junja mode
    Junja => VK_JUNJA
    /// IME final mode
    Final => VK_FINAL
    /// IME Hanja mode
    Hanja => VK_HANJA
    /// IME Kanji mode
    Kanji => VK_KANJI
    /// IME Off
    ImeOff => VK_IME_OFF
    /// ESC key
    Escape => VK_ESCAPE
    /// IME convert
    Convert => VK_CONVERT
    /// IME nonconvert
    NonConvert => VK_NONCONVERT
    /// IME accept
    Accept => VK_ACCEPT
    /// IME mode change request
    ModeChange => VK_MODECHANGE
    /// SPACEBAR
    Space => VK_SPACE
    /// PAGE UP key
    Prior => VK_PRIOR
    /// PAGE DOWN key
    Next => VK_NEXT
    /// END key
    End => VK_END
    /// HOME key
    Home => VK_HOME
    /// LEFT ARROW key
    Left => VK_LEFT
    /// UP ARROW key
    Up => VK_UP
    /// RIGHT ARROW key
    Right => VK_RIGHT
    /// DOWN ARROW key
    Down => VK_DOWN
    /// SELECT key
    Select => VK_SELECT
    /// PRINT key
    Print => VK_PRINT
    /// EXECUTE key
    Execute => VK_EXECUTE
    /// PRINT SCREEN key
    Snapshot => VK_SNAPSHOT
    /// INS key
    Insert => VK_INSERT
    /// DEL key
    Delete => VK_DELETE
    /// HELP key
    Help => VK_HELP
    /// Left Windows key (Natural keyboard)
    LWin => VK_LWIN
    /// Right Windows key (Natural keyboard)
    RWin => VK_RWIN
    /// Applications key (Natural keyboard)
    Apps => VK_APPS
    /// Computer Sleep key
    Sleep => VK_SLEEP
    /// Numeric keypad 0 key
    NumPad0 => VK_NUMPAD0
    /// Numeric keypad 1 key
    NumPad1 => VK_NUMPAD1
    /// Numeric keypad 2 key
    NumPad2 => VK_NUMPAD2
    /// Numeric keypad 3 key
    NumPad3 => VK_NUMPAD3
    /// Numeric keypad 4 key
    NumPad4 => VK_NUMPAD4
    /// Numeric keypad 5 key
    NumPad5 => VK_NUMPAD5
    /// Numeric keypad 6 key
    NumPad6 => VK_NUMPAD6
    /// Numeric keypad 7 key
    NumPad7 => VK_NUMPAD7
    /// Numeric keypad 8 key
    NumPad8 => VK_NUMPAD8
    /// Numeric keypad 9 key
    NumPad9 => VK_NUMPAD9
    /// Multiply key
    Multiply => VK_MULTIPLY
    /// Add key
    Add => VK_ADD
    /// Separator key
    Separator => VK_SEPARATOR
    /// Subtract key
    Subtract => VK_SUBTRACT
    /// Decimal key
    Decimal => VK_DECIMAL
    /// Divide key
    Divide => VK_DIVIDE
    /// F1 key
    F1 => VK_F1
    /// F2 key
    F2 => VK_F2
    /// F3 key
    F3 => VK_F3
    /// F4 key
    F4 => VK_F4
    /// F5 key
    F5 => VK_F5
    /// F6 key
    F6 => VK_F6
    /// F7 key
    F7 => VK_F7
    /// F8 key
    F8 => VK_F8
    /// F9 key
    F9 => VK_F9
    /// F10 key
    F10 => VK_F10
    /// F11 key
    F11 => VK_F11
    /// F12 key
    F12 => VK_F12
    /// F13 key
    F13 => VK_F13
    /// F14 key
    F14 => VK_F14
    /// F15 key
    F15 => VK_F15
    /// F16 key
    F16 => VK_F16
    /// F17 key
    F17 => VK_F17
    /// F18 key
    F18 => VK_F18
    /// F19 key
    F19 => VK_F19
    /// F20 key
    F20 => VK_F20
    /// F21 key
    F21 => VK_F21
    /// F22 key
    F22 => VK_F22
    /// F23 key
    F23 => VK_F23
    /// F24 key
    F24 => VK_F24
    NavigationView => VK_NAVIGATION_VIEW
    NavigationMenu => VK_NAVIGATION_MENU
    NavigationUP => VK_NAVIGATION_UP
    NavigationDown => VK_NAVIGATION_DOWN
    NavigationLeft => VK_NAVIGATION_LEFT
    NavigationRight => VK_NAVIGATION_RIGHT
    NavigationAccept => VK_NAVIGATION_ACCEPT
    NavigationCancel => VK_NAVIGATION_CANCEL
    /// NUM LOCK key
    NumLock => VK_NUMLOCK
    /// SCROLL LOCK key
    Scroll => VK_SCROLL
    OemNecEqual => VK_OEM_NEC_EQUAL
    OemFjJisho => VK_OEM_FJ_JISHO
    OemFjMasshou => VK_OEM_FJ_MASSHOU
    OemFjTouroku => VK_OEM_FJ_TOUROKU
    OemFjLoya => VK_OEM_FJ_LOYA
    OemFjRoya => VK_OEM_FJ_ROYA
    /// Left SHIFT key
    LShift => VK_LSHIFT
    /// Right SHIFT key
    RShift => VK_RSHIFT
    /// Left CONTROL key
    LControl => VK_LCONTROL
    /// Right CONTROL key
    RControl => VK_RCONTROL
    /// Left ALT key
    LMenu => VK_LMENU
    /// Right ALT key
    RMenu => VK_RMENU
    /// Browser Back key
    BrowserBack => VK_BROWSER_BACK
    /// Browser Forward key
    BrowserForward => VK_BROWSER_FORWARD
    /// Browser Refresh key
    BrowserRefresh => VK_BROWSER_REFRESH
    /// Browser Stop key
    BrowserStop => VK_BROWSER_STOP
    /// Browser Search key
    BrowserSearch => VK_BROWSER_SEARCH
    /// Browser Favorites key
    BrowserFavorites => VK_BROWSER_FAVORITES
    /// Browser Start and Home key
    BrowserHome => VK_BROWSER_HOME
    /// Volume Mute key
    VolumeMute => VK_VOLUME_MUTE
    /// Volume Down key
    VolumeDown => VK_VOLUME_DOWN
    /// Volume Up key
    VolumeUp => VK_VOLUME_UP
    /// Next Track key
    MediaNextTrack => VK_MEDIA_NEXT_TRACK
    /// Previous Track key
    MediaPrevTrack => VK_MEDIA_PREV_TRACK
    /// Stop Media key
    MediaStop => VK_MEDIA_STOP
    /// Play/Pause Media key
    MediaPlayPause => VK_MEDIA_PLAY_PAUSE
    /// Start Mail key
    LaunchMail => VK_LAUNCH_MAIL
    /// Select Media key
    LaunchMediaSelect => VK_LAUNCH_MEDIA_SELECT
    /// Start Application 1 key
    LaunchApp1 => VK_LAUNCH_APP1
    /// Start Application 2 key
    LaunchApp2 => VK_LAUNCH_APP2
    /// For any country/region, the `;:` key
    Oem1 => VK_OEM_1
    /// For any country/region, the `+` key
    OemPlus => VK_OEM_PLUS
    /// For any country/region, the `,` key
    OemComma => VK_OEM_COMMA
    /// For any country/region, the `-` key
    OemMinus => VK_OEM_MINUS
    /// For any country/region, the `.` key
    OemPeriod => VK_OEM_PERIOD
    /// Used for miscellaneous characters; it can vary by keyboard. For the US standard keyboard, the `/?` key
    Oem2 => VK_OEM_2
    /// Used for miscellaneous characters; it can vary by keyboard. For the US standard keyboard, the `\`~` key
    Oem3 => VK_OEM_3
    GamepadA => VK_GAMEPAD_A
    GamepadB => VK_GAMEPAD_B
    GamepadX => VK_GAMEPAD_X
    GamepadY => VK_GAMEPAD_Y
    GamepadRightShoulder => VK_GAMEPAD_RIGHT_SHOULDER
    GamepadLeftShoulder => VK_GAMEPAD_LEFT_SHOULDER
    GamepadLeftTrigger => VK_GAMEPAD_LEFT_TRIGGER
    GamepadRightTrigger => VK_GAMEPAD_RIGHT_TRIGGER
    GamepadDPadUp => VK_GAMEPAD_DPAD_UP
    GamepadDPadDown => VK_GAMEPAD_DPAD_DOWN
    GamepadDPadLeft => VK_GAMEPAD_DPAD_LEFT
    GamepadDPadRight => VK_GAMEPAD_DPAD_RIGHT
    GamepadMenu => VK_GAMEPAD_MENU
    GamepadView => VK_GAMEPAD_VIEW
    GamepadLeftThumbStickButton => VK_GAMEPAD_LEFT_THUMBSTICK_BUTTON
    GamepadRightThumbStickButton => VK_GAMEPAD_RIGHT_THUMBSTICK_BUTTON
    GamepadLeftThumbStickUp => VK_GAMEPAD_LEFT_THUMBSTICK_UP
    GamepadLeftThumbStickDown => VK_GAMEPAD_LEFT_THUMBSTICK_DOWN
    GamepadLeftThumbStickRight => VK_GAMEPAD_LEFT_THUMBSTICK_RIGHT
    GamepadLeftThumbStickLeft => VK_GAMEPAD_LEFT_THUMBSTICK_LEFT
    GamepadRightThumbStickUp => VK_GAMEPAD_RIGHT_THUMBSTICK_UP
    GamepadRightThumbStickDown => VK_GAMEPAD_RIGHT_THUMBSTICK_DOWN
    GamepadRightThumbStickRight => VK_GAMEPAD_RIGHT_THUMBSTICK_RIGHT
    GamepadRightThumbStickLeft => VK_GAMEPAD_RIGHT_THUMBSTICK_LEFT
    /// Used for miscellaneous characters; it can vary by keyboard. For the US standard keyboard, the `[{` key
    Oem4 => VK_OEM_4
    /// Used for miscellaneous characters; it can vary by keyboard. For the US standard keyboard, the `\|` key
    Oem5 => VK_OEM_5
    /// Used for miscellaneous characters; it can vary by keyboard. For the US standard keyboard, the `]}` key
    Oem6 => VK_OEM_6
    /// Used for miscellaneous characters; it can vary by keyboard. For the US standard keyboard, the `'"` key
    Oem7 => VK_OEM_7
    /// Used for miscellaneous characters; it can vary by keyboard.
    Oem8 => VK_OEM_8
    OemAx => VK_OEM_AX
    /// The `<>` keys on the US standard keyboard, or the `\\|` key on the non-US 102-key keyboard
    Oem102 => VK_OEM_102
    IcoHelp => VK_ICO_HELP
    Ico00 => VK_ICO_00
    /// IME PROCESS key
    ProcessKey => VK_PROCESSKEY
    IcoClear => VK_ICO_CLEAR
    /// Used to pass Unicode characters as if they were keystrokes. The VK_PACKET key is the low word of a 32-bit Virtual Key value used for non-keyboard input methods. For more information, see Remark in <a href="/en-us/windows/win32/api/winuser/ns-winuser-keybdinput" data-linktype="absolute-path">KEYBDINPUT</a>, <a href="/en-us/windows/win32/api/winuser/nf-winuser-sendinput" data-linktype="absolute-path">SendInput</a>, <a href="wm-keydown" data-linktype="relative-path">WM_KEYDOWN</a>, and <a href="wm-keyup" data-linktype="relative-path">WM_KEYUP</a>
    Packet => VK_PACKET
    OemReset => VK_OEM_RESET
    OemJump => VK_OEM_JUMP
    OemPa1 => VK_OEM_PA1
    OemPa2 => VK_OEM_PA2
    OemPa3 => VK_OEM_PA3
    OemWsctrl => VK_OEM_WSCTRL
    OemCusel => VK_OEM_CUSEL
    OemAttn => VK_OEM_ATTN
    OemFinish => VK_OEM_FINISH
    OemCopy => VK_OEM_COPY
    OemAuto => VK_OEM_AUTO
    OemEnlw => VK_OEM_ENLW
    OemBacktab => VK_OEM_BACKTAB
    /// Attn key
    Attn => VK_ATTN
    /// CrSel key
    CrSel => VK_CRSEL
    /// ExSel key
    ExSel => VK_EXSEL
    /// Erase EOF key
    ErEof => VK_EREOF
    /// Play key
    Play => VK_PLAY
    /// Zoom key
    Zoom => VK_ZOOM
    /// Reserved
    Noname => VK_NONAME
    /// PA1 key
    Pa1 => VK_PA1
    /// Clear key
    OemClear => VK_OEM_CLEAR
}

/// Mouse position precision is subject to differ a bit due to weird Windows API
/// I had to normalize.
#[derive(Default)]
pub struct Windows;

impl Windows {
    pub fn new() -> Windows {
        Windows
    }
}

impl Simulate<SetTo<common::MouseButton, bool>> for Windows {
    fn simulate(&mut self, simulatable: SetTo<common::MouseButton, bool>) {
        let SetTo {
            input: button,
            to: is_down,
        } = simulatable;
        if is_down {
            inner::mouse_button_down(button)
        } else {
            inner::mouse_button_up(button)
        }
    }
}

impl Simulate<SetTo<common::MousePosition, (i32, i32)>> for Windows {
    fn simulate(&mut self, simulatable: SetTo<common::MousePosition, (i32, i32)>) {
        let SetTo {
            input: _,
            to: position,
        } = simulatable;
        inner::virtual_desktop_normalized_mouse_move_to(position.0, position.1);
    }
}

impl Simulate<ChangeBy<common::MousePosition, (i32, i32)>> for Windows {
    fn simulate(&mut self, simulatable: ChangeBy<common::MousePosition, (i32, i32)>) {
        let ChangeBy { input: _, by } = simulatable;
        inner::virtual_desktop_normalized_mouse_move_by(by.0, by.1);
    }
}

impl Simulate<ChangeBy<common::MouseScroll, (i32, i32)>> for Windows {
    fn simulate(&mut self, simulatable: ChangeBy<common::MouseScroll, (i32, i32)>) {
        let ChangeBy { input: _, by } = simulatable;
        inner::mouse_scroll(by.0, by.1)
    }
}
