use super::{GraphicsWriter, Screen};
use crate::writers::PrimitiveDrawing;
use crate::{
    colors::DEFAULT_PALETTE,
    registers::PlaneMask,
    vga::{VideoMode, VGA},
};
use font8x8::UnicodeFonts;

const WIDTH: usize = 320;
const HEIGHT: usize = 240;
const SIZE: usize = (WIDTH * HEIGHT) / 4;

/// A basic interface for interacting with vga graphics mode 320x200x256.
///
/// # Examples
///
/// Basic usage:
///
/// ```no_run
/// use vga::colors::Color16;
/// use vga::writers::{Graphics320x240x256, GraphicsWriter, PrimitiveDrawing};
///
/// let mode = Graphics320x240x256::new();
/// mode.set_mode();
/// mode.clear_screen(0);
/// mode.draw_line((60, 20), (260, 20), 255);
/// mode.draw_line((60, 20), (60, 180), 255);
/// mode.draw_line((60, 180), (260, 180), 255);
/// mode.draw_line((260, 180), (260, 20), 255);
/// mode.draw_line((60, 40), (260, 40), 255);
/// for (offset, character) in "Hello World!".chars().enumerate() {
///     mode.draw_character(118 + offset * 8, 27, character, 255);
/// }
/// mode.draw_rect((300, 180), (320, 240), 255);
/// ```
#[derive(Debug, Clone, Copy, Default)]
pub struct Graphics320x240x256;

impl Screen for Graphics320x240x256 {
    const WIDTH: usize = WIDTH;
    const HEIGHT: usize = HEIGHT;
    const SIZE: usize = SIZE;
}

impl GraphicsWriter<u8> for Graphics320x240x256 {
    fn clear_screen(&self, color: u8) {
        let frame_buffer = self.get_frame_buffer();
        VGA.lock()
            .sequencer_registers
            .set_plane_mask(PlaneMask::ALL_PLANES);
        unsafe {
            frame_buffer.write_bytes(color, Self::SIZE);
        }
    }

    fn set_pixel(&self, x: usize, y: usize, color: u8) {
        let frame_buffer = self.get_frame_buffer();
        unsafe {
            let offset = (WIDTH * y + x) / 4;
            let plane_mask = 0x1 << (x & 3);
            VGA.lock()
                .sequencer_registers
                .set_plane_mask(PlaneMask::from_bits(plane_mask).unwrap());
            frame_buffer.add(offset).write_volatile(color);
        }
    }
    fn draw_character(&self, x: usize, y: usize, character: char, color: u8) {
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
    fn set_mode(&self) {
        let mut vga = VGA.lock();
        vga.set_video_mode(VideoMode::Mode320x240x256);

        // Some bios mess up the palette when switching modes,
        // so explicitly set it.
        vga.color_palette_registers.load_palette(&DEFAULT_PALETTE);
    }
}

impl PrimitiveDrawing<u8> for Graphics320x240x256 {}

impl Graphics320x240x256 {
    /// Creates a new `Graphics320x240x256`.
    pub const fn new() -> Graphics320x240x256 {
        Graphics320x240x256
    }
}
