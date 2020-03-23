use super::{Color16Bit, PlaneMask, GRX_DATA_ADDRESS, GRX_INDEX_ADDRESS};
use core::convert::TryFrom;
use x86_64::instructions::port::Port;

/// Represents a plane for the `GraphicsControllerIndex::ReadPlaneSelect` register.
#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
#[repr(u8)]
pub enum ReadPlane {
    /// Represents `Plane 0 (0x0)`.
    Plane0 = 0x0,
    /// Represents `Plane 1 (0x1)`.
    Plane1 = 0x1,
    /// Represents `Plane 2 (0x2)`.
    Plane2 = 0x2,
    /// Represents `Plane 3 (0x3)`.
    Plane3 = 0x3,
}

impl TryFrom<u8> for ReadPlane {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ReadPlane::Plane0),
            1 => Ok(ReadPlane::Plane1),
            2 => Ok(ReadPlane::Plane2),
            3 => Ok(ReadPlane::Plane3),
            _ => Err("ReadPlane only accepts values between 0-3!"),
        }
    }
}

impl From<ReadPlane> for u8 {
    fn from(value: ReadPlane) -> u8 {
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

/// Represents the graphics controller registers on vga hardware.
#[derive(Debug)]
pub struct GraphicsControllerRegisters {
    grx_index: Port<u8>,
    grx_data: Port<u8>,
}

impl GraphicsControllerRegisters {
    pub(crate) fn new() -> GraphicsControllerRegisters {
        GraphicsControllerRegisters {
            grx_index: Port::new(GRX_INDEX_ADDRESS),
            grx_data: Port::new(GRX_DATA_ADDRESS),
        }
    }

    /// Reads the current value from the graphics controller, as specified
    /// by `index`.
    pub fn read(&mut self, index: GraphicsControllerIndex) -> u8 {
        self.set_index(index);
        unsafe { self.grx_data.read() }
    }

    /// Writes the `value` to the graphics controller, as specified
    /// by `index.
    pub fn write(&mut self, index: GraphicsControllerIndex, value: u8) {
        self.set_index(index);
        unsafe {
            self.grx_data.write(value);
        }
    }

    /// Sets the read plane of the graphics controller, as specified by `read_plane`.
    pub fn write_read_plane(&mut self, read_plane: ReadPlane) {
        let read_plane = u8::from(read_plane) & 0x3;
        self.write(GraphicsControllerIndex::ReadPlaneSelect, read_plane);
    }

    /// Sets the value to use for `GraphicsControllerIndex::SetReset`,
    /// as spcified by `color`.
    pub fn write_set_reset(&mut self, color: Color16Bit) {
        let original_value = self.read(GraphicsControllerIndex::SetReset) & 0xF0;
        self.write(
            GraphicsControllerIndex::SetReset,
            original_value | u8::from(color),
        );
    }

    /// Sets which planes are effected by `GraphicsControllerIndex::SetReset`,
    /// as specified by `plane_mask`.
    pub fn write_enable_set_reset(&mut self, plane_mask: PlaneMask) {
        let original_value = self.read(GraphicsControllerIndex::EnableSetReset) & 0xF0;
        self.write(
            GraphicsControllerIndex::EnableSetReset,
            original_value | u8::from(plane_mask),
        );
    }

    fn set_index(&mut self, index: GraphicsControllerIndex) {
        unsafe {
            self.grx_index.write(u8::from(index));
        }
    }
}
