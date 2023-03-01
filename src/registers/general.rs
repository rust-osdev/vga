use super::{
    FCR_CGA_WRITE_ADDRESS, FCR_MDA_WRITE_ADDRESS, FCR_READ_ADDRESS, MSR_READ_ADDRESS,
    MSR_WRITE_ADDRESS, ST00_READ_ADDRESS, ST01_READ_CGA_ADDRESS, ST01_READ_MDA_ADDRESS,
};
use x86_64::instructions::port::{PortReadOnly, PortWriteOnly};

/// Represents the general registers on vga hardware.
#[allow(dead_code)]
#[derive(Debug)]
pub struct GeneralRegisters {
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
    pub(crate) fn new() -> GeneralRegisters {
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

    /// Reads the current value from the miscellaneous output register.
    pub fn read_msr(&mut self) -> u8 {
        unsafe { self.msr_read.read() }
    }

    /// Writes the `value` to the miscellaneous output register.
    pub fn write_msr(&mut self, value: u8) {
        unsafe {
            self.msr_write.write(value);
        }
    }
}
