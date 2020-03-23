use super::{Screen, TextWriter};
use crate::{
    colors::DEFAULT_PALETTE,
    fonts::TEXT_8X8_FONT,
    vga::{VideoMode, VGA},
};

const WIDTH: usize = 40;
const HEIGHT: usize = 50;

/// A basic interface for interacting with vga text mode 40x50
///
/// # Examples
///
/// Basic usage:
///
/// ```no_run
/// use vga::colors::{Color16Bit, TextModeColor};
/// use vga::writers::{ScreenCharacter, TextWriter, Text40x50};
///
/// let text_mode = Text40x50::new();
/// let color = TextModeColor::new(Color16Bit::Yellow, Color16Bit::Black);
/// let screen_character = ScreenCharacter::new(b'T', color);
///
/// text_mode.set_mode();
/// text_mode.clear_screen();
/// text_mode.write_character(0, 0, screen_character);
/// ```
#[derive(Default)]
pub struct Text40x50;

impl Screen for Text40x50 {
    fn get_width(&self) -> usize {
        WIDTH
    }

    fn get_height(&self) -> usize {
        HEIGHT
    }

    fn get_size(&self) -> usize {
        WIDTH * HEIGHT
    }
}

impl TextWriter for Text40x50 {
    /// Sets the graphics device to `VideoMode::Mode40x50`.
    fn set_mode(&self) {
        let mut vga = VGA.lock();
        vga.set_video_mode(VideoMode::Mode40x50);

        // Some bios mess up the palette when switching modes,
        // so explicitly set it.
        vga.color_palette_registers.load_palette(&DEFAULT_PALETTE);
        vga.load_font(&TEXT_8X8_FONT);
    }
}

impl Text40x50 {
    /// Creates a new `Text40x50`.
    pub fn new() -> Text40x50 {
        Text40x50 {}
    }
}
