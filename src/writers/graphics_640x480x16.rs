use super::{GraphicsWriter, Screen};
use crate::{
    colors::{Color16, DEFAULT_PALETTE},
    drawing::{Bresenham, Device, Point},
    vga::{VideoMode, VGA},
};
use alloc::vec::Vec;
use font8x8::UnicodeFonts;

const WIDTH: usize = 640;
const HEIGHT: usize = 480;
const SIZE: usize = WIDTH * HEIGHT;

/// A basic interface for interacting with vga graphics mode 640x480x16
///
/// # Examples
///
/// Basic usage:
///
/// ```no_run
/// use vga::colors::Color16;
/// use vga::writers::{Graphics640x480x16, GraphicsWriter};

/// let mode = Graphics640x480x16::new();
/// mode.set_mode();
/// mode.clear_screen(Color16::Black);
/// mode.draw_line((80, 60), (80, 420), Color16::White);
/// mode.draw_line((80, 60), (540, 60), Color16::White);
/// mode.draw_line((80, 420), (540, 420), Color16::White);
/// mode.draw_line((540, 420), (540, 60), Color16::White);
/// mode.draw_line((80, 90), (540, 90), Color16::White);
/// for (offset, character) in "Hello World!".chars().enumerate() {
///     mode.draw_character(270 + offset * 8, 72, character, Color16::White)
/// }
/// ```
#[derive(Default)]
pub struct Graphics640x480x16 {
    screen_buffer: Vec<u8>,
}

impl Screen for Graphics640x480x16 {
    fn get_width(&self) -> usize {
        WIDTH
    }
    fn get_height(&self) -> usize {
        HEIGHT
    }
    fn get_size(&self) -> usize {
        SIZE
    }
}

impl Device<Color16> for Graphics640x480x16 {
    fn draw_character(&mut self, x: usize, y: usize, character: char, color: Color16) {
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

    fn draw_line(&mut self, start: Point<isize>, end: Point<isize>, color: Color16) {
        for Point { x, y } in Bresenham::new(start, end) {
            self.set_pixel(x as usize, y as usize, color);
        }
    }

    fn present(&self) {
        let frame_buffer = self.get_frame_buffer();
        let mut vga = VGA.lock();
        let emulation_mode = vga.get_emulation_mode();
        while vga.general_registers.read_st01(emulation_mode) & 0x3 != 0 {}
        for offset in 0..SIZE {
            let color = self.screen_buffer[offset];
            // Set the mask to the pixel being modified
            vga.graphics_controller_registers
                .set_bit_mask(0x80 >> (offset & 0x7));
            // Faster then offset / 8 ?
            let offset = offset >> 3;
            unsafe {
                // Load the memory latch with 8 pixels
                frame_buffer.add(offset).read_volatile();
                // Write the color to the masked pixel
                frame_buffer.add(offset).write_volatile(color);
            }
        }
    }
}

impl GraphicsWriter<Color16> for Graphics640x480x16 {
    fn clear_screen(&mut self, color: Color16) {
        for x in 0..WIDTH {
            for y in 0..HEIGHT {
                self.set_pixel(x, y, color);
            }
        }
    }

    fn set_pixel(&mut self, x: usize, y: usize, color: Color16) {
        self.screen_buffer[(WIDTH * y) + x] = color as u8;
    }

    fn set_mode(&self) {
        let mut vga = VGA.lock();
        vga.set_video_mode(VideoMode::Mode640x480x16);

        // Some bios mess up the palette when switching modes,
        // so explicitly set it.
        vga.color_palette_registers.load_palette(&DEFAULT_PALETTE);
    }
}

impl Graphics640x480x16 {
    /// Creates a new `Graphics640x480x16`.
    pub fn new() -> Graphics640x480x16 {
        let mut screen_buffer = Vec::with_capacity(SIZE);
        for _ in 0..SIZE {
            screen_buffer.push(0);
        }
        Graphics640x480x16 { screen_buffer }
    }

    fn get_frame_buffer(&self) -> *mut u8 {
        u32::from(VGA.lock().get_frame_buffer()) as *mut u8
    }
}
