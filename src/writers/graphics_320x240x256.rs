use super::{GraphicsWriter, Screen};
use crate::{
    colors::DEFAULT_PALETTE,
    drawing::{Bresenham, Point},
    registers::PlaneMask,
    vga::{VideoMode, VGA},
};
use font8x8::UnicodeFonts;

const WIDTH: usize = 320;
const HEIGHT: usize = 240;
const SIZE: usize = (WIDTH * HEIGHT) / 4;

#[derive(Debug, Clone, Copy, Default)]
pub struct Graphics320x240x256;

impl Screen for Graphics320x240x256 {
    const WIDTH: usize = WIDTH;
    const HEIGHT: usize = HEIGHT;
    const SIZE: usize = SIZE;
}

impl GraphicsWriter<u8> for Graphics320x240x256 {
    fn clear_screen(&self, color: u8) {
        unsafe {
            self.get_frame_buffer().write_bytes(color, Self::SIZE);
        }
    }
    fn draw_line(&self, start: Point<isize>, end: Point<isize>, color: u8) {
        for (x, y) in Bresenham::new(start, end) {
            self.set_pixel(x as usize, y as usize, color);
        }
    }
    fn set_pixel(&self, x: usize, y: usize, color: u8) {
        let frame_buffer = self.get_frame_buffer();
        unsafe {
            let offset = (WIDTH * y + x) / 4;
            let plane_mask = 0x1 << (x & 3);
            VGA.lock()
                .sequencer_registers
                .set_plane_mask(PlaneMask::from_bits(plane_mask).unwrap());
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
        vga.set_video_mode(VideoMode::Mode320x240x256);

        // Some bios mess up the palette when switching modes,
        // so explicitly set it.
        vga.color_palette_registers.load_palette(&DEFAULT_PALETTE);
    }
}

impl Graphics320x240x256 {
    pub const fn new() -> Graphics320x240x256 {
        Graphics320x240x256
    }
}
