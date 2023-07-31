use crate::{
    combinator::{Combine, Sequence},
    common_inputs,
    input_event::{ChangeBy, SetTo},
    simulator::Simulate,
};

mod virtual_key;
pub use virtual_key::VirtualKey;

mod inner;

impl VirtualKey {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum MouseButton {
    Left,
    Middle,
    Right,
    /// Also known as foward button
    #[doc(alias = "Forward")]
    X1,
    /// Also known as backward button
    #[doc(alias = "Backward")]
    #[doc(alias = "Back")]
    X2,
}

impl MouseButton {
    /// Set this button.
    /// This is a convenience shorthand for
    /// ```
    /// # use kemuler::{prelude::*, input_event::*};
    /// # let this = MouseButton::Left;
    /// # let to = true;
    /// # let output =
    /// SetTo { input: this, to: to }
    /// # ;
    /// # assert_eq!(this.set_to(to), output);
    /// ```
    pub fn set_to(self, to: bool) -> SetTo<Self, bool> {
        SetTo { input: self, to }
    }

    /// Press the button.
    /// This is a convenience shorthand for
    /// ```
    /// # use kemuler::{prelude::*, input_event::*};
    /// # let this = MouseButton::Left;
    /// # let output =
    /// SetTo { input: this, to: true }
    /// # ;
    /// # assert_eq!(this.down(), output);
    /// ```
    pub fn down(self) -> SetTo<Self, bool> {
        self.set_to(true)
    }

    /// Release the button.
    /// This is a convenience shorthand for
    /// ```
    /// # use kemuler::{prelude::*, input_event::*};
    /// # let this = MouseButton::Left;
    /// # let output =
    /// SetTo { input: this, to: false }
    /// # ;
    /// # assert_eq!(this.up(), output);
    /// ```
    pub fn up(self) -> SetTo<Self, bool> {
        self.set_to(false)
    }

    /// Press and release the button.
    /// This is a convenience shorthand for
    /// ```
    /// # use kemuler::{prelude::*, input_event::*};
    /// # let this = MouseButton::Left;
    /// # let output =
    /// (
    ///     SetTo { input: this, to: true },
    ///     SetTo { input: this, to: false }
    /// ).seq()
    /// # ;
    /// # assert_eq!(this.click(), output);
    /// ```
    pub fn click(self) -> Sequence<(SetTo<Self, bool>, SetTo<Self, bool>)> {
        self.down().then(self.up())
    }
}

fn windowsify_common_mouse_button(button: common_inputs::MouseButton) -> MouseButton {
    match button {
        common_inputs::MouseButton::Left => MouseButton::Left,
        common_inputs::MouseButton::Middle => MouseButton::Middle,
        common_inputs::MouseButton::Right => MouseButton::Right,
    }
}

fn windowsify_common_key(key: common_inputs::Key) -> VirtualKey {
    match key {
        common_inputs::Key::Alt => VirtualKey::Alt,
        common_inputs::Key::Shift => VirtualKey::Shift,
        common_inputs::Key::Control => VirtualKey::Control,
        common_inputs::Key::F1 => VirtualKey::F1,
        common_inputs::Key::F2 => VirtualKey::F2,
        common_inputs::Key::F3 => VirtualKey::F3,
        common_inputs::Key::F4 => VirtualKey::F4,
        common_inputs::Key::F5 => VirtualKey::F5,
        common_inputs::Key::F6 => VirtualKey::F6,
        common_inputs::Key::F7 => VirtualKey::F7,
        common_inputs::Key::F8 => VirtualKey::F8,
        common_inputs::Key::F9 => VirtualKey::F9,
        common_inputs::Key::F10 => VirtualKey::F10,
        common_inputs::Key::F11 => VirtualKey::F11,
        common_inputs::Key::F12 => VirtualKey::F12,
        common_inputs::Key::CapsLock => VirtualKey::CapsLock,
        common_inputs::Key::End => VirtualKey::End,
        common_inputs::Key::Home => VirtualKey::Home,
        common_inputs::Key::PageUp => VirtualKey::PageUp,
        common_inputs::Key::PageDown => VirtualKey::PageDown,
        common_inputs::Key::Escape => VirtualKey::Escape,
        common_inputs::Key::Enter => VirtualKey::Enter,
        common_inputs::Key::Space => VirtualKey::Space,
        common_inputs::Key::Tab => VirtualKey::Tab,
        common_inputs::Key::Backspace => VirtualKey::Backspace,
        common_inputs::Key::Delete => VirtualKey::Delete,
        common_inputs::Key::UpArrow => VirtualKey::UpArrow,
        common_inputs::Key::DownArrow => VirtualKey::DownArrow,
        common_inputs::Key::LeftArrow => VirtualKey::LeftArrow,
        common_inputs::Key::RightArrow => VirtualKey::RightArrow,
    }
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

impl Simulate<SetTo<common_inputs::MouseButton, bool>> for Windows {
    fn simulate(&mut self, simulatable: SetTo<common_inputs::MouseButton, bool>) {
        let SetTo {
            input: button,
            to: is_down,
        } = simulatable;
        let button = windowsify_common_mouse_button(button);
        if is_down {
            inner::mouse_button_down(button)
        } else {
            inner::mouse_button_up(button)
        }
    }
}

impl Simulate<SetTo<common_inputs::MousePosition, (i32, i32)>> for Windows {
    fn simulate(&mut self, simulatable: SetTo<common_inputs::MousePosition, (i32, i32)>) {
        let SetTo {
            input: _,
            to: position,
        } = simulatable;
        inner::virtual_desktop_normalized_mouse_move_to(position.0, position.1);
    }
}

impl Simulate<ChangeBy<common_inputs::MousePosition, (i32, i32)>> for Windows {
    fn simulate(&mut self, simulatable: ChangeBy<common_inputs::MousePosition, (i32, i32)>) {
        let ChangeBy { input: _, by } = simulatable;
        inner::virtual_desktop_normalized_mouse_move_by(by.0, by.1);
    }
}

impl Simulate<ChangeBy<common_inputs::MouseScroll, (i32, i32)>> for Windows {
    fn simulate(&mut self, simulatable: ChangeBy<common_inputs::MouseScroll, (i32, i32)>) {
        let ChangeBy { input: _, by } = simulatable;
        inner::mouse_scroll(by.0, by.1)
    }
}