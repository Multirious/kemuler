use crate::{
    combinator::{AndThen, Combine},
    inputs::common,
    simulatable::{ChangeBy, SetTo},
    simulator::Simulate,
};

pub mod inner;

pub enum VirtualKey {
    /// `VK_0`
    Num0, // VK_0
    /// `VK_1`
    Num1, // VK_1
    /// `VK_2`
    Num2, // VK_2
    /// `VK_3`
    Num3, // VK_3
    /// `VK_4`
    Num4, // VK_4
    /// `VK_5`
    Num5, // VK_5
    /// `VK_6`
    Num6, // VK_6
    /// `VK_7`
    Num7, // VK_7
    /// `VK_8`
    Num8, // VK_8
    /// `VK_9`
    Num9, // VK_9
    /// `VK_A`
    A, // VK_A
    /// `VK_B`
    B, // VK_B
    /// `VK_C`
    C, // VK_C
    /// `VK_D`
    D, // VK_D
    /// `VK_E`
    E, // VK_E
    /// `VK_F`
    F, // VK_F
    /// `VK_G`
    G, // VK_G
    /// `VK_H`
    H, // VK_H
    /// `VK_I`
    I, // VK_I
    /// `VK_J`
    J, // VK_J
    /// `VK_K`
    K, // VK_K
    /// `VK_L`
    L, // VK_L
    /// `VK_M`
    M, // VK_M
    /// `VK_N`
    N, // VK_N
    /// `VK_O`
    O, // VK_O
    /// `VK_P`
    P, // VK_P
    /// `VK_Q`
    Q, // VK_Q
    /// `VK_R`
    R, // VK_R
    /// `VK_S`
    S, // VK_S
    /// `VK_T`
    T, // VK_T
    /// `VK_U`
    U, // VK_U
    /// `VK_V`
    V, // VK_V
    /// `VK_W`
    W, // VK_W
    /// `VK_X`
    X, // VK_X
    /// `VK_Y`
    Y, // VK_Y
    /// `VK_Z`
    Z, // VK_Z
    /// `VK_ABNT_C1`
    AbntC1, // VK_ABNT_C1
    /// `VK_ABNT_C2`
    AbntC2, // VK_ABNT_C2
    /// `VK_DBE_ALPHANUMERIC`
    DbeAlphanumeric, // VK_DBE_ALPHANUMERIC
    /// `VK_DBE_CODEINPUT`
    DbeCodeInput, // VK_DBE_CODEINPUT
    /// `VK_DBE_DBCSCHAR`
    DbeDbcsChar, // VK_DBE_DBCSCHAR
    /// `VK_DBE_DETERMINESTRING`
    DbeDetermineString, // VK_DBE_DETERMINESTRING
    /// `VK_DBE_ENTERDLGCONVERSIONMODE`
    DbeEnterDlgConversionMode, // VK_DBE_ENTERDLGCONVERSIONMODE
    /// `VK_DBE_ENTERIMECONFIGMODE`
    DbeEnterImeConfigMode, // VK_DBE_ENTERIMECONFIGMODE
    /// `VK_DBE_ENTERWORDREGISTERMODE`
    DbeEnterWordRegisterMode, // VK_DBE_ENTERWORDREGISTERMODE
    /// `VK_DBE_FLUSHSTRING`
    DbeFlushString, // VK_DBE_FLUSHSTRING
    /// `VK_DBE_HIRAGANA`
    DbeHiragana, // VK_DBE_HIRAGANA
    /// `VK_DBE_KATAKANA`
    DbeKatakana, // VK_DBE_KATAKANA
    /// `VK_DBE_NOCODEINPUT`
    DbeNoCodeInput, // VK_DBE_NOCODEINPUT
    /// `VK_DBE_NOROMAN`
    DbeNoRoman, // VK_DBE_NOROMAN
    /// `VK_DBE_ROMAN`
    DbeRoman, // VK_DBE_ROMAN
    /// `VK_DBE_SBCSCHAR`
    DbeSbcsChar, // VK_DBE_SBCSCHAR
    /// `VK__none_`
    _none_, // VK__none_
    /// Left mouse button
    /// `VK_LBUTTON`
    LButton, // VK_LBUTTON
    /// Right mouse button
    /// `VK_RBUTTON`
    RButton, // VK_RBUTTON
    /// Control-break processing
    /// `VK_CANCEL`
    Cancel, // VK_CANCEL
    /// Middle mouse button (three-button mouse)
    /// `VK_MBUTTON`
    MButton, // VK_MBUTTON
    /// X1 mouse button
    /// `VK_XBUTTON1`
    XButton1, // VK_XBUTTON1
    /// X2 mouse button
    /// `VK_XBUTTON2`
    XButton2, // VK_XBUTTON2
    /// BACKSPACE key
    /// `VK_BACK`
    Back, // VK_BACK
    /// TAB key
    /// `VK_TAB`
    Tab, // VK_TAB
    /// CLEAR key
    /// `VK_CLEAR`
    Clear, // VK_CLEAR
    /// ENTER key
    /// `VK_RETURN`
    Return, // VK_RETURN
    /// SHIFT key
    /// `VK_SHIFT`
    Shift, // VK_SHIFT
    /// CTRL key
    /// `VK_CONTROL`
    Control, // VK_CONTROL
    /// ALT key
    /// `VK_MENU`
    Menu, // VK_MENU
    /// PAUSE key
    /// `VK_PAUSE`
    Pause, // VK_PAUSE
    /// CAPS LOCK key
    /// `VK_CAPITAL`
    Capital, // VK_CAPITAL
    /// IME Kana mode
    /// `VK_KANA`
    Kana, // VK_KANA
    /// IME Hanguel mode (maintained for compatibility; use VK_HANGUL)
    /// `VK_HANGEUL`
    Hangeul, // VK_HANGEUL
    /// IME Hangul mode
    /// `VK_HANGUL`
    Hangul, // VK_HANGUL
    /// IME On
    /// `VK_IME_ON`
    ImeOn, // VK_IME_ON
    /// IME Junja mode
    /// `VK_JUNJA`
    Junja, // VK_JUNJA
    /// IME final mode
    /// `VK_FINAL`
    Final, // VK_FINAL
    /// IME Hanja mode
    /// `VK_HANJA`
    Hanja, // VK_HANJA
    /// IME Kanji mode
    /// `VK_KANJI`
    Kanji, // VK_KANJI
    /// IME Off
    /// `VK_IME_OFF`
    ImeOff, // VK_IME_OFF
    /// ESC key
    /// `VK_ESCAPE`
    Escape, // VK_ESCAPE
    /// IME convert
    /// `VK_CONVERT`
    Convert, // VK_CONVERT
    /// IME nonconvert
    /// `VK_NONCONVERT`
    NonConvert, // VK_NONCONVERT
    /// IME accept
    /// `VK_ACCEPT`
    Accept, // VK_ACCEPT
    /// IME mode change request
    /// `VK_MODECHANGE`
    ModeChange, // VK_MODECHANGE
    /// SPACEBAR
    /// `VK_SPACE`
    Space, // VK_SPACE
    /// PAGE UP key
    /// `VK_PRIOR`
    Prior, // VK_PRIOR
    /// PAGE DOWN key
    /// `VK_NEXT`
    Next, // VK_NEXT
    /// END key
    /// `VK_END`
    End, // VK_END
    /// HOME key
    /// `VK_HOME`
    Home, // VK_HOME
    /// LEFT ARROW key
    /// `VK_LEFT`
    Left, // VK_LEFT
    /// UP ARROW key
    /// `VK_UP`
    Up, // VK_UP
    /// RIGHT ARROW key
    /// `VK_RIGHT`
    Right, // VK_RIGHT
    /// DOWN ARROW key
    /// `VK_DOWN`
    Down, // VK_DOWN
    /// SELECT key
    /// `VK_SELECT`
    Select, // VK_SELECT
    /// PRINT key
    /// `VK_PRINT`
    Print, // VK_PRINT
    /// EXECUTE key
    /// `VK_EXECUTE`
    Execute, // VK_EXECUTE
    /// PRINT SCREEN key
    /// `VK_SNAPSHOT`
    Snapshot, // VK_SNAPSHOT
    /// INS key
    /// `VK_INSERT`
    Insert, // VK_INSERT
    /// DEL key
    /// `VK_DELETE`
    Delete, // VK_DELETE
    /// HELP key
    /// `VK_HELP`
    Help, // VK_HELP
    /// Left Windows key (Natural keyboard)
    /// `VK_LWIN`
    LWin, // VK_LWIN
    /// Right Windows key (Natural keyboard)
    /// `VK_RWIN`
    RWin, // VK_RWIN
    /// Applications key (Natural keyboard)
    /// `VK_APPS`
    Apps, // VK_APPS
    /// Computer Sleep key
    /// `VK_SLEEP`
    Sleep, // VK_SLEEP
    /// Numeric keypad 0 key
    /// `VK_NUMPAD0`
    NumPad0, // VK_NUMPAD0
    /// Numeric keypad 1 key
    /// `VK_NUMPAD1`
    NumPad1, // VK_NUMPAD1
    /// Numeric keypad 2 key
    /// `VK_NUMPAD2`
    NumPad2, // VK_NUMPAD2
    /// Numeric keypad 3 key
    /// `VK_NUMPAD3`
    NumPad3, // VK_NUMPAD3
    /// Numeric keypad 4 key
    /// `VK_NUMPAD4`
    NumPad4, // VK_NUMPAD4
    /// Numeric keypad 5 key
    /// `VK_NUMPAD5`
    NumPad5, // VK_NUMPAD5
    /// Numeric keypad 6 key
    /// `VK_NUMPAD6`
    NumPad6, // VK_NUMPAD6
    /// Numeric keypad 7 key
    /// `VK_NUMPAD7`
    NumPad7, // VK_NUMPAD7
    /// Numeric keypad 8 key
    /// `VK_NUMPAD8`
    NumPad8, // VK_NUMPAD8
    /// Numeric keypad 9 key
    /// `VK_NUMPAD9`
    NumPad9, // VK_NUMPAD9
    /// Multiply key
    /// `VK_MULTIPLY`
    Multiply, // VK_MULTIPLY
    /// Add key
    /// `VK_ADD`
    Add, // VK_ADD
    /// Separator key
    /// `VK_SEPARATOR`
    Separator, // VK_SEPARATOR
    /// Subtract key
    /// `VK_SUBTRACT`
    Subtract, // VK_SUBTRACT
    /// Decimal key
    /// `VK_DECIMAL`
    Decimal, // VK_DECIMAL
    /// Divide key
    /// `VK_DIVIDE`
    Divide, // VK_DIVIDE
    /// F1 key
    /// `VK_F1`
    F1, // VK_F1
    /// F2 key
    /// `VK_F2`
    F2, // VK_F2
    /// F3 key
    /// `VK_F3`
    F3, // VK_F3
    /// F4 key
    /// `VK_F4`
    F4, // VK_F4
    /// F5 key
    /// `VK_F5`
    F5, // VK_F5
    /// F6 key
    /// `VK_F6`
    F6, // VK_F6
    /// F7 key
    /// `VK_F7`
    F7, // VK_F7
    /// F8 key
    /// `VK_F8`
    F8, // VK_F8
    /// F9 key
    /// `VK_F9`
    F9, // VK_F9
    /// F10 key
    /// `VK_F10`
    F10, // VK_F10
    /// F11 key
    /// `VK_F11`
    F11, // VK_F11
    /// F12 key
    /// `VK_F12`
    F12, // VK_F12
    /// F13 key
    /// `VK_F13`
    F13, // VK_F13
    /// F14 key
    /// `VK_F14`
    F14, // VK_F14
    /// F15 key
    /// `VK_F15`
    F15, // VK_F15
    /// F16 key
    /// `VK_F16`
    F16, // VK_F16
    /// F17 key
    /// `VK_F17`
    F17, // VK_F17
    /// F18 key
    /// `VK_F18`
    F18, // VK_F18
    /// F19 key
    /// `VK_F19`
    F19, // VK_F19
    /// F20 key
    /// `VK_F20`
    F20, // VK_F20
    /// F21 key
    /// `VK_F21`
    F21, // VK_F21
    /// F22 key
    /// `VK_F22`
    F22, // VK_F22
    /// F23 key
    /// `VK_F23`
    F23, // VK_F23
    /// F24 key
    /// `VK_F24`
    F24, // VK_F24
    /// `VK_NAVIGATION_VIEW`
    NavigationView, // VK_NAVIGATION_VIEW
    /// `VK_NAVIGATION_MENU`
    NavigationMenu, // VK_NAVIGATION_MENU
    /// `VK_NAVIGATION_UP`
    NavigationUP, // VK_NAVIGATION_UP
    /// `VK_NAVIGATION_DOWN`
    NavigationDown, // VK_NAVIGATION_DOWN
    /// `VK_NAVIGATION_LEFT`
    NavigationLeft, // VK_NAVIGATION_LEFT
    /// `VK_NAVIGATION_RIGHT`
    NavigationRight, // VK_NAVIGATION_RIGHT
    /// `VK_NAVIGATION_ACCEPT`
    NavigationAccept, // VK_NAVIGATION_ACCEPT
    /// `VK_NAVIGATION_CANCEL`
    NavigationCancel, // VK_NAVIGATION_CANCEL
    /// NUM LOCK key
    /// `VK_NUMLOCK`
    NumLock, // VK_NUMLOCK
    /// SCROLL LOCK key
    /// `VK_SCROLL`
    Scroll, // VK_SCROLL
    /// `VK_OEM_NEC_EQUAL`
    OemNecEqual, // VK_OEM_NEC_EQUAL
    /// `VK_OEM_FJ_JISHO`
    OemFjJisho, // VK_OEM_FJ_JISHO
    /// `VK_OEM_FJ_MASSHOU`
    OemFjMasshou, // VK_OEM_FJ_MASSHOU
    /// `VK_OEM_FJ_TOUROKU`
    OemFjTouroku, // VK_OEM_FJ_TOUROKU
    /// `VK_OEM_FJ_LOYA`
    OemFjLoya, // VK_OEM_FJ_LOYA
    /// `VK_OEM_FJ_ROYA`
    OemFjRoya, // VK_OEM_FJ_ROYA
    /// Left SHIFT key
    /// `VK_LSHIFT`
    LShift, // VK_LSHIFT
    /// Right SHIFT key
    /// `VK_RSHIFT`
    RShift, // VK_RSHIFT
    /// Left CONTROL key
    /// `VK_LCONTROL`
    LControl, // VK_LCONTROL
    /// Right CONTROL key
    /// `VK_RCONTROL`
    RControl, // VK_RCONTROL
    /// Left ALT key
    /// `VK_LMENU`
    LMenu, // VK_LMENU
    /// Right ALT key
    /// `VK_RMENU`
    RMenu, // VK_RMENU
    /// Browser Back key
    /// `VK_BROWSER_BACK`
    BrowserBack, // VK_BROWSER_BACK
    /// Browser Forward key
    /// `VK_BROWSER_FORWARD`
    BrowserForward, // VK_BROWSER_FORWARD
    /// Browser Refresh key
    /// `VK_BROWSER_REFRESH`
    BrowserRefresh, // VK_BROWSER_REFRESH
    /// Browser Stop key
    /// `VK_BROWSER_STOP`
    BrowserStop, // VK_BROWSER_STOP
    /// Browser Search key
    /// `VK_BROWSER_SEARCH`
    BrowserSearch, // VK_BROWSER_SEARCH
    /// Browser Favorites key
    /// `VK_BROWSER_FAVORITES`
    BrowserFavorites, // VK_BROWSER_FAVORITES
    /// Browser Start and Home key
    /// `VK_BROWSER_HOME`
    BrowserHome, // VK_BROWSER_HOME
    /// Volume Mute key
    /// `VK_VOLUME_MUTE`
    VolumeMute, // VK_VOLUME_MUTE
    /// Volume Down key
    /// `VK_VOLUME_DOWN`
    VolumeDown, // VK_VOLUME_DOWN
    /// Volume Up key
    /// `VK_VOLUME_UP`
    VolumeUp, // VK_VOLUME_UP
    /// Next Track key
    /// `VK_MEDIA_NEXT_TRACK`
    MediaNextTrack, // VK_MEDIA_NEXT_TRACK
    /// Previous Track key
    /// `VK_MEDIA_PREV_TRACK`
    MediaPrevTrack, // VK_MEDIA_PREV_TRACK
    /// Stop Media key
    /// `VK_MEDIA_STOP`
    MediaStop, // VK_MEDIA_STOP
    /// Play/Pause Media key
    /// `VK_MEDIA_PLAY_PAUSE`
    MediaPlayPause, // VK_MEDIA_PLAY_PAUSE
    /// Start Mail key
    /// `VK_LAUNCH_MAIL`
    LaunchMail, // VK_LAUNCH_MAIL
    /// Select Media key
    /// `VK_LAUNCH_MEDIA_SELECT`
    LaunchMediaSelect, // VK_LAUNCH_MEDIA_SELECT
    /// Start Application 1 key
    /// `VK_LAUNCH_APP1`
    LaunchApp1, // VK_LAUNCH_APP1
    /// Start Application 2 key
    /// `VK_LAUNCH_APP2`
    LaunchApp2, // VK_LAUNCH_APP2
    /// For any country/region, the '+' key
    /// `VK_OEM_1`
    Oem1, // VK_OEM_1
    /// For any country/region, the ',' key
    /// `VK_OEM_PLUS`
    OemPlus, // VK_OEM_PLUS
    /// For any country/region, the '-' key
    /// `VK_OEM_COMMA`
    OemComma, // VK_OEM_COMMA
    /// For any country/region, the '.' key
    /// `VK_OEM_MINUS`
    OemMinus, // VK_OEM_MINUS
    /// Used for miscellaneous characters; it can vary by keyboard. For the US standard keyboard, the '/?' key
    /// `VK_OEM_PERIOD`
    OemPeriod, // VK_OEM_PERIOD
    /// Used for miscellaneous characters; it can vary by keyboard. For the US standard keyboard, the '`~' key
    /// `VK_OEM_2`
    Oem2, // VK_OEM_2
    /// Used for miscellaneous characters; it can vary by keyboard. For the US standard keyboard, the '`~' key
    /// `VK_OEM_3`
    Oem3, // VK_OEM_3
    /// `VK_GAMEPAD_A`
    GamepadA, // VK_GAMEPAD_A
    /// `VK_GAMEPAD_B`
    GamepadB, // VK_GAMEPAD_B
    /// `VK_GAMEPAD_X`
    GamepadX, // VK_GAMEPAD_X
    /// `VK_GAMEPAD_Y`
    GamepadY, // VK_GAMEPAD_Y
    /// `VK_GAMEPAD_RIGHT_SHOULDER`
    GamepadRightShoulder, // VK_GAMEPAD_RIGHT_SHOULDER
    /// `VK_GAMEPAD_LEFT_SHOULDER`
    GamepadLeftShoulder, // VK_GAMEPAD_LEFT_SHOULDER
    /// `VK_GAMEPAD_LEFT_TRIGGER`
    GamepadLeftTrigger, // VK_GAMEPAD_LEFT_TRIGGER
    /// `VK_GAMEPAD_RIGHT_TRIGGER`
    GamepadRightTrigger, // VK_GAMEPAD_RIGHT_TRIGGER
    /// `VK_GAMEPAD_DPAD_UP`
    GamepadDPadUp, // VK_GAMEPAD_DPAD_UP
    /// `VK_GAMEPAD_DPAD_DOWN`
    GamepadDPadDown, // VK_GAMEPAD_DPAD_DOWN
    /// `VK_GAMEPAD_DPAD_LEFT`
    GamepadDPadLeft, // VK_GAMEPAD_DPAD_LEFT
    /// `VK_GAMEPAD_DPAD_RIGHT`
    GamepadDPadRight, // VK_GAMEPAD_DPAD_RIGHT
    /// `VK_GAMEPAD_MENU`
    GamepadMenu, // VK_GAMEPAD_MENU
    /// `VK_GAMEPAD_VIEW`
    GamepadView, // VK_GAMEPAD_VIEW
    /// `VK_GAMEPAD_LEFT_THUMBSTICK_BUTTON`
    GamepadLeftThumbStickButton, // VK_GAMEPAD_LEFT_THUMBSTICK_BUTTON
    /// `VK_GAMEPAD_RIGHT_THUMBSTICK_BUTTON`
    GamepadRightThumbStickButton, // VK_GAMEPAD_RIGHT_THUMBSTICK_BUTTON
    /// `VK_GAMEPAD_LEFT_THUMBSTICK_UP`
    GamepadLeftThumbStickUp, // VK_GAMEPAD_LEFT_THUMBSTICK_UP
    /// `VK_GAMEPAD_LEFT_THUMBSTICK_DOWN`
    GamepadLeftThumbStickDown, // VK_GAMEPAD_LEFT_THUMBSTICK_DOWN
    /// `VK_GAMEPAD_LEFT_THUMBSTICK_RIGHT`
    GamepadLeftThumbStickRight, // VK_GAMEPAD_LEFT_THUMBSTICK_RIGHT
    /// `VK_GAMEPAD_LEFT_THUMBSTICK_LEFT`
    GamepadLeftThumbStickLeft, // VK_GAMEPAD_LEFT_THUMBSTICK_LEFT
    /// `VK_GAMEPAD_RIGHT_THUMBSTICK_UP`
    GamepadRightThumbStickUp, // VK_GAMEPAD_RIGHT_THUMBSTICK_UP
    /// `VK_GAMEPAD_RIGHT_THUMBSTICK_DOWN`
    GamepadRightThumbStickDown, // VK_GAMEPAD_RIGHT_THUMBSTICK_DOWN
    /// `VK_GAMEPAD_RIGHT_THUMBSTICK_RIGHT`
    GamepadRightThumbStickRight, // VK_GAMEPAD_RIGHT_THUMBSTICK_RIGHT
    /// `VK_GAMEPAD_RIGHT_THUMBSTICK_LEFT`
    GamepadRightThumbStickLeft, // VK_GAMEPAD_RIGHT_THUMBSTICK_LEFT
    /// Used for miscellaneous characters; it can vary by keyboard. For the US standard keyboard, the '[{' key
    /// `VK_OEM_4`
    Oem4, // VK_OEM_4
    /// Used for miscellaneous characters; it can vary by keyboard. For the US standard keyboard, the '\|' key
    /// `VK_OEM_5`
    Oem5, // VK_OEM_5
    /// Used for miscellaneous characters; it can vary by keyboard. For the US standard keyboard, the ']}' key
    /// `VK_OEM_6`
    Oem6, // VK_OEM_6
    /// Used for miscellaneous characters; it can vary by keyboard. For the US standard keyboard, the 'single-quote/double-quote' key
    /// `VK_OEM_7`
    Oem7, // VK_OEM_7
    /// Used for miscellaneous characters; it can vary by keyboard.
    /// `VK_OEM_8`
    Oem8, // VK_OEM_8
    /// `VK_OEM_AX`
    OemAx, // VK_OEM_AX
    /// The `<>` keys on the US standard keyboard, or the `\\|` key on the non-US 102-key keyboard
    /// `VK_OEM_102`
    Oem102, // VK_OEM_102
    /// `VK_ICO_HELP`
    IcoHelp, // VK_ICO_HELP
    /// `VK_ICO_00`
    Ico00, // VK_ICO_00
    /// IME PROCESS key
    /// `VK_PROCESSKEY`
    ProcessKey, // VK_PROCESSKEY
    /// `VK_ICO_CLEAR`
    IcoClear, // VK_ICO_CLEAR
    /// Used to pass Unicode characters as if they were keystrokes. The VK_PACKET key is the low word of a 32-bit Virtual Key value used for non-keyboard input methods. For more information, see Remark in <a href="/en-us/windows/win32/api/winuser/ns-winuser-keybdinput" data-linktype="absolute-path">KEYBDINPUT</a>, <a href="/en-us/windows/win32/api/winuser/nf-winuser-sendinput" data-linktype="absolute-path">SendInput</a>, <a href="wm-keydown" data-linktype="relative-path">WM_KEYDOWN</a>, and <a href="wm-keyup" data-linktype="relative-path">WM_KEYUP</a>
    /// `VK_PACKET`
    Packet, // VK_PACKET
    /// `VK_OEM_RESET`
    OemReset, // VK_OEM_RESET
    /// `VK_OEM_JUMP`
    OemJump, // VK_OEM_JUMP
    /// `VK_OEM_PA1`
    OemPa1, // VK_OEM_PA1
    /// `VK_OEM_PA2`
    OemPa2, // VK_OEM_PA2
    /// `VK_OEM_PA3`
    OemPa3, // VK_OEM_PA3
    /// `VK_OEM_WSCTRL`
    OemWsctrl, // VK_OEM_WSCTRL
    /// `VK_OEM_CUSEL`
    OemCusel, // VK_OEM_CUSEL
    /// `VK_OEM_ATTN`
    OemAttn, // VK_OEM_ATTN
    /// `VK_OEM_FINISH`
    OemFinish, // VK_OEM_FINISH
    /// `VK_OEM_COPY`
    OemCopy, // VK_OEM_COPY
    /// `VK_OEM_AUTO`
    OemAuto, // VK_OEM_AUTO
    /// `VK_OEM_ENLW`
    OemEnlw, // VK_OEM_ENLW
    /// `VK_OEM_BACKTAB`
    OemBacktab, // VK_OEM_BACKTAB
    /// Attn key
    /// `VK_ATTN`
    Attn, // VK_ATTN
    /// CrSel key
    /// `VK_CRSEL`
    CrSel, // VK_CRSEL
    /// ExSel key
    /// `VK_EXSEL`
    ExSel, // VK_EXSEL
    /// Erase EOF key
    /// `VK_EREOF`
    ErEof, // VK_EREOF
    /// Play key
    /// `VK_PLAY`
    Play, // VK_PLAY
    /// Zoom key
    /// `VK_ZOOM`
    Zoom, // VK_ZOOM
    /// Reserved
    /// `VK_NONAME`
    Noname, // VK_NONAME
    /// PA1 key
    /// `VK_PA1`
    Pa1, // VK_PA1
    /// Clear key
    /// `VK_OEM_CLEAR`
    OemClear, // VK_OEM_CLEAR
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
        inner::virtual_desktop_normalized_mouse_move_to(position.0, position.1)
    }
}

impl Simulate<ChangeBy<common::MousePosition, (i32, i32)>> for Windows {
    fn simulate(&mut self, simulatable: ChangeBy<common::MousePosition, (i32, i32)>) {
        let ChangeBy { input: _, by } = simulatable;
        inner::normalized_mouse_move_by(by.0, by.1);
    }
}

impl Simulate<ChangeBy<common::MouseScroll, (i32, i32)>> for Windows {
    fn simulate(&mut self, simulatable: ChangeBy<common::MouseScroll, (i32, i32)>) {
        let ChangeBy { input: _, by } = simulatable;
        inner::mouse_scroll(by.0, by.1)
    }
}
