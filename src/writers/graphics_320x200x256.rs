use super::{GraphicsWriter, Screen};
use crate::{
    colors::DEFAULT_PALETTE,
    drawing::{Bresenham, Device, Point},
    vga::{VideoMode, VGA},
};
use alloc::vec::Vec;
use core::ptr;
use font8x8::UnicodeFonts;

const WIDTH: usize = 320;
const HEIGHT: usize = 200;
const SIZE: usize = WIDTH * HEIGHT;

/// A basic interface for interacting with vga graphics mode 320x200x256.
///
/// # Examples
///
/// Basic usage:
///
/// ```no_run
/// use vga::colors::Color16;
/// use vga::writers::{Graphics320x200x256, GraphicsWriter};

/// let mode = Graphics320x200x256::new();
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
/// ```
#[derive(Default)]
pub struct Graphics320x200x256 {
    screen_buffer: Vec<u8>,
}

impl Screen for Graphics320x200x256 {
    #[inline]
    fn get_width(&self) -> usize {
        WIDTH
    }

    #[inline]
    fn get_height(&self) -> usize {
        HEIGHT
    }

    #[inline]
    fn get_size(&self) -> usize {
        SIZE
    }
}

impl Device<u8> for Graphics320x200x256 {
    fn draw_character(&mut self, x: usize, y: usize, character: char, color: u8) {
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

    fn draw_line(&mut self, start: Point<isize>, end: Point<isize>, color: u8) {
        for Point { x, y } in Bresenham::new(start, end) {
            self.set_pixel(x as usize, y as usize, color);
        }
    }

    fn present(&self) {
        {
            let mut vga = VGA.lock();
            let emulation_mode = vga.get_emulation_mode();
            while vga.general_registers.read_st01(emulation_mode) & 0x3 != 0 {}
        }
        unsafe {
            ptr::copy_nonoverlapping(
                self.screen_buffer.as_ptr(),
                self.get_frame_buffer(),
                self.screen_buffer.len(),
            );
        }
    }
}

impl GraphicsWriter<u8> for Graphics320x200x256 {
    fn clear_screen(&mut self, color: u8) {
        unsafe {
            self.screen_buffer
                .as_mut_ptr()
                .write_bytes(color, self.screen_buffer.len());
        }
    }

    fn set_pixel(&mut self, x: usize, y: usize, color: u8) {
        self.screen_buffer[(y * WIDTH) + x] = color;
    }

    fn set_mode(&self) {
        let mut vga = VGA.lock();
        vga.set_video_mode(VideoMode::Mode320x200x256);

        // Some bios mess up the palette when switching modes,
        // so explicitly set it.
        vga.color_palette_registers.load_palette(&DEFAULT_PALETTE);
    }
}

impl Graphics320x200x256 {
    /// Creates a new `Graphics320x200x256`.
    pub fn new() -> Graphics320x200x256 {
        let mut screen_buffer = Vec::with_capacity(SIZE);
        for _ in 0..SIZE {
            screen_buffer.push(0);
        }
        Graphics320x200x256 { screen_buffer }
    }

    fn get_frame_buffer(&self) -> *mut u8 {
        u32::from(VGA.lock().get_frame_buffer()) as *mut u8
    }
}
