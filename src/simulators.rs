//! Collection of built-in `Simulator` implemented by this crate

#[cfg(feature = "enigo")]
pub mod enigo;

#[cfg(feature = "windows")]
pub mod windows;

#[cfg(any(test, doctest, feature = "test"))]
pub mod string_event_logger;
