use super::{SRX_DATA_ADDRESS, SRX_INDEX_ADDRESS};
use bitflags::bitflags;
use core::convert::TryFrom;
use x86_64::instructions::port::Port;

bitflags! {
    /// Represents the plane masks of the `SequencerIndex::PlaneMask` register.
    pub struct PlaneMask: u8 {
        /// Represents none of the plane masks of vga memory.
        const NONE = 0b0000_0000;
        /// Represents `Plane0` of vga memory.
        const PLANE0 = 0b0000_0001;
        /// Represents `Plane1` of vga memory.
        const PLANE1 = 0b0000_0010;
        /// Represents `Plane2` of vga memory.
        const PLANE2 = 0b0000_0100;
        /// Represents `Plane3` of vga memory.
        const PLANE3 = 0b0000_1000;
        /// Represents all of the plane masks of vga memory.
        const ALL_PLANES = Self::PLANE0.bits() | Self::PLANE1.bits() | Self::PLANE2.bits() | Self::PLANE3.bits();
    }
}

impl TryFrom<u8> for PlaneMask {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(PlaneMask::PLANE0),
            1 => Ok(PlaneMask::PLANE1),
            2 => Ok(PlaneMask::PLANE2),
            3 => Ok(PlaneMask::PLANE3),
            _ => Err("PlaneMask only accepts values between 0-3!"),
        }
    }
}

impl From<PlaneMask> for u8 {
    fn from(value: PlaneMask) -> u8 {
        value.bits()
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

/// Represents the sequencer registers on vga hardware.
#[derive(Debug)]
pub struct SequencerRegisters {
    srx_index: Port<u8>,
    srx_data: Port<u8>,
}

impl SequencerRegisters {
    pub(crate) fn new() -> SequencerRegisters {
        SequencerRegisters {
            srx_index: Port::new(SRX_INDEX_ADDRESS),
            srx_data: Port::new(SRX_DATA_ADDRESS),
        }
    }

    /// Reads the current value from the sequencer, as specified by `index`.
    pub fn read(&mut self, index: SequencerIndex) -> u8 {
        self.set_index(index);
        unsafe { self.srx_data.read() }
    }

    /// Writes the `value` to the sequencer, as specified by `index`.
    pub fn write(&mut self, index: SequencerIndex, value: u8) {
        self.set_index(index);
        unsafe {
            self.srx_data.write(value);
        }
    }

    /// Sets the plane mask of the sequencer controller, as specified by `plane_mask`.
    pub fn set_plane_mask(&mut self, plane_mask: PlaneMask) {
        let original_value = self.read(SequencerIndex::PlaneMask) & 0xF0;
        self.write(
            SequencerIndex::PlaneMask,
            original_value | u8::from(plane_mask),
        );
    }

    fn set_index(&mut self, index: SequencerIndex) {
        unsafe {
            self.srx_index.write(u8::from(index));
        }
    }
}
