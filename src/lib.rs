//! This crate provides vga specific functions, data structures,
//! and access to various registers.
//!
//! Memory addresses `0xA0000 -> 0xBFFFF` must be readable and writeable
//! this crate to work properly.

#![no_std]
#![warn(missing_docs)]

mod colors;
mod configurations;
mod fonts;
mod registers;
mod vga;
mod writers;

pub use self::colors::{Color16Bit, TextModeColor, DEFAULT_PALETTE, PALETTE_SIZE};
pub use self::configurations::{
    VgaConfiguration, MODE_40X25_CONFIGURATION, MODE_40X50_CONFIGURATION,
    MODE_640X480X16_CONFIGURATION, MODE_80X25_CONFIGURATION,
};
pub use self::vga::{Vga, VideoMode, VGA};
pub use self::writers::{Graphics640x480x16, Text40x25, Text40x50, Text80x25};
