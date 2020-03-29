//! This crate provides vga specific functions, data structures,
//! and access to various registers.
//!
//! Memory addresses `0xA0000 -> 0xBFFFF` must be readable and writeable
//! this crate to work properly.

#![no_std]
#![warn(missing_docs)]

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod colors;
pub mod configurations;
#[cfg(feature = "alloc")]
pub mod drawing;
pub mod fonts;
pub mod registers;
pub mod vga;
pub mod writers;
