pub mod backend;
pub mod combinator;
pub mod control_flow;
pub mod emulatable;
pub mod peripherals;
pub mod prelude {
    use super::*;

    pub use combinator::Combine;
    pub use control_flow::IntoDuration;
    pub use emulatable::{EmulateAbsoluteValue, EmulateBinaryState, EmulateRelativeValue};
    pub use peripherals::{KeyCommon, KeyLayout, MouseButton, MousePosition, MouseScroll};
}
