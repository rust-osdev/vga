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
    /// Creates a new `ScreenCharacter` with the specified `character` and `TextModeColor`.
    pub fn new(character: u8, color: TextModeColor) -> ScreenCharacter {
        ScreenCharacter { character, color }
    }

    /// Returns the `character` associated with the `ScreenCharacter`.
    pub fn get_character(self) -> u8 {
        self.character
    }

    /// Returns the `color` associated with the `ScreenCharacter`.
    pub fn get_color(self) -> TextModeColor {
        self.color
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

    /// Disables the cursor in vga text modes.
    fn disable_cursor(&self) {
        let (mut vga, _frame_buffer) = self.get_frame_buffer();
        let emulation_mode = vga.get_emulation_mode();
        let cursor_start =
            vga.read_crtc_controller(emulation_mode, CrtcControllerIndex::TextCursorStart);
        vga.write_crtc_controller(
            emulation_mode,
            CrtcControllerIndex::TextCursorStart,
            cursor_start | 0x20,
        );
    }

    /// Enables the cursor in vga text modes.
    fn enable_cursor(&self) {
        let (mut vga, _frame_buffer) = self.get_frame_buffer();
        let emulation_mode = vga.get_emulation_mode();
        let cursor_start =
            vga.read_crtc_controller(emulation_mode, CrtcControllerIndex::TextCursorStart);
        vga.write_crtc_controller(
            emulation_mode,
            CrtcControllerIndex::TextCursorStart,
            cursor_start & 0xDF,
        );
    }

    /// Returns the `ScreenCharacter` at the given `(x, y)` position.
    fn read_character(&self, x: usize, y: usize) -> ScreenCharacter {
        let (_vga, frame_buffer) = self.get_frame_buffer();
        let offset = self.get_width() * y + x;
        unsafe { frame_buffer.add(offset).read_volatile() }
    }

    /// Sets the size of the cursor, as specified by `scan_line_start` and `scan_line_end`.
    ///
    /// This field controls the appearance of the text mode cursor by specifying the scan
    /// line location within a character cell. The top most scan line is 0, with the bottom
    /// determined by `CrtcControllerIndex::MaxiumumScanLine (usually 15)`.
    /// If `scan_line_start > scan_line_end`, the cursor isn't drawn.
    fn set_cursor(&self, scan_line_start: u8, scan_line_end: u8) {
        let (mut vga, _frame_buffer) = self.get_frame_buffer();
        let emulation_mode = vga.get_emulation_mode();
        let cursor_start =
            vga.read_crtc_controller(emulation_mode, CrtcControllerIndex::TextCursorStart) & 0xC0;
        let cursor_end =
            vga.read_crtc_controller(emulation_mode, CrtcControllerIndex::TextCursorEnd) & 0xE0;
        vga.write_crtc_controller(
            emulation_mode,
            CrtcControllerIndex::TextCursorStart,
            cursor_start | scan_line_start,
        );
        vga.write_crtc_controller(
            emulation_mode,
            CrtcControllerIndex::TextCursorEnd,
            cursor_end | scan_line_end,
        );
    }

    /// Sets the current text cursor to the position specified by
    /// `x` and `y`.
    fn set_cursor_position(&self, x: usize, y: usize) {
        let offset = self.get_width() * y + x;
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
    fn write_character(&self, x: usize, y: usize, screen_character: ScreenCharacter) {
        let (_vga, frame_buffer) = self.get_frame_buffer();
        let offset = self.get_width() * y + x;
        unsafe {
            frame_buffer.add(offset).write_volatile(screen_character);
        }
    }
}
