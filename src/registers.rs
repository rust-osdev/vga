//! Common registers used in vga programming.

use crate::colors::PALETTE_SIZE;
use x86_64::instructions::port::{Port, PortReadOnly, PortWriteOnly};

const ST00_READ_ADDRESS: u16 = 0x3C2;
const ST01_READ_CGA_ADDRESS: u16 = 0x3DA;
const ST01_READ_MDA_ADDRESS: u16 = 0x3BA;
const FCR_READ_ADDRESS: u16 = 0x3CA;
const FCR_CGA_WRITE_ADDRESS: u16 = 0x3DA;
const FCR_MDA_WRITE_ADDRESS: u16 = 0x3BA;
const MSR_READ_ADDRESS: u16 = 0x3CC;
const MSR_WRITE_ADDRESS: u16 = 0x3C2;

const SRX_INDEX_ADDRESS: u16 = 0x3C4;
const SRX_DATA_ADDRESS: u16 = 0x3C5;

const GRX_INDEX_ADDRESS: u16 = 0x3CE;
const GRX_DATA_ADDRESS: u16 = 0x3CF;

const ARX_INDEX_ADDRESS: u16 = 0x3C0;
const ARX_DATA_ADDRESS: u16 = 0x3C1;

const CRX_INDEX_CGA_ADDRESS: u16 = 0x3D4;
const CRX_INDEX_MDA_ADDRESS: u16 = 0x3B4;
const CRX_DATA_CGA_ADDRESS: u16 = 0x3D5;
const CRX_DATA_MDA_ADDRESS: u16 = 0x3B5;

const COLOR_PALETTE_DATA_ADDRESS: u16 = 0x3C9;
const COLOR_PALETTE_INDEX_READ_ADDRESS: u16 = 0x3C7;
const COLOR_PALETTE_INDEX_WRITE_ADDRESSS: u16 = 0x3C8;

/// Represents a vga emulation mode.
#[derive(Debug, Copy, Clone)]
#[repr(u8)]
pub enum EmulationMode {
    /// Represents a monochrome emulation mode.
    Mda = 0x0,
    /// Respresents a color emulation mode.
    Cga = 0x1,
}

impl From<u8> for EmulationMode {
    fn from(value: u8) -> EmulationMode {
        match value {
            0x0 => EmulationMode::Mda,
            0x1 => EmulationMode::Cga,
            _ => panic!("{} is an invalid emulation mode", value),
        }
    }
}

/// Represents an index for the seqeuncer registers.
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum SequencerIndex {
    /// Represents the `Sequencer Reset` register index.
    SequencerReset = 0x0,
    /// Represents the `Clocking Mode` register index.
    ClockingMode = 0x1,
    /// Represents the `Plane/Map` mask register index.
    PlaneMask = 0x2,
    /// Represents the `Character Font` register index.
    CharacterFont = 0x3,
    /// Represents the `Memory Mode` register index.
    MemoryMode = 0x4,
    /// Represents the `Horizontal Character Counter Reset` register index.
    CounterReset = 0x7,
}

impl From<SequencerIndex> for u8 {
    fn from(value: SequencerIndex) -> u8 {
        value as u8
    }
}

/// Represents an index for the graphics controller registers.
#[derive(Debug, Copy, Clone)]
#[repr(u8)]
pub enum GraphicsControllerIndex {
    /// Represents the `Set/Reset` register index.
    SetReset = 0x0,
    /// Represents the `Enable Set/Reset` register index.
    EnableSetReset = 0x1,
    /// Represents the `Color Compare` register index.
    ColorCompare = 0x2,
    /// Represents the `Data Rotate` register index.
    DataRotate = 0x3,
    /// Represents the `Read Plane Select` register index.
    ReadPlaneSelect = 0x4,
    /// Represents the `Graphics Mode` register index.
    GraphicsMode = 0x5,
    /// Represents the `Miscellaneous` register index.
    Miscellaneous = 0x6,
    /// Represents the `Color Don't Care` register index.
    ColorDontCare = 0x7,
    /// Represents the `Bit Mask` register index.
    BitMask = 0x8,
    /// Represents the `Address Mapping` register index.
    AddressMapping = 0x10,
    /// Represents the `Page Selector` register index.
    PageSelector = 0x11,
    /// Represents the `Software Flags` register index.
    SoftwareFlags = 0x18,
}

impl From<GraphicsControllerIndex> for u8 {
    fn from(value: GraphicsControllerIndex) -> u8 {
        value as u8
    }
}

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

#[derive(Debug)]
pub(crate) struct GeneralRegisters {
    st00_read: PortReadOnly<u8>,
    st01_read_cga: PortReadOnly<u8>,
    st01_read_mda: PortReadOnly<u8>,
    fcr_read: PortReadOnly<u8>,
    fcr_write_cga: PortWriteOnly<u8>,
    fcr_write_mda: PortWriteOnly<u8>,
    msr_read: PortReadOnly<u8>,
    msr_write: PortWriteOnly<u8>,
}

impl GeneralRegisters {
    pub fn new() -> GeneralRegisters {
        GeneralRegisters {
            st00_read: PortReadOnly::new(ST00_READ_ADDRESS),
            st01_read_cga: PortReadOnly::new(ST01_READ_CGA_ADDRESS),
            st01_read_mda: PortReadOnly::new(ST01_READ_MDA_ADDRESS),
            fcr_read: PortReadOnly::new(FCR_READ_ADDRESS),
            fcr_write_cga: PortWriteOnly::new(FCR_CGA_WRITE_ADDRESS),
            fcr_write_mda: PortWriteOnly::new(FCR_MDA_WRITE_ADDRESS),
            msr_read: PortReadOnly::new(MSR_READ_ADDRESS),
            msr_write: PortWriteOnly::new(MSR_WRITE_ADDRESS),
        }
    }

    pub fn read_msr(&mut self) -> u8 {
        unsafe { self.msr_read.read() }
    }

    pub fn write_msr(&mut self, value: u8) {
        unsafe {
            self.msr_write.write(value);
        }
    }
}

#[derive(Debug)]
pub(crate) struct SequencerRegisters {
    srx_index: Port<u8>,
    srx_data: Port<u8>,
}

impl SequencerRegisters {
    pub fn new() -> SequencerRegisters {
        SequencerRegisters {
            srx_index: Port::new(SRX_INDEX_ADDRESS),
            srx_data: Port::new(SRX_DATA_ADDRESS),
        }
    }

    pub fn read(&mut self, index: SequencerIndex) -> u8 {
        self.set_index(index);
        unsafe { self.srx_data.read() }
    }

    pub fn write(&mut self, index: SequencerIndex, value: u8) {
        self.set_index(index);
        unsafe {
            self.srx_data.write(value);
        }
    }

    fn set_index(&mut self, index: SequencerIndex) {
        unsafe {
            self.srx_index.write(u8::from(index));
        }
    }
}

#[derive(Debug)]
pub(crate) struct GraphicsControllerRegisters {
    grx_index: Port<u8>,
    grx_data: Port<u8>,
}

impl GraphicsControllerRegisters {
    pub fn new() -> GraphicsControllerRegisters {
        GraphicsControllerRegisters {
            grx_index: Port::new(GRX_INDEX_ADDRESS),
            grx_data: Port::new(GRX_DATA_ADDRESS),
        }
    }

    pub fn read(&mut self, index: GraphicsControllerIndex) -> u8 {
        self.set_index(index);
        unsafe { self.grx_data.read() }
    }

    pub fn write(&mut self, index: GraphicsControllerIndex, value: u8) {
        self.set_index(index);
        unsafe {
            self.grx_data.write(value);
        }
    }

    fn set_index(&mut self, index: GraphicsControllerIndex) {
        unsafe {
            self.grx_index.write(u8::from(index));
        }
    }
}

#[derive(Debug)]
pub(crate) struct AttributeControllerRegisters {
    arx_index: Port<u8>,
    arx_data: Port<u8>,
    st01_read_cga: Port<u8>,
    st01_read_mda: Port<u8>,
}

impl AttributeControllerRegisters {
    pub fn new() -> AttributeControllerRegisters {
        AttributeControllerRegisters {
            arx_index: Port::new(ARX_INDEX_ADDRESS),
            arx_data: Port::new(ARX_DATA_ADDRESS),
            st01_read_cga: Port::new(ST01_READ_CGA_ADDRESS),
            st01_read_mda: Port::new(ST01_READ_MDA_ADDRESS),
        }
    }

    pub fn read(&mut self, emulation_mode: EmulationMode, index: AttributeControllerIndex) -> u8 {
        self.toggle_index(emulation_mode);
        self.set_index(index);
        unsafe { self.arx_data.read() }
    }

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
}

#[derive(Debug)]
pub(crate) struct CrtcControllerRegisters {
    crx_index_cga: Port<u8>,
    crx_index_mda: Port<u8>,
    crx_data_cga: Port<u8>,
    crx_data_mda: Port<u8>,
}

impl CrtcControllerRegisters {
    pub fn new() -> CrtcControllerRegisters {
        CrtcControllerRegisters {
            crx_index_cga: Port::new(CRX_INDEX_CGA_ADDRESS),
            crx_index_mda: Port::new(CRX_INDEX_MDA_ADDRESS),
            crx_data_cga: Port::new(CRX_DATA_CGA_ADDRESS),
            crx_data_mda: Port::new(CRX_DATA_MDA_ADDRESS),
        }
    }

    pub fn read(&mut self, emulation_mode: EmulationMode, index: CrtcControllerIndex) -> u8 {
        self.set_index(emulation_mode, index);
        unsafe { self.get_data_port(emulation_mode).read() }
    }

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

#[derive(Debug)]
pub(crate) struct ColorPaletteRegisters {
    data_port: Port<u8>,
    index_read_port: Port<u8>,
    index_write_port: Port<u8>,
}

impl ColorPaletteRegisters {
    pub fn new() -> ColorPaletteRegisters {
        ColorPaletteRegisters {
            data_port: Port::new(COLOR_PALETTE_DATA_ADDRESS),
            index_read_port: Port::new(COLOR_PALETTE_INDEX_READ_ADDRESS),
            index_write_port: Port::new(COLOR_PALETTE_INDEX_WRITE_ADDRESSS),
        }
    }

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
