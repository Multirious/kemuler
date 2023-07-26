#![allow(unused)]

use std::mem::size_of;
use windows::{
    self,
    Win32::UI::{
        Input::KeyboardAndMouse,
        WindowsAndMessaging::{self, GetSystemMetrics},
    },
};

use crate::inputs::common::MouseButton;

// Thanks solution from https://stackoverflow.com/questions/35138778/sending-keys-to-a-directx-game

#[allow(non_snake_case)]
enum WindowsSendInputEnum {
    Keyboard {
        wVk: KeyboardAndMouse::VIRTUAL_KEY,
        wScan: u16,
        dwFlags: KeyboardAndMouse::KEYBD_EVENT_FLAGS,
        time: u32,
        dwExtraInfo: usize,
    },
    Mouse {
        dx: i32,
        dy: i32,
        mouseData: i32,
        dwFlags: KeyboardAndMouse::MOUSE_EVENT_FLAGS,
        time: u32,
        dwExtraInfo: usize,
    },
    #[allow(unused)]
    Hardware {
        uMsg: u32,
        wParamL: u16,
        wParamH: u16,
    },
}

impl WindowsSendInputEnum {
    #[rustfmt::skip]
    pub fn into_windows(self) -> KeyboardAndMouse::INPUT {
        let (a, b) = match self {
            WindowsSendInputEnum::Keyboard { wVk, wScan, dwFlags, time, dwExtraInfo, } => (
                KeyboardAndMouse::INPUT_KEYBOARD,
                KeyboardAndMouse::INPUT_0 {
                    ki: KeyboardAndMouse::KEYBDINPUT { wVk, wScan, dwFlags, time, dwExtraInfo, },
                },
            ),
            WindowsSendInputEnum::Mouse { dx, dy, mouseData, dwFlags, time, dwExtraInfo, } => (
                KeyboardAndMouse::INPUT_MOUSE,
                KeyboardAndMouse::INPUT_0 {
                    mi: KeyboardAndMouse::MOUSEINPUT { dx, dy, mouseData, dwFlags, time, dwExtraInfo, },
                },
            ),
            WindowsSendInputEnum::Hardware { uMsg, wParamL, wParamH, } => (
                KeyboardAndMouse::INPUT_HARDWARE,
                KeyboardAndMouse::INPUT_0 {
                    hi: KeyboardAndMouse::HARDWAREINPUT { uMsg, wParamL, wParamH, },
                },
            ),
        };
        KeyboardAndMouse::INPUT { r#type: a, Anonymous: b }
    }
}

macro_rules! virtual_key_code_enum {
    ($($variant:ident)*) => {
        #[allow(non_camel_case_types)]
        #[derive(Debug, Clone, Copy)]
        pub enum VirtualKeyCode {
            $(
                $variant,
            )*
        }

        impl VirtualKeyCode {
            pub fn code(&self) -> KeyboardAndMouse::VIRTUAL_KEY {
                match self {
                    $(
                        VirtualKeyCode::$variant => KeyboardAndMouse::$variant,
                    )*
                }
            }
        }
    };
}

virtual_key_code_enum! {
    VK_0 VK_1 VK_2 VK_3 VK_4 VK_5 VK_6 VK_7 VK_8 VK_9
    VK_A VK_B VK_C VK_D VK_E VK_F VK_G VK_H VK_I VK_J
    VK_K VK_L VK_M VK_N VK_O VK_P VK_Q VK_R VK_S VK_T
    VK_U VK_V VK_W VK_X VK_Y VK_Z VK_ABNT_C1 VK_ABNT_C2
    VK_DBE_ALPHANUMERIC VK_DBE_CODEINPUT VK_DBE_DBCSCHAR
    VK_DBE_DETERMINESTRING VK_DBE_ENTERDLGCONVERSIONMODE
    VK_DBE_ENTERIMECONFIGMODE VK_DBE_ENTERWORDREGISTERMODE
    VK_DBE_FLUSHSTRING VK_DBE_HIRAGANA VK_DBE_KATAKANA
    VK_DBE_NOCODEINPUT VK_DBE_NOROMAN VK_DBE_ROMAN VK_DBE_SBCSCHAR
    VK__none_ VK_LBUTTON VK_RBUTTON VK_CANCEL VK_MBUTTON VK_XBUTTON1
    VK_XBUTTON2 VK_BACK VK_TAB VK_CLEAR VK_RETURN VK_SHIFT VK_CONTROL
    VK_MENU VK_PAUSE VK_CAPITAL VK_KANA VK_HANGEUL VK_HANGUL VK_IME_ON
    VK_JUNJA VK_FINAL VK_HANJA VK_KANJI VK_IME_OFF VK_ESCAPE VK_CONVERT
    VK_NONCONVERT VK_ACCEPT VK_MODECHANGE VK_SPACE VK_PRIOR VK_NEXT
    VK_END VK_HOME VK_LEFT VK_UP VK_RIGHT VK_DOWN VK_SELECT VK_PRINT
    VK_EXECUTE VK_SNAPSHOT VK_INSERT VK_DELETE VK_HELP VK_LWIN
    VK_RWIN VK_APPS VK_SLEEP VK_NUMPAD0 VK_NUMPAD1 VK_NUMPAD2
    VK_NUMPAD3 VK_NUMPAD4 VK_NUMPAD5 VK_NUMPAD6 VK_NUMPAD7 VK_NUMPAD8 VK_NUMPAD9
    VK_MULTIPLY VK_ADD VK_SEPARATOR VK_SUBTRACT VK_DECIMAL VK_DIVIDE VK_F1
    VK_F2 VK_F3 VK_F4 VK_F5 VK_F6 VK_F7 VK_F8 VK_F9 VK_F10 VK_F11 VK_F12
    VK_F13 VK_F14 VK_F15 VK_F16 VK_F17 VK_F18 VK_F19 VK_F20 VK_F21 VK_F22
    VK_F23 VK_F24 VK_NAVIGATION_VIEW VK_NAVIGATION_MENU VK_NAVIGATION_UP
    VK_NAVIGATION_DOWN VK_NAVIGATION_LEFT VK_NAVIGATION_RIGHT VK_NAVIGATION_ACCEPT
    VK_NAVIGATION_CANCEL VK_NUMLOCK VK_SCROLL VK_OEM_NEC_EQUAL VK_OEM_FJ_JISHO
    VK_OEM_FJ_MASSHOU VK_OEM_FJ_TOUROKU VK_OEM_FJ_LOYA VK_OEM_FJ_ROYA VK_LSHIFT
    VK_RSHIFT VK_LCONTROL VK_RCONTROL VK_LMENU VK_RMENU VK_BROWSER_BACK
    VK_BROWSER_FORWARD VK_BROWSER_REFRESH VK_BROWSER_STOP VK_BROWSER_SEARCH
    VK_BROWSER_FAVORITES VK_BROWSER_HOME VK_VOLUME_MUTE VK_VOLUME_DOWN
    VK_VOLUME_UP VK_MEDIA_NEXT_TRACK VK_MEDIA_PREV_TRACK VK_MEDIA_STOP
    VK_MEDIA_PLAY_PAUSE VK_LAUNCH_MAIL VK_LAUNCH_MEDIA_SELECT VK_LAUNCH_APP1
    VK_LAUNCH_APP2 VK_OEM_1 VK_OEM_PLUS VK_OEM_COMMA VK_OEM_MINUS
    VK_OEM_PERIOD VK_OEM_2 VK_OEM_3 VK_GAMEPAD_A VK_GAMEPAD_B VK_GAMEPAD_X
    VK_GAMEPAD_Y VK_GAMEPAD_RIGHT_SHOULDER VK_GAMEPAD_LEFT_SHOULDER
    VK_GAMEPAD_LEFT_TRIGGER VK_GAMEPAD_RIGHT_TRIGGER VK_GAMEPAD_DPAD_UP
    VK_GAMEPAD_DPAD_DOWN VK_GAMEPAD_DPAD_LEFT VK_GAMEPAD_DPAD_RIGHT
    VK_GAMEPAD_MENU VK_GAMEPAD_VIEW VK_GAMEPAD_LEFT_THUMBSTICK_BUTTON
    VK_GAMEPAD_RIGHT_THUMBSTICK_BUTTON VK_GAMEPAD_LEFT_THUMBSTICK_UP
    VK_GAMEPAD_LEFT_THUMBSTICK_DOWN VK_GAMEPAD_LEFT_THUMBSTICK_RIGHT
    VK_GAMEPAD_LEFT_THUMBSTICK_LEFT VK_GAMEPAD_RIGHT_THUMBSTICK_UP
    VK_GAMEPAD_RIGHT_THUMBSTICK_DOWN VK_GAMEPAD_RIGHT_THUMBSTICK_RIGHT
    VK_GAMEPAD_RIGHT_THUMBSTICK_LEFT VK_OEM_4 VK_OEM_5 VK_OEM_6 VK_OEM_7
    VK_OEM_8 VK_OEM_AX VK_OEM_102 VK_ICO_HELP VK_ICO_00 VK_PROCESSKEY VK_ICO_CLEAR
    VK_PACKET VK_OEM_RESET VK_OEM_JUMP VK_OEM_PA1 VK_OEM_PA2 VK_OEM_PA3
    VK_OEM_WSCTRL VK_OEM_CUSEL VK_OEM_ATTN VK_OEM_FINISH VK_OEM_COPY
    VK_OEM_AUTO VK_OEM_ENLW VK_OEM_BACKTAB VK_ATTN VK_CRSEL VK_EXSEL
    VK_EREOF VK_PLAY VK_ZOOM VK_NONAME VK_PA1 VK_OEM_CLEAR
}

fn primary_screen_size() -> (i32, i32) {
    // SAFETY: calls has no dangerous side-effects
    let x = unsafe { GetSystemMetrics(WindowsAndMessaging::SM_CXSCREEN) };
    let y = unsafe { GetSystemMetrics(WindowsAndMessaging::SM_CYSCREEN) };
    (x, y)
}

fn virtual_screen_size() -> (i32, i32) {
    // SAFETY: calls has no dangerous side-effects
    let x = unsafe { GetSystemMetrics(WindowsAndMessaging::SM_CXVIRTUALSCREEN) };
    let y = unsafe { GetSystemMetrics(WindowsAndMessaging::SM_CYVIRTUALSCREEN) };
    (x, y)
}

fn get_cursor_position() -> Option<(i32, i32)> {
    let mut pos = windows::Win32::Foundation::POINT { x: 0, y: 0 };
    let res = unsafe { WindowsAndMessaging::GetCursorPos(&mut pos) };
    if res.as_bool() {
        Some((pos.x, pos.y))
    } else {
        None
    }
}

#[allow(unused)]
fn set_cursor_position(x: i32, y: i32) {
    unsafe { WindowsAndMessaging::SetCursorPos(x, y) };
}

fn send_input(inputs: &[KeyboardAndMouse::INPUT]) {
    unsafe { KeyboardAndMouse::SendInput(inputs, size_of::<KeyboardAndMouse::INPUT>() as i32) };
}

fn get_message_extra_info() -> usize {
    unsafe { WindowsAndMessaging::GetMessageExtraInfo() }.0 as usize
}

// TODO: Needed testing
pub fn mouse_scroll(x: i32, y: i32) {
    send_input(&[
        WindowsSendInputEnum::Mouse {
            dx: 0,
            dy: 0,
            mouseData: y,
            dwFlags: KeyboardAndMouse::MOUSEEVENTF_WHEEL,
            time: 0,
            dwExtraInfo: get_message_extra_info(),
        }
        .into_windows(),
        WindowsSendInputEnum::Mouse {
            dx: 0,
            dy: 0,
            mouseData: x,
            dwFlags: KeyboardAndMouse::MOUSEEVENTF_HWHEEL,
            time: 0,
            dwExtraInfo: get_message_extra_info(),
        }
        .into_windows(),
    ]);
}

// TODO: Needed testing
pub fn mouse_button_down(button: MouseButton) {
    let flag = match button {
        MouseButton::Left => KeyboardAndMouse::MOUSEEVENTF_LEFTDOWN,
        MouseButton::Middle => KeyboardAndMouse::MOUSEEVENTF_MIDDLEDOWN,
        MouseButton::Right => KeyboardAndMouse::MOUSEEVENTF_RIGHTDOWN,
        MouseButton::X1 | MouseButton::X2 => KeyboardAndMouse::MOUSEEVENTF_XDOWN,
    };
    let mouse_data = match button {
        MouseButton::X1 => WindowsAndMessaging::XBUTTON1,
        MouseButton::X2 => WindowsAndMessaging::XBUTTON2,
        MouseButton::Left | MouseButton::Middle | MouseButton::Right => 0,
    };
    send_input(&[WindowsSendInputEnum::Mouse {
        dx: 0,
        dy: 0,
        mouseData: mouse_data as i32,
        dwFlags: flag,
        time: 0,
        dwExtraInfo: get_message_extra_info(),
    }
    .into_windows()]);
}

// TODO: Needed testing
pub fn mouse_button_up(button: MouseButton) {
    let flag = match button {
        MouseButton::Left => KeyboardAndMouse::MOUSEEVENTF_LEFTUP,
        MouseButton::Middle => KeyboardAndMouse::MOUSEEVENTF_MIDDLEUP,
        MouseButton::Right => KeyboardAndMouse::MOUSEEVENTF_RIGHTUP,
        MouseButton::X1 | MouseButton::X2 => KeyboardAndMouse::MOUSEEVENTF_XUP,
    };
    let mouse_data = match button {
        MouseButton::X1 => WindowsAndMessaging::XBUTTON1,
        MouseButton::X2 => WindowsAndMessaging::XBUTTON2,
        MouseButton::Left | MouseButton::Middle | MouseButton::Right => 0,
    };
    send_input(&[WindowsSendInputEnum::Mouse {
        dx: 0,
        dy: 0,
        mouseData: mouse_data as i32,
        dwFlags: flag,
        time: 0,
        dwExtraInfo: get_message_extra_info(),
    }
    .into_windows()]);
}

/// `x` and `y` contain normalized absolute coordinates between 0 and 65,535.
/// The event procedure maps these coordinates onto the display surface.
/// Coordinate (0,0) maps onto the upper-left corner of the display surface;
/// coordinate (65535,65535) maps onto the lower-right corner.
/// In a multimonitor system, the coordinates map to the primary monitor.
pub fn mouse_move_to(x: i32, y: i32) {
    send_input(&[WindowsSendInputEnum::Mouse {
        dx: x,
        dy: y,
        mouseData: 0,
        dwFlags: KeyboardAndMouse::MOUSEEVENTF_MOVE | KeyboardAndMouse::MOUSEEVENTF_ABSOLUTE,
        time: 0,
        dwExtraInfo: get_message_extra_info(),
    }
    .into_windows()]);
}

// TODO: Needed testing
pub fn normalized_mouse_move_to(x: i32, y: i32) {
    let (screen_size_x, screen_size_y) = primary_screen_size();
    let x = x * 65535 / screen_size_x;
    let y = y * 65535 / screen_size_y;
    mouse_move_to(x, y);
}

/// same as [`mouse_move_to`] but the coordinates map to the entire virtual desktop.
fn virtual_desktop_mouse_move_to(x: i32, y: i32) {
    send_input(&[WindowsSendInputEnum::Mouse {
        dx: x,
        dy: y,
        mouseData: 0,
        dwFlags: KeyboardAndMouse::MOUSEEVENTF_MOVE
            | KeyboardAndMouse::MOUSEEVENTF_ABSOLUTE
            | KeyboardAndMouse::MOUSEEVENTF_VIRTUALDESK,
        time: 0,
        dwExtraInfo: get_message_extra_info(),
    }
    .into_windows()]);
}

// TODO: Needed testing
pub fn virtual_desktop_normalized_mouse_move_to(x: i32, y: i32) {
    let (screen_size_x, screen_size_y) = virtual_screen_size();
    let x = x * 65535 / screen_size_x;
    let y = y * 65535 / screen_size_y;
    virtual_desktop_mouse_move_to(x, y);
}

/// `x` and `y` specify movement relative to the previous mouse event (the last reported position).
/// Positive values mean the mouse moved right (or down);
/// negative values mean the mouse moved left (or up).
///
/// Relative mouse motion is subject to the effects of the mouse speed and the two-mouse threshold values.
/// A user sets these three values with the Pointer Speed slider of the Control Panel's Mouse Properties sheet.
/// You can obtain and set these values using the `SystemParametersInfo` function.
///
/// The system applies two tests to the specified relative mouse movement.
/// If the specified distance along either the x or y axis is greater than the first mouse threshold value,
/// and the mouse speed is not zero, the system doubles the distance.
/// If the specified distance along either the x or y axis is greater than the second mouse threshold value,
/// and the mouse speed is equal to two,
/// the system doubles the distance that resulted from applying the first threshold test.
/// It is thus possible for the system to multiply specified relative mouse movement along the x or y axis
// by up to four times.
///
/// from https://stackoverflow.com/questions/60268940/sendinput-mouse-movement-calculation
/// It is not worth it trying to normalize by mathing.
pub fn mouse_move_by(x: i32, y: i32) {
    send_input(&[WindowsSendInputEnum::Mouse {
        dx: x,
        dy: y,
        mouseData: 0,
        dwFlags: KeyboardAndMouse::MOUSEEVENTF_MOVE,
        time: 0,
        dwExtraInfo: get_message_extra_info(),
    }
    .into_windows()]);
}

/// return true if is successful
pub fn normalized_mouse_move_by(x: i32, y: i32) -> bool {
    let Some((current_x, current_y)) = get_cursor_position() else {
        return false;
    };
    normalized_mouse_move_to(current_x + x, current_y + y);
    true
}

/// return true if is successful
pub fn virtual_desktop_normalized_mouse_move_by(x: i32, y: i32) -> bool {
    let Some((current_x, current_y)) = get_cursor_position() else {
        return false;
    };
    virtual_desktop_normalized_mouse_move_to(current_x + x, current_y + y);
    true
}

pub fn virtual_key_down(code: VirtualKeyCode) {
    send_input(&[WindowsSendInputEnum::Keyboard {
        wVk: code.code(),
        wScan: 0,
        dwFlags: KeyboardAndMouse::KEYBD_EVENT_FLAGS::default(),
        time: 0,
        dwExtraInfo: get_message_extra_info(),
    }
    .into_windows()]);
}

pub fn virtual_key_up(code: VirtualKeyCode) {
    send_input(&[WindowsSendInputEnum::Keyboard {
        wVk: code.code(),
        wScan: 0,
        dwFlags: KeyboardAndMouse::KEYEVENTF_KEYUP,
        time: 0,
        dwExtraInfo: get_message_extra_info(),
    }
    .into_windows()]);
}

pub fn unicode_key_down(utf16_char: u16) {
    send_input(&[WindowsSendInputEnum::Keyboard {
        wVk: KeyboardAndMouse::VIRTUAL_KEY(0),
        wScan: utf16_char,
        dwFlags: KeyboardAndMouse::KEYEVENTF_UNICODE,
        time: 0,
        dwExtraInfo: get_message_extra_info(),
    }
    .into_windows()]);
}

pub fn unicode_key_up(utf16_char: u16) {
    send_input(&[WindowsSendInputEnum::Keyboard {
        wVk: KeyboardAndMouse::VIRTUAL_KEY(0),
        wScan: utf16_char,
        dwFlags: KeyboardAndMouse::KEYEVENTF_UNICODE | KeyboardAndMouse::KEYEVENTF_KEYUP,
        time: 0,
        dwExtraInfo: get_message_extra_info(),
    }
    .into_windows()]);
}
