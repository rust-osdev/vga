use crate::{
    colors::{Color16Bit, DEFAULT_PALETTE},
    vga::{PlaneMask, Vga, VideoMode, VGA},
};
use spinning_top::SpinlockGuard;

const WIDTH: usize = 640;
const HEIGHT: usize = 480;
const ALL_PLANES_SCREEN_SIZE: usize = (WIDTH * HEIGHT) / 4;

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

    /// Clears the screen by setting all pixels to `Color16Bit::Black`.
    pub fn clear_screen(&self) {
        let (mut vga, frame_buffer) = self.get_frame_buffer();
        vga.set_plane_mask(PlaneMask::ALL_PLANES);
        vga.set_graphics_enable_set_reset(PlaneMask::NONE);
        for offset in 0..ALL_PLANES_SCREEN_SIZE {
            unsafe {
                frame_buffer
                    .add(offset)
                    .write_volatile(Color16Bit::Black as u8);
            }
        }
    }

    /// Sets the given pixel at `(x, y)` to the given `color`.
    pub fn set_pixel(&self, x: usize, y: usize, color: Color16Bit) {
        let (mut vga, frame_buffer) = self.get_frame_buffer();
        let offset = x / 8 + (WIDTH / 8) * y;

        // Write to all 4 planes
        vga.set_plane_mask(PlaneMask::ALL_PLANES);

        // Set the bits we want set/reset to the color
        vga.set_graphics_set_reset(color);

        // Enable set/reset for all planes
        vga.set_graphics_enable_set_reset(PlaneMask::ALL_PLANES);
        unsafe {
            // In write mode 0, when enable set/reset is turned on, cpu data
            // is replaced with the data from the set/reset register. Since
            // we're using set/reset for all 4 planes, it doesn't matter what value
            // we write to the memory address.
            frame_buffer.add(offset).write(0x0);
        }
    }

    /// Sets the graphics device to `VideoMode::Mode640x480x16`.
    pub fn set_mode(&self) {
        let mut vga = VGA.lock();
        vga.set_video_mode(VideoMode::Mode640x480x16);

        // Some bios mess up the palette when switching modes,
        // so explicitly set it.
        vga.load_palette(&DEFAULT_PALETTE);
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
