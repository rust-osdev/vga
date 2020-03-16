//! Writers for common vga modes.
mod graphics_640x480x16;
mod text_40x25;
mod text_40x50;
mod text_80x25;

use super::{
    colors::{Color16Bit, TextModeColor},
    registers::CrtcControllerIndex,
    vga::{Vga, VGA},
};
use spinning_top::SpinlockGuard;

pub use graphics_640x480x16::Graphics640x480x16;
pub use text_40x25::Text40x25;
pub use text_40x50::Text40x50;
pub use text_80x25::Text80x25;

/// Represents a `ScreenCharacter` in vga text modes.
#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct ScreenCharacter {
    character: u8,
    color: TextModeColor,
}

impl ScreenCharacter {
    /// Creates a new `ScreenCharacter` with the specified `character`
    /// and a `TextModeColor` with the specified `foreground` and `background`.
    pub fn new(character: u8, foreground: Color16Bit, background: Color16Bit) -> ScreenCharacter {
        let color = TextModeColor::new(foreground, background);
        ScreenCharacter { character, color }
    }
}

static BLANK_CHARACTER: ScreenCharacter = ScreenCharacter {
    character: b' ',
    color: TextModeColor::new(Color16Bit::Yellow, Color16Bit::Black),
};

/// A helper trait used to interact with various vga text modes.
pub trait TextWriter {
    /// Returns the width of the `TextWriter`.
    fn get_width(&self) -> usize;

    /// Returns the height of the `TextWriter`.
    fn get_height(&self) -> usize;

    /// Sets the graphics device to a video mode as determined by
    /// the `TextWriter` implementation.
    fn set_mode(&self);

    /// Returns the start of the `FrameBuffer` as `*mut ScreenCharacter`
    /// as well as a lock to the vga driver. This ensures the vga
    /// driver stays locked while the frame buffer is in use.
    fn get_frame_buffer(&self) -> (SpinlockGuard<Vga>, *mut ScreenCharacter) {
        let mut vga = VGA.lock();
        let frame_buffer = vga.get_frame_buffer();
        (vga, u32::from(frame_buffer) as *mut ScreenCharacter)
    }

    /// Clears the screen by setting all cells to `b' '` with
    /// a background color of `Color16Bit::Black` and a foreground
    /// color of `Color16Bit::Yellow`.
    fn clear_screen(&self) {
        let (_vga, frame_buffer) = self.get_frame_buffer();
        let screen_size = self.get_width() * self.get_height();
        for i in 0..screen_size {
            unsafe {
                frame_buffer.add(i).write_volatile(BLANK_CHARACTER);
            }
        }
    }

    /// Sets the current text cursor to the position specified by
    /// `x` and `y`.
    ///
    /// Panics if `x >= se.lf.get_width()` or `y >= self.get_height()`.
    fn set_cursor_position(&self, x: usize, y: usize) {
        let width = self.get_width();
        let height = self.get_height();
        assert!(x < width, "x >= {}", width);
        assert!(y < height, "y >= {}", height);
        let offset = width * y + x;
        let (mut vga, _frame_buffer) = self.get_frame_buffer();
        let emulation_mode = vga.get_emulation_mode();
        let cursor_start = offset & 0xFF;
        let cursor_end = (offset >> 8) & 0xFF;
        vga.write_crtc_controller(
            emulation_mode,
            CrtcControllerIndex::TextCursorLocationLow,
            cursor_start as u8,
        );
        vga.write_crtc_controller(
            emulation_mode,
            CrtcControllerIndex::TextCursorLocationHigh,
            cursor_end as u8,
        );
    }

    /// Prints the given `character` and `color` at `(x, y)`.
    ///
    /// Panics if `x >= self.get_width()` or `y >= self.get_height()`.
    fn write_character(&self, x: usize, y: usize, screen_character: ScreenCharacter) {
        let width = self.get_width();
        let height = self.get_height();
        assert!(x < width, "x >= {}", width);
        assert!(y < height, "y >= {}", height);
        let (_vga, frame_buffer) = self.get_frame_buffer();
        let offset = width * y + x;
        unsafe {
            frame_buffer.add(offset).write_volatile(screen_character);
        }
    }
}
