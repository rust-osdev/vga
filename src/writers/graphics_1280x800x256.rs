use core::slice::from_raw_parts_mut;

use font8x8::UnicodeFonts;

use crate::colors::DEFAULT_PALETTE;
use crate::registers::PlaneMask;
use crate::vga::VGA;
use crate::writers::PrimitiveDrawing;

use super::{GraphicsWriter, Screen};

const WIDTH: usize = 1280;
const HEIGHT: usize = 800;
const BYTES_PER_PIXEL: usize = 4;
const PIXEL_COUNT: usize = WIDTH * HEIGHT;
const SIZE: usize = PIXEL_COUNT * BYTES_PER_PIXEL;

type ColorT = u32;

/// A basic interface for interacting with vga graphics mode 1280x800x256.
///
/// # Examples
///
/// Basic usage:
///
/// ```no_run
/// use vga::writers::{Graphics1280x800x256, GraphicsWriter, PrimitiveDrawing};
///
/// let mode = Graphics1280x800x256::new();
/// mode.set_mode();
/// mode.clear_screen(0x00_FF_00);
/// mode.draw_line((60, 20), (260, 20), 0xFF_00_FF);
/// mode.draw_line((60, 20), (60, 180), 0xFF_00_FF);
/// mode.draw_line((60, 180), (260, 180), 0xFF_00_FF);
/// mode.draw_line((260, 180), (260, 20), 0xFF_00_FF);
/// mode.draw_line((60, 40), (260, 40), 0xFF_00_FF);
/// for (offset, character) in "Hello World!".chars().enumerate() {
///     mode.draw_character(118 + offset * 8, 27, character, 0xFF_00_FF);
/// }
/// ```
#[derive(Debug, Clone, Copy, Default)]
pub struct Graphics1280x800x256;

impl Screen for Graphics1280x800x256 {
    const WIDTH: usize = WIDTH;
    const HEIGHT: usize = HEIGHT;
    const SIZE: usize = SIZE;
}

impl GraphicsWriter<ColorT> for Graphics1280x800x256 {
    fn clear_screen(&self, color: ColorT) {
        let frame_buffer = self.get_frame_buffer() as *mut ColorT;
        VGA.lock()
            .sequencer_registers
            .set_plane_mask(PlaneMask::ALL_PLANES);
        unsafe {
            from_raw_parts_mut(frame_buffer, PIXEL_COUNT).fill(color);
        }
    }

    fn draw_character(&self, x: usize, y: usize, character: char, color: ColorT) {
        let character = match font8x8::BASIC_FONTS.get(character) {
            Some(character) => character,
            // Default to a filled block if the character isn't found
            None => font8x8::unicode::BLOCK_UNICODE[8].byte_array(),
        };

        for (row, byte) in character.iter().enumerate() {
            for bit in 0..8 {
                match *byte & 1 << bit {
                    0 => (),
                    _ => self.set_pixel(x + bit, y + row, color),
                }
            }
        }
    }

    fn set_pixel(&self, x: usize, y: usize, color: ColorT) {
        let frame_buffer = self.get_frame_buffer() as *mut ColorT;
        let offset = WIDTH * y + x;
        unsafe {
            frame_buffer.add(offset).write_volatile(color);
        }
    }

    fn set_mode(&self) {
        let mut vga = VGA.lock();

        // Some bios mess up the palette when switching modes,
        // so explicitly set it.
        vga.color_palette_registers.load_palette(&DEFAULT_PALETTE);
    }
}

impl PrimitiveDrawing<ColorT> for Graphics1280x800x256 {}

impl Graphics1280x800x256 {
    /// Creates a new `Graphics1280x800x256`.
    pub const fn new() -> Graphics1280x800x256 {
        Graphics1280x800x256
    }
}
