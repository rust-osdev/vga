use crate::{
    colors::{Color16Bit, DEFAULT_PALETTE},
    registers::{PlaneMask, WriteMode},
    vga::{Vga, VideoMode, VGA},
};
use spinning_top::SpinlockGuard;

const WIDTH: usize = 640;
const HEIGHT: usize = 480;
const ALL_PLANES_SCREEN_SIZE: usize = (WIDTH * HEIGHT) / 8;
const WIDTH_IN_BYTES: usize = WIDTH / 8;

/// A basic interface for interacting with vga graphics mode 640x480x16
///
/// # Examples
///
/// Basic usage:
///
/// ```no_run
/// use vga::writers::Graphics640x480x16;
///
/// let graphics_mode = Graphics640x480x16::new();
///
/// graphics_mode.set_mode();
/// graphics_mode.clear_screen();
/// ```
#[derive(Default)]
pub struct Graphics640x480x16;

impl Graphics640x480x16 {
    /// Creates a new `Graphics640x480x16`.
    pub fn new() -> Graphics640x480x16 {
        Graphics640x480x16 {}
    }

    /// Clears the screen by setting all pixels to the specified `color`.
    pub fn clear_screen(&self, color: Color16Bit) {
        let (mut vga, frame_buffer) = self.get_frame_buffer();
        // Set write mode 2 so data is modified by the bitmask
        vga.graphics_controller_registers
            .set_write_mode(WriteMode::Mode2);
        // Write to all 4 planes at once
        vga.sequencer_registers
            .set_plane_mask(PlaneMask::ALL_PLANES);
        // Every bit should be set to the same color
        vga.graphics_controller_registers.set_bit_mask(0xFF);
        for offset in 0..ALL_PLANES_SCREEN_SIZE {
            unsafe {
                frame_buffer.add(offset).write_volatile(u8::from(color));
            }
        }
    }

    /// Sets the given pixel at `(x, y)` to the given `color`.
    pub fn set_pixel(&self, x: usize, y: usize, color: Color16Bit) {
        let (mut vga, frame_buffer) = self.get_frame_buffer();
        let offset = x / 8 + y * WIDTH_IN_BYTES;
        // Which pixel to modify this write
        let pixel_offset = x & 7;
        // Set write mode 2 so screen data is only modified by the bitmask
        vga.graphics_controller_registers
            .set_write_mode(WriteMode::Mode2);
        // Write to all 4 planes at once
        vga.sequencer_registers
            .set_plane_mask(PlaneMask::ALL_PLANES);
        // Only modify 1 pixel, based on the offset
        vga.graphics_controller_registers
            .set_bit_mask(1 << pixel_offset);
        unsafe {
            // Reads the current offset into the memory latches
            frame_buffer.add(offset).read_volatile();
            // Sets the pixel specified by the offset to the color. The
            // pixels not inlcuded in the bit mask remain untouched.
            frame_buffer.add(offset).write_volatile(u8::from(color));
        }
    }

    /// Sets the graphics device to `VideoMode::Mode640x480x16`.
    pub fn set_mode(&self) {
        let mut vga = VGA.lock();
        vga.set_video_mode(VideoMode::Mode640x480x16);

        // Some bios mess up the palette when switching modes,
        // so explicitly set it.
        vga.color_palette_registers.load_palette(&DEFAULT_PALETTE);
    }

    /// Returns the start of the `FrameBuffer` as `*mut u8` as
    /// well as a lock to the vga driver. This ensures the vga
    /// driver stays locked while the frame buffer is in use.
    fn get_frame_buffer(&self) -> (SpinlockGuard<Vga>, *mut u8) {
        let mut vga = VGA.lock();
        let frame_buffer = vga.get_frame_buffer();
        (vga, u32::from(frame_buffer) as *mut u8)
    }
}
