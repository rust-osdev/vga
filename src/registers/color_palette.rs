use super::{
    COLOR_PALETTE_DATA_ADDRESS, COLOR_PALETTE_INDEX_READ_ADDRESS,
    COLOR_PALETTE_INDEX_WRITE_ADDRESSS, PALETTE_SIZE,
};
use x86_64::instructions::port::Port;

/// Represents the color palette registers on vga hardware.
#[derive(Debug)]
pub struct ColorPaletteRegisters {
    data_port: Port<u8>,
    index_read_port: Port<u8>,
    index_write_port: Port<u8>,
}

impl ColorPaletteRegisters {
    pub(crate) fn new() -> ColorPaletteRegisters {
        ColorPaletteRegisters {
            data_port: Port::new(COLOR_PALETTE_DATA_ADDRESS),
            index_read_port: Port::new(COLOR_PALETTE_INDEX_READ_ADDRESS),
            index_write_port: Port::new(COLOR_PALETTE_INDEX_WRITE_ADDRESSS),
        }
    }

    /// Loads a 256 color palette, as specified by `palette`, with every 3
    /// bytes representing a color.
    pub fn load_palette(&mut self, palette: &[u8; PALETTE_SIZE]) {
        unsafe {
            self.index_write_port.write(0);
        }
        for i in palette.iter() {
            unsafe {
                self.data_port.write(*i);
            }
        }
    }

    /// Reads the current 256 color palette into `palette`, with every 3
    /// bytes representing a color.
    pub fn read_palette(&mut self, palette: &mut [u8; PALETTE_SIZE]) {
        unsafe {
            self.index_read_port.write(0);
        }
        for byte in palette.iter_mut().take(PALETTE_SIZE) {
            unsafe {
                *byte = self.data_port.read();
            }
        }
    }
}
