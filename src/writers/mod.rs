//! Writers for common vga modes.
mod graphics_1280x800x256;
mod graphics_320x200x256;
mod graphics_320x240x256;
mod graphics_640x480x16;
mod text_40x25;
mod text_40x50;
mod text_80x25;

use super::{
    colors::{Color16, TextModeColor},
    drawing::Point,
    registers::CrtcControllerIndex,
    vga::{Vga, VGA},
};
use core::slice::from_raw_parts_mut;
use spinning_top::SpinlockGuard;

use crate::drawing::Bresenham;
pub use graphics_1280x800x256::Graphics1280x800x256;
pub use graphics_320x200x256::Graphics320x200x256;
pub use graphics_320x240x256::Graphics320x240x256;
pub use graphics_640x480x16::Graphics640x480x16;
pub use text_40x25::Text40x25;
pub use text_40x50::Text40x50;
pub use text_80x25::Text80x25;

/// Represents a `ScreenCharacter` in vga text modes.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct ScreenCharacter {
    character: u8,
    color: TextModeColor,
}

impl ScreenCharacter {
    /// Creates a new `ScreenCharacter` with the specified `character` and `TextModeColor`.
    pub const fn new(character: u8, color: TextModeColor) -> ScreenCharacter {
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
    color: TextModeColor::new(Color16::Yellow, Color16::Black),
};

/// A helper trait used to interact with various vga screens.
pub trait Screen {
    /// The width of the `Screen`.
    const WIDTH: usize;
    /// The height of the `Screen`.
    const HEIGHT: usize;
    /// The size (total area) of the `Screen`.
    const SIZE: usize;
}

/// A helper trait used to interact with various vga text modes.
pub trait TextWriter: Screen {
    /// Sets the graphics device to a video mode as determined by
    /// the `TextWriter` implementation.
    fn set_mode(&self);

    /// Returns the start of the `FrameBuffer` as `*mut ScreenCharacter`
    /// as well as a lock to the vga driver. This ensures the vga
    /// driver stays locked while the frame buffer is in use.
    fn get_frame_buffer(&self) -> (SpinlockGuard<Vga>, *mut ScreenCharacter) {
        let mut vga = VGA.lock();
        let frame_buffer = vga.get_frame_buffer();
        (vga, usize::from(frame_buffer) as *mut ScreenCharacter)
    }

    /// Clears the screen by setting all cells to `b' '` with
    /// a background color of `Color16::Black` and a foreground
    /// color of `Color16::Yellow`.
    fn clear_screen(&self) {
        self.fill_screen(BLANK_CHARACTER);
    }

    /// Fills the screen by setting all cells to the given screen character.
    fn fill_screen(&self, character: ScreenCharacter) {
        let (_vga, frame_buffer) = self.get_frame_buffer();
        for i in 0..Self::SIZE {
            unsafe {
                frame_buffer.add(i).write_volatile(character);
            }
        }
    }

    /// Disables the cursor in vga text modes.
    fn disable_cursor(&self) {
        let (mut vga, _frame_buffer) = self.get_frame_buffer();
        let emulation_mode = vga.get_emulation_mode();
        let cursor_start = vga
            .crtc_controller_registers
            .read(emulation_mode, CrtcControllerIndex::TextCursorStart);
        vga.crtc_controller_registers.write(
            emulation_mode,
            CrtcControllerIndex::TextCursorStart,
            cursor_start | 0x20,
        );
    }

    /// Enables the cursor in vga text modes.
    fn enable_cursor(&self) {
        let (mut vga, _frame_buffer) = self.get_frame_buffer();
        let emulation_mode = vga.get_emulation_mode();
        let cursor_start = vga
            .crtc_controller_registers
            .read(emulation_mode, CrtcControllerIndex::TextCursorStart);
        vga.crtc_controller_registers.write(
            emulation_mode,
            CrtcControllerIndex::TextCursorStart,
            cursor_start & 0xDF,
        );
    }

    /// Returns the `ScreenCharacter` at the given `(x, y)` position.
    fn read_character(&self, x: usize, y: usize) -> ScreenCharacter {
        let (_vga, frame_buffer) = self.get_frame_buffer();
        let offset = Self::WIDTH * y + x;
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
        let cursor_start = vga
            .crtc_controller_registers
            .read(emulation_mode, CrtcControllerIndex::TextCursorStart)
            & 0xC0;
        let cursor_end = vga
            .crtc_controller_registers
            .read(emulation_mode, CrtcControllerIndex::TextCursorEnd)
            & 0xE0;
        vga.crtc_controller_registers.write(
            emulation_mode,
            CrtcControllerIndex::TextCursorStart,
            cursor_start | scan_line_start,
        );
        vga.crtc_controller_registers.write(
            emulation_mode,
            CrtcControllerIndex::TextCursorEnd,
            cursor_end | scan_line_end,
        );
    }

    /// Sets the current text cursor to the position specified by
    /// `x` and `y`.
    fn set_cursor_position(&self, x: usize, y: usize) {
        let offset = Self::WIDTH * y + x;
        let (mut vga, _frame_buffer) = self.get_frame_buffer();
        let emulation_mode = vga.get_emulation_mode();
        let cursor_start = offset & 0xFF;
        let cursor_end = (offset >> 8) & 0xFF;
        vga.crtc_controller_registers.write(
            emulation_mode,
            CrtcControllerIndex::TextCursorLocationLow,
            cursor_start as u8,
        );
        vga.crtc_controller_registers.write(
            emulation_mode,
            CrtcControllerIndex::TextCursorLocationHigh,
            cursor_end as u8,
        );
    }

    /// Prints the given `character` and `color` at `(x, y)`.
    fn write_character(&self, x: usize, y: usize, screen_character: ScreenCharacter) {
        let (_vga, frame_buffer) = self.get_frame_buffer();
        let offset = Self::WIDTH * y + x;
        unsafe {
            frame_buffer.add(offset).write_volatile(screen_character);
        }
    }
}

/// A helper trait used to interact with various vga graphics modes.
pub trait GraphicsWriter<Color: Copy> {
    /// Clears the screen by setting all pixels to the specified `color`.
    fn clear_screen(&self, color: Color);

    /// Draws a character at the given `(x, y)` coordinate to the specified `color`.
    fn draw_character(&self, x: usize, y: usize, character: char, color: Color);

    /// Sets the given pixel at `(x, y)` to the given `color`.
    fn set_pixel(&self, x: usize, y: usize, color: Color);

    /// Sets the graphics device to a `VideoMode`.
    fn set_mode(&self);

    /// Returns the frame buffer for this vga mode.
    fn get_frame_buffer(&self) -> *mut u8 {
        usize::from(VGA.lock().get_frame_buffer()) as *mut u8
    }
}

/// Implementations of this trait can draw primitive shapes.
pub trait PrimitiveDrawing<C>: GraphicsWriter<C> + Screen
where
    C: Copy,
{
    /// Draws a line from `start` to `end` with the specified `color`.
    fn draw_line(&self, start: Point<isize>, end: Point<isize>, color: C) {
        for (x, y) in Bresenham::new(start, end) {
            self.set_pixel(x as usize, y as usize, color);
        }
    }

    /// Draws a rectangle from `p1` to `p2` with the specified `color`.
    fn draw_rect(&self, p1: Point<usize>, p2: Point<usize>, color: C) {
        let frame_buffer = self.get_frame_buffer() as *mut C;
        let line_width = p2.0.abs_diff(p1.0);

        (p1.1..p2.1)
            .map(|y| Self::WIDTH * y + p1.0)
            .map(|offset| unsafe { frame_buffer.add(offset) })
            .map(|ptr| unsafe { from_raw_parts_mut(ptr, line_width) })
            .for_each(|line| line.fill(color));
    }
}
