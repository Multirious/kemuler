//! Collection of built-in `Simulator` implemented by this crate

#[cfg(feature = "enigo")]
pub mod enigo;
#[cfg(feature = "windows")]
pub mod window;
