use super::ScreenCharacter;
use crate::{
    colors::{Color16Bit, TextModeColor, DEFAULT_PALETTE},
    fonts::TEXT_8X16_FONT,
    vga::{Vga, VideoMode, VGA},
};
use spinning_top::SpinlockGuard;

const WIDTH: usize = 80;
const HEIGHT: usize = 25;
const SCREEN_SIZE: usize = WIDTH * HEIGHT;

static BLANK_CHARACTER: ScreenCharacter = ScreenCharacter {
    character: b' ',
    color: TextModeColor::new(Color16Bit::Yellow, Color16Bit::Black),
};

/// A basic interface for interacting with vga text mode 80x25
///
/// # Examples
///
/// Basic usage:
///
/// ```no_run
/// use vga::Text80x25;
///
/// let text_mode = Text80x25::new();
///
/// text_mode.set_mode();
/// text_mode.clear_screen();
/// ```
#[derive(Default)]
pub struct Text80x25;

impl Text80x25 {
    /// Creates a new `Text80x25`.
    pub fn new() -> Text80x25 {
        Text80x25 {}
    }

    /// Clears the screen by setting all cells to `b' '` with
    /// a background color of `Color16Bit::Black` and a foreground
    /// color of `Color16Bit::Yellow`.
    pub fn clear_screen(&self) {
        let (_vga, frame_buffer) = self.get_frame_buffer();
        for i in 0..SCREEN_SIZE {
            unsafe {
                frame_buffer.add(i).write_volatile(BLANK_CHARACTER);
            }
        }
    }

    /// Prints the given `character` and `color` at `(x, y)`.
    ///
    /// Panics if `x >= 80` or `y >= 25`.
    pub fn write_character(&self, x: usize, y: usize, character: u8, color: TextModeColor) {
        assert!(x < WIDTH, "x >= {}", WIDTH);
        assert!(y < HEIGHT, "y >= {}", HEIGHT);
        let (_vga, frame_buffer) = self.get_frame_buffer();
        let offset = WIDTH * y + x;
        unsafe {
            frame_buffer
                .add(offset)
                .write_volatile(ScreenCharacter { character, color });
        }
    }

    /// Sets the graphics device to `VideoMode::Mode80x25`.
    pub fn set_mode(&self) {
        let mut vga = VGA.lock();
        vga.set_video_mode(VideoMode::Mode80x25);

        // Some bios mess up the palette when switching modes,
        // so explicitly set it.
        vga.load_palette(&DEFAULT_PALETTE);
        vga.load_font(&TEXT_8X16_FONT);
    }

    /// Returns the start of the `FrameBuffer` as `*mut ScreenCharacter`
    /// as well as a lock to the vga driver. This ensures the vga
    /// driver stays locked while the frame buffer is in use.
    fn get_frame_buffer(&self) -> (SpinlockGuard<Vga>, *mut ScreenCharacter) {
        let mut vga = VGA.lock();
        let frame_buffer = vga.get_frame_buffer();
        (vga, u32::from(frame_buffer) as *mut ScreenCharacter)
    }
}
