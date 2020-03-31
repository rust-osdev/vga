use super::{GraphicsWriter, Screen};
use crate::{
    colors::{Color16, DEFAULT_PALETTE},
    drawing::{Bresenham, Point},
    registers::{PlaneMask, WriteMode},
    vga::{Vga, VideoMode, VGA},
};
use font8x8::UnicodeFonts;
use spinning_top::SpinlockGuard;

const WIDTH: usize = 640;
const HEIGHT: usize = 480;
const SIZE: usize = WIDTH * HEIGHT;
const ALL_PLANES_SCREEN_SIZE: usize = (WIDTH * HEIGHT) / 8;
const WIDTH_IN_BYTES: usize = WIDTH / 8;

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
#[derive(Debug, Clone, Copy, Default)]
pub struct Graphics640x480x16;

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

impl GraphicsWriter<Color16> for Graphics640x480x16 {
    fn clear_screen(&self, color: Color16) {
        self.set_write_mode_2();
        let (_vga, frame_buffer) = self.get_frame_buffer();
        for offset in 0..ALL_PLANES_SCREEN_SIZE {
            unsafe {
                frame_buffer.add(offset).write_volatile(u8::from(color));
            }
        }
    }

    fn draw_line(&self, start: Point<isize>, end: Point<isize>, color: Color16) {
        self.set_write_mode_0(color);
        for (x, y) in Bresenham::new(start, end) {
            self._set_pixel(x as usize, y as usize, color);
        }
    }

    fn draw_character(&self, x: usize, y: usize, character: char, color: Color16) {
        self.set_write_mode_2();
        let character = match font8x8::BASIC_FONTS.get(character) {
            Some(character) => character,
            // Default to a filled block if the character isn't found
            None => font8x8::unicode::BLOCK_UNICODE[8].byte_array(),
        };

        for (row, byte) in character.iter().enumerate() {
            for bit in 0..8 {
                match *byte & 1 << bit {
                    0 => (),
                    _ => self._set_pixel(x + bit, y + row, color),
                }
            }
        }
    }

    /// **Note:** This method is provided for convenience, but has terrible
    /// performance since it needs to ensure the correct `WriteMode` per pixel
    /// drawn. If you need to draw more then one pixel, consider using a method
    /// such as `draw_line`.
    fn set_pixel(&self, x: usize, y: usize, color: Color16) {
        self.set_write_mode_2();
        self._set_pixel(x, y, color);
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
        Graphics640x480x16 {}
    }

    fn set_write_mode_0(&self, color: Color16) {
        let (mut vga, _frame_buffer) = self.get_frame_buffer();
        vga.graphics_controller_registers.write_set_reset(color);
        vga.graphics_controller_registers
            .write_enable_set_reset(0xF);
        vga.graphics_controller_registers
            .set_write_mode(WriteMode::Mode0);
    }

    fn set_write_mode_2(&self) {
        let (mut vga, _frame_buffer) = self.get_frame_buffer();
        vga.graphics_controller_registers
            .set_write_mode(WriteMode::Mode2);
        vga.graphics_controller_registers.set_bit_mask(0xFF);
        vga.sequencer_registers
            .set_plane_mask(PlaneMask::ALL_PLANES);
    }

    /// Returns the start of the `FrameBuffer` as `*mut u8` as
    /// well as a lock to the vga driver. This ensures the vga
    /// driver stays locked while the frame buffer is in use.
    fn get_frame_buffer(&self) -> (SpinlockGuard<Vga>, *mut u8) {
        let mut vga = VGA.lock();
        let frame_buffer = vga.get_frame_buffer();
        (vga, u32::from(frame_buffer) as *mut u8)
    }

    #[inline]
    fn _set_pixel(&self, x: usize, y: usize, color: Color16) {
        let (mut vga, frame_buffer) = self.get_frame_buffer();
        let offset = x / 8 + y * WIDTH_IN_BYTES;
        let pixel_mask = 0x80 >> (x & 0x07);
        vga.graphics_controller_registers.set_bit_mask(pixel_mask);
        unsafe {
            frame_buffer.add(offset).read_volatile();
            frame_buffer.add(offset).write_volatile(u8::from(color));
        }
    }
}
