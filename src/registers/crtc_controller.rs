use super::{
    EmulationMode, CRX_DATA_CGA_ADDRESS, CRX_DATA_MDA_ADDRESS, CRX_INDEX_CGA_ADDRESS,
    CRX_INDEX_MDA_ADDRESS,
};
use x86_64::instructions::port::Port;

/// Represents an index for the crtc controller registers.
#[derive(Debug, Copy, Clone)]
#[repr(u8)]
pub enum CrtcControllerIndex {
    /// Represents the `Horizontal Total` register index.
    HorizontalTotal = 0x00,
    /// Represents the `Horizontal Display Enable End` register index.
    HorizontalDisplayEnableEnd = 0x01,
    /// Represents the `Horizontal Blanking Start` register index.
    HorizontalBlankingStart = 0x02,
    /// Represents the `Horizontal Blanking End` register index.
    HorizontalBlankingEnd = 0x03,
    /// Represents the `Horizontal Sync Start` register index.
    HorizontalSyncStart = 0x04,
    /// Represents the `Horizontal Sync End` register index.
    HorizontalSyncEnd = 0x05,
    /// Represents the `Vertical Total` register index.
    VeritcalTotal = 0x06,
    /// Represents the `Overflow` register index.
    Overflow = 0x07,
    /// Represents the `Preset Row Scan` register index.
    PresetRowScan = 0x08,
    /// Represents the `Maximum Scan Line` register index.
    MaximumScanLine = 0x09,
    /// Represents the `Text Cursor Start` register index.
    TextCursorStart = 0x0A,
    /// Represents the `Text Cursor End` register index.
    TextCursorEnd = 0x0B,
    /// Represents the `Start Address High` register index.
    StartAddressHigh = 0x0C,
    /// Represents the `Start Address Low` register index.
    StartAddressLow = 0x0D,
    /// Represents the `Text Cursor Location High` register index.
    TextCursorLocationHigh = 0x0E,
    /// Represents the `Text Cursor Location Low` register index.
    TextCursorLocationLow = 0x0F,
    /// Represents the `Vertical Sync Start` register index.
    VerticalSyncStart = 0x10,
    /// Represents the `Vertical Sync End` register index.
    VerticalSyncEnd = 0x11,
    /// Represents the `Vertical Display Enable End` register index
    VerticalDisplayEnableEnd = 0x12,
    /// Represents the `Offset` register index.
    Offset = 0x13,
    /// Represents the `Underline Location` register index.
    UnderlineLocation = 0x14,
    /// Represents the `Vertical Blanking Start` register index.
    VerticalBlankingStart = 0x15,
    /// Represents the `Vertical Blanking End` register index.
    VerticalBlankingEnd = 0x16,
    /// Represents the `Mode Control` register index.
    ModeControl = 0x17,
    /// Represents the `Line Compare` register index.
    LineCompare = 0x18,
    /// Represents the `Memory Read Latch Data` register index.
    MemoryReadLatchData = 0x22,
    /// Represents the `Toggle State Of Attribute Controller` register index.
    ToggleStateOfAttributeController = 0x24,
}

impl From<CrtcControllerIndex> for u8 {
    fn from(value: CrtcControllerIndex) -> u8 {
        value as u8
    }
}

/// Represents the crtc controller registers on vga hardware.
#[derive(Debug)]
pub struct CrtcControllerRegisters {
    crx_index_cga: Port<u8>,
    crx_index_mda: Port<u8>,
    crx_data_cga: Port<u8>,
    crx_data_mda: Port<u8>,
}

impl CrtcControllerRegisters {
    pub(crate) fn new() -> CrtcControllerRegisters {
        CrtcControllerRegisters {
            crx_index_cga: Port::new(CRX_INDEX_CGA_ADDRESS),
            crx_index_mda: Port::new(CRX_INDEX_MDA_ADDRESS),
            crx_data_cga: Port::new(CRX_DATA_CGA_ADDRESS),
            crx_data_mda: Port::new(CRX_DATA_MDA_ADDRESS),
        }
    }

    /// Reads the current value from the crtc controller, as specified
    /// by `emulation_mode` and `index`.
    pub fn read(&mut self, emulation_mode: EmulationMode, index: CrtcControllerIndex) -> u8 {
        self.set_index(emulation_mode, index);
        unsafe { self.get_data_port(emulation_mode).read() }
    }

    /// Writes the `value` to the crtc_controller, as specified
    /// by `emulation_mode` and `index`.
    pub fn write(&mut self, emulation_mode: EmulationMode, index: CrtcControllerIndex, value: u8) {
        self.set_index(emulation_mode, index);
        unsafe {
            self.get_data_port(emulation_mode).write(value);
        }
    }

    fn set_index(&mut self, emulation_mode: EmulationMode, index: CrtcControllerIndex) {
        unsafe {
            self.get_index_port(emulation_mode).write(u8::from(index));
        }
    }

    fn get_data_port(&mut self, emulation_mode: EmulationMode) -> &mut Port<u8> {
        match emulation_mode {
            EmulationMode::Cga => &mut self.crx_data_cga,
            EmulationMode::Mda => &mut self.crx_data_mda,
        }
    }

    fn get_index_port(&mut self, emulation_mode: EmulationMode) -> &mut Port<u8> {
        match emulation_mode {
            EmulationMode::Cga => &mut self.crx_index_cga,
            EmulationMode::Mda => &mut self.crx_index_mda,
        }
    }
}
