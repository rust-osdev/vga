use super::{GraphicsWriter, Screen};
use crate::{
    colors::DEFAULT_PALETTE,
    drawing::{Bresenham, Point},
    vga::{Vga, VideoMode, VGA},
};
use font8x8::UnicodeFonts;
use spinning_top::SpinlockGuard;

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
#[derive(Debug, Clone, Copy, Default)]
pub struct Graphics320x200x256;

impl Screen for Graphics320x200x256 {
    const WIDTH: usize = WIDTH;
    const HEIGHT: usize = HEIGHT;
    const SIZE: usize = SIZE;
}

impl GraphicsWriter<u8> for Graphics320x200x256 {
    fn clear_screen(&self, color: u8) {
        for x in 0..WIDTH {
            for y in 0..HEIGHT {
                self.set_pixel(x, y, color);
            }
        }
    }
    fn draw_line(&self, start: Point<isize>, end: Point<isize>, color: u8) {
        for (x, y) in Bresenham::new(start, end) {
            self.set_pixel(x as usize, y as usize, color);
        }
    }
    fn set_pixel(&self, x: usize, y: usize, color: u8) {
        let (_vga, frame_buffer) = self.get_frame_buffer();
        let offset = (y * WIDTH) + x;
        unsafe {
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
        vga.set_video_mode(VideoMode::Mode320x200x256);

        // Some bios mess up the palette when switching modes,
        // so explicitly set it.
        vga.color_palette_registers.load_palette(&DEFAULT_PALETTE);
    }
}

impl Graphics320x200x256 {
    /// Creates a new `Graphics320x200x256`.
    pub const fn new() -> Graphics320x200x256 {
        Graphics320x200x256
    }

    /// Returns the start of the `FrameBuffer` as `*mut u8` as
    /// well as a lock to the vga driver. This ensures the vga
    /// driver stays locked while the frame buffer is in use.
    fn get_frame_buffer(self) -> (SpinlockGuard<'static, Vga>, *mut u8) {
        let mut vga = VGA.lock();
        let frame_buffer = vga.get_frame_buffer();
        (vga, u32::from(frame_buffer) as *mut u8)
    }
}
