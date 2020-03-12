//! This crate provides vga specific functions, data structures,
//! and access to various registers.
//!
//! Memory addresses `0xA0000 -> 0xBFFFF` must be readable and writeable
//! this crate to work properly.

#![no_std]
#![warn(missing_docs)]

pub mod colors;
mod configurations;
mod fonts;
mod registers;
pub mod vga;
mod writers;

pub use self::vga::VGA;
pub use self::writers::{Graphics640x480x16, Text40x25, Text40x50, Text80x25};
