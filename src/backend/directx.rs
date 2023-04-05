use super::*;
use windows::{
    self,
    Win32::UI::{
        Input::KeyboardAndMouse::{SendInput, INPUT, INPUT_0, INPUT_MOUSE, INPUT_TYPE, MOUSEINPUT},
        WindowsAndMessaging::GetMessageExtraInfo,
    },
};

// Thanks solutation from https://stackoverflow.com/questions/35138778/sending-keys-to-a-directx-game

#[test]
fn test() {
    unsafe {
        SendInput(
            &[INPUT {
                r#type: INPUT_MOUSE,
                Anonymous: INPUT_0 {
                    mi: MOUSEINPUT {
                        dx: todo!(),
                        dy: todo!(),
                        mouseData: todo!(),
                        dwFlags: todo!(),
                        time: todo!(),
                        dwExtraInfo: todo!(),
                    },
                },
            }],
            0,
        );
    }
}
