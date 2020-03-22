use super::{SRX_DATA_ADDRESS, SRX_INDEX_ADDRESS};
use x86_64::instructions::port::Port;

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

#[derive(Debug)]
pub struct SequencerRegisters {
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
