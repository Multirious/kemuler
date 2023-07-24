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
fn inner_mouse_move_to(x: i32, y: i32) {
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
pub fn mouse_move_to(x: i32, y: i32) {
    let (screen_size_x, screen_size_y) = primary_screen_size();
    let x = x * 65535 / screen_size_x;
    let y = y * 65535 / screen_size_y;
    inner_mouse_move_to(x, y);
}

/// same as [`mouse_move_to`] but the coordinates map to the entire virtual desktop.
fn inner_mouse_move_to_virtual_desktop(x: i32, y: i32) {
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
pub fn mouse_move_to_virtual_desktop(x: i32, y: i32) {
    let (screen_size_x, screen_size_y) = virtual_screen_size();
    let x = x * 65535 / screen_size_x;
    let y = y * 65535 / screen_size_y;
    inner_mouse_move_to_virtual_desktop(x, y);
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
/// by up to four times.
// TODO: Need testing
pub fn mouse_move_by(x: i32, y: i32) {
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
