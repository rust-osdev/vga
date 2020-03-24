use super::{Color16, GRX_DATA_ADDRESS, GRX_INDEX_ADDRESS};
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

/// Represents a write mode for vga hardware.
#[derive(Debug)]
#[repr(u8)]
pub enum WriteMode {
    /// Represents `WriteMode` 0.
    ///
    /// During a CPU write to the frame buffer, the
    /// addressed byte in each of the 4 memory planes is written with the
    /// CPU write data after it has been rotated by the number of counts
    /// specified in the `GraphicsControllerIndex::DataRotate` register. If, however, the bit(s)
    /// in the `GraphicsControllerIndex::EnableSetReset` register corresponding to one or
    /// more of the memory planes is set to 1, then those memory planes
    /// will be written to with the data stored in the corresponding bits in
    /// the `GraphicsControllerIndex::SetReset` register.
    Mode0 = 0x0,
    /// Represents `WriteMode` 1.
    ///
    /// During a CPU write to the frame buffer, the
    /// addressed byte in each of the 4 memory planes is written to with
    /// the data stored in the memory read latches. (The memory read
    /// latches stores an unaltered copy of the data last read from any
    /// location in the frame buffer.)
    Mode1 = 0x1,
    /// Represents `WriteMode` 2.
    ///
    /// During a CPU write to the frame buffer, the least
    /// significant 4 data bits of the CPU write data is treated as the color
    /// value for the pixels in the addressed byte in all 4 memory planes.
    /// The 8 bits of the `GraphicsControllerIndex::BitMask` register are used to selectively
    /// enable or disable the ability to write to the corresponding bit in
    /// each of the 4 memory planes that correspond to a given pixel. A
    /// setting of 0 in a bit in the Bit Mask Register at a given bit position
    /// causes the bits in the corresponding bit positions in the addressed
    /// byte in all 4 memory planes to be written with value of their
    /// counterparts in the memory read latches. A setting of 1 in a Bit
    /// Mask Register at a given bit position causes the bits in the
    /// corresponding bit positions in the addressed byte in all 4 memory
    /// planes to be written with the 4 bits taken from the CPU write data
    /// to thereby cause the pixel corresponding to these bits to be set to
    /// the color value.
    Mode2 = 0x2,
    /// Represents `WriteMode` 3.
    ///
    /// During a CPU write to the frame buffer, the CPU
    /// write data is logically ANDed with the contents of the `GraphicsControllerIndex::BitMask`
    /// register. The result of this ANDing is treated as the bit
    /// mask used in writing the contents of the `GraphicsControllerIndex::SetReset` register
    /// are written to addressed byte in all 4 memory planes.
    Mode3 = 0x3,
}

impl From<WriteMode> for u8 {
    fn from(value: WriteMode) -> u8 {
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
    pub fn write_set_reset(&mut self, color: Color16) {
        let original_value = self.read(GraphicsControllerIndex::SetReset) & 0xF0;
        self.write(
            GraphicsControllerIndex::SetReset,
            original_value | u8::from(color),
        );
    }

    /// Sets which bits are effected by `GraphicsControllerIndex::SetReset`,
    /// as specified by `bit_mask`.
    pub fn write_enable_set_reset(&mut self, bit_mask: u8) {
        let original_value = self.read(GraphicsControllerIndex::EnableSetReset) & 0xF0;
        self.write(
            GraphicsControllerIndex::EnableSetReset,
            original_value | bit_mask,
        );
    }

    /// Sets which mode the vga writes in, as specified by `write_mode`.
    pub fn set_write_mode(&mut self, write_mode: WriteMode) {
        let original_value = self.read(GraphicsControllerIndex::GraphicsMode) & 0xFC;
        self.write(
            GraphicsControllerIndex::GraphicsMode,
            original_value | u8::from(write_mode),
        );
    }

    /// Sets which bits are effected by certain operations, as specified
    /// by `bit_mask`.
    pub fn set_bit_mask(&mut self, bit_mask: u8) {
        self.write(GraphicsControllerIndex::BitMask, bit_mask);
    }

    fn set_index(&mut self, index: GraphicsControllerIndex) {
        unsafe {
            self.grx_index.write(u8::from(index));
        }
    }
}
