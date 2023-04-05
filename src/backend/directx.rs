use super::*;
use std::mem::size_of;
use windows::{
    self,
    Win32::UI::{Input::KeyboardAndMouse, WindowsAndMessaging},
};

// Thanks solutation from https://stackoverflow.com/questions/35138778/sending-keys-to-a-directx-game

pub struct General;

impl MouseBackend for General {
    fn mouse_move_to(&self, x: i32, y: i32) {
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

    fn mouse_move_by(&self, x: i32, y: i32) {
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

    fn mouse_scroll(&self, x: i32, y: i32) {
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

    fn mouse_down(&self, button: MouseButton) {
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

    fn mouse_up(&self, button: MouseButton) {
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
}

impl KeyboardBackend<KeyCommon> for General {
    fn key_down(&self, key: KeyCommon) {
        todo!()
    }

    fn key_up(&self, key: KeyCommon) {
        todo!()
    }
}

impl KeyboardBackend<KeyLayout> for General {
    fn key_down(&self, key: KeyLayout) {
        todo!()
    }

    fn key_up(&self, key: KeyLayout) {
        todo!()
    }
}

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

fn send_input(inputs: &[KeyboardAndMouse::INPUT]) {
    unsafe { KeyboardAndMouse::SendInput(inputs, size_of::<KeyboardAndMouse::INPUT>() as i32) };
}

fn get_message_extra_info() -> usize {
    unsafe { WindowsAndMessaging::GetMessageExtraInfo() }.0 as usize
}
