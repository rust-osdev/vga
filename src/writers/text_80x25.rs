use super::{Screen, TextWriter};
use crate::{
    colors::DEFAULT_PALETTE,
    fonts::TEXT_8X16_FONT,
    vga::{VideoMode, VGA},
};

const WIDTH: usize = 80;
const HEIGHT: usize = 25;
const SIZE: usize = WIDTH * HEIGHT;

/// A basic interface for interacting with vga text mode 80x25
///
/// # Examples
///
/// Basic usage:
///
/// ```no_run
/// use vga::colors::{Color16, TextModeColor};
/// use vga::writers::{ScreenCharacter, TextWriter, Text80x25};
///
/// let text_mode = Text80x25::new();
/// let color = TextModeColor::new(Color16::Yellow, Color16::Black);
/// let screen_character = ScreenCharacter::new(b'T', color);
///
/// text_mode.set_mode();
/// text_mode.clear_screen();
/// text_mode.write_character(0, 0, screen_character);
/// ```
#[derive(Debug, Clone, Copy, Default)]
pub struct Text80x25;

impl Screen for Text80x25 {
    const WIDTH: usize = WIDTH;
    const HEIGHT: usize = HEIGHT;
    const SIZE: usize = SIZE;
}

impl TextWriter for Text80x25 {
    fn set_mode(&self) {
        let mut vga = VGA.lock();
        vga.set_video_mode(VideoMode::Mode80x25);

        // Some bios mess up the palette when switching modes,
        // so explicitly set it.
        vga.color_palette_registers.load_palette(&DEFAULT_PALETTE);
        vga.load_font(&TEXT_8X16_FONT);
    }
}

impl Text80x25 {
    /// Creates a new `Text80x25`.
    pub const fn new() -> Text80x25 {
        Text80x25
    }
}
