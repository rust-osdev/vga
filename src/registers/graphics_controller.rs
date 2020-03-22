use super::{GRX_DATA_ADDRESS, GRX_INDEX_ADDRESS};
use x86_64::instructions::port::Port;

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

#[derive(Debug)]
pub struct GraphicsControllerRegisters {
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
