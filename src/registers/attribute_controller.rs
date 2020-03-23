use super::{
    EmulationMode, ARX_DATA_ADDRESS, ARX_INDEX_ADDRESS, ST01_READ_CGA_ADDRESS,
    ST01_READ_MDA_ADDRESS,
};
use x86_64::instructions::port::Port;

/// Represents an index for the attribute controller registers.
#[derive(Debug, Copy, Clone)]
#[repr(u8)]
pub enum AttributeControllerIndex {
    /// Represents the `Palette 0` register index.
    PaletteRegister0 = 0x00,
    /// Represents the `Palette 1` register index.
    PaletteRegister1 = 0x01,
    /// Represents the `Palette 2` register index.
    PaletteRegister2 = 0x02,
    /// Represents the `Palette 3` register index.
    PaletteRegister3 = 0x03,
    /// Represents the `Palette 4` register index.
    PaletteRegister4 = 0x04,
    /// Represents the `Palette 5` register index.
    PaletteRegister5 = 0x05,
    /// Represents the `Palette 6` register index.
    PaletteRegister6 = 0x06,
    /// Represents the `Palette 7` register index.
    PaletteRegister7 = 0x07,
    /// Represents the `Palette 8` register index.
    PaletteRegister8 = 0x08,
    /// Represents the `Palette 9` register index.
    PaletteRegister9 = 0x09,
    /// Represents the `Palette A` register index.
    PaletteRegisterA = 0x0A,
    /// Represents the `Palette B` register index.
    PaletteRegisterB = 0x0B,
    /// Represents the `Palette C` register index.
    PaletteRegisterC = 0x0C,
    /// Represents the `Palette D` register index.
    PaletteRegisterD = 0x0D,
    /// Represents the `Palette E` register index.
    PaletteRegisterE = 0x0E,
    /// Represents the `Palette F` register index.
    PaletteRegisterF = 0x0F,
    /// Represents the `Mode Control` register index.
    ModeControl = 0x10,
    /// Represents the `Overscan Color` register index.
    OverscanColor = 0x11,
    /// Represents the `Memory Plane Enable` register index.
    MemoryPlaneEnable = 0x12,
    /// Represents the `Horizontal Pixel Panning` register index.
    HorizontalPixelPanning = 0x13,
    /// Represents the `Color Select` register index.
    ColorSelect = 0x14,
}

impl From<AttributeControllerIndex> for u8 {
    fn from(value: AttributeControllerIndex) -> u8 {
        value as u8
    }
}

/// Represents the attribute controller registers on vga hardware.
#[derive(Debug)]
pub struct AttributeControllerRegisters {
    arx_index: Port<u8>,
    arx_data: Port<u8>,
    st01_read_cga: Port<u8>,
    st01_read_mda: Port<u8>,
}

impl AttributeControllerRegisters {
    pub(crate) fn new() -> AttributeControllerRegisters {
        AttributeControllerRegisters {
            arx_index: Port::new(ARX_INDEX_ADDRESS),
            arx_data: Port::new(ARX_DATA_ADDRESS),
            st01_read_cga: Port::new(ST01_READ_CGA_ADDRESS),
            st01_read_mda: Port::new(ST01_READ_MDA_ADDRESS),
        }
    }

    /// Reads the current value of the attribute controller, as specified
    /// by `emulation_mode` and `index`.
    pub fn read(&mut self, emulation_mode: EmulationMode, index: AttributeControllerIndex) -> u8 {
        self.toggle_index(emulation_mode);
        self.set_index(index);
        unsafe { self.arx_data.read() }
    }

    /// Writes the `value` to the attribute controller, as specified
    /// `emulation_mode` and `index`.
    pub fn write(
        &mut self,
        emulation_mode: EmulationMode,
        index: AttributeControllerIndex,
        value: u8,
    ) {
        self.toggle_index(emulation_mode);
        self.set_index(index);
        unsafe {
            self.arx_index.write(value);
        }
    }

    /// Video Enable. Note that In the VGA standard, this is called the "Palette Address Source" bit.
    /// Clearing this bit will cause the VGA display data to become all 00 index values. For the default
    /// palette, this will cause a black screen. The video timing signals continue. Another control bit will
    /// turn video off and stop the data fetches.
    ///
    /// 0 = Disable. Attribute controller color registers (AR[00:0F]) can be accessed by the CPU.
    ///
    /// 1 = Enable. Attribute controller color registers (AR[00:0F]) are inaccessible by the CPU.
    pub fn blank_screen(&mut self, emulation_mode: EmulationMode) {
        self.toggle_index(emulation_mode);
        let arx_index_value = unsafe { self.arx_index.read() };
        unsafe {
            self.arx_index.write(arx_index_value & 0xDF);
        }
    }

    /// Video Enable. Note that In the VGA standard, this is called the "Palette Address Source" bit.
    /// Clearing this bit will cause the VGA display data to become all 00 index values. For the default
    /// palette, this will cause a black screen. The video timing signals continue. Another control bit will
    /// turn video off and stop the data fetches.
    ///
    /// 0 = Disable. Attribute controller color registers (AR[00:0F]) can be accessed by the CPU.
    ///
    /// 1 = Enable. Attribute controller color registers (AR[00:0F]) are inaccessible by the CPU.
    pub fn unblank_screen(&mut self, emulation_mode: EmulationMode) {
        self.toggle_index(emulation_mode);
        let arx_index_value = unsafe { self.arx_index.read() };
        unsafe {
            self.arx_index.write(arx_index_value | 0x20);
        }
    }

    fn set_index(&mut self, index: AttributeControllerIndex) {
        unsafe {
            self.arx_index.write(u8::from(index));
        }
    }

    fn toggle_index(&mut self, emulation_mode: EmulationMode) {
        let st01_read = match emulation_mode {
            EmulationMode::Cga => &mut self.st01_read_cga,
            EmulationMode::Mda => &mut self.st01_read_mda,
        };
        unsafe {
            st01_read.read();
        }
    }
}
