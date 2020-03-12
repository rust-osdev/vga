//! This crate provides vga specific functions, data structures,
//! and access to various registers.

#![no_std]
#![warn(missing_docs)]

pub mod vga;
pub mod vga_colors;
mod vga_configurations;
mod vga_fonts;
mod vga_registers;

pub use vga::VGA;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
