pub mod backend;
pub mod combinator;
pub mod control_flow;
pub mod simulate;
pub mod prelude {
    use super::*;

    pub use combinator::Combine;
    pub use control_flow::IntoDuration;
    pub use simulate::{KeyCommon, KeyLayout, MouseButton, MousePosition, MouseScroll};
    pub use simulate::{SimulateAbsoluteValue, SimulateBinaryState, SimulateRelativeValue};
}
