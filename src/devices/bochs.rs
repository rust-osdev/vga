use super::pci::{find_pci_device, PciDevice, PciHeader};
use x86_64::instructions::port::Port;

const BOCHS_ID: u32 = 0x1111_1234;
const BOCHS_INDEX_PORT_ADDRESS: u16 = 0x01CE;
const BOCHS_DATA_PORT_ADDRESS: u16 = 0x01CF;

const VBE_DISPI_INDEX_XRES: u16 = 0x01;
const VBE_DISPI_INDEX_YRES: u16 = 0x02;
const VBE_DISPI_INDEX_BPP: u16 = 0x03;
const VBE_DISPI_INDEX_ENABLE: u16 = 0x04;

const VBE_DISPI_GETCAPS: u16 = 0x02;

#[derive(Debug, Copy, Clone)]
pub struct Capabilities {
    width: u16,
    height: u16,
    bpp: u16,
}

#[derive(Debug)]
pub struct BochsDevice {
    index_port: Port<u16>,
    data_port: Port<u16>,
    pci_device: PciDevice,
}

impl BochsDevice {
    pub fn new() -> Option<BochsDevice> {
        if let Some(pci_device) = find_pci_device(BOCHS_ID) {
            let index_port = Port::new(BOCHS_INDEX_PORT_ADDRESS);
            let data_port = Port::new(BOCHS_DATA_PORT_ADDRESS);
            Some(BochsDevice {
                pci_device,
                index_port,
                data_port,
            })
        } else {
            None
        }
    }

    pub fn base_address(&self) -> u32 {
        match self.pci_device.pci_header {
            PciHeader::PciHeaderType0 { base_addresses, .. } => base_addresses[0] & 0xFFFF_FFF0,
        }
    }

    pub fn capabilities(&mut self) -> Capabilities {
        unsafe {
            // Save original value of VBE_DISPI_INDEX_ENABLE
            self.index_port.write(VBE_DISPI_INDEX_ENABLE);
            let original_value = self.data_port.read();
            self.data_port.write(VBE_DISPI_GETCAPS);

            // Read max width
            self.index_port.write(VBE_DISPI_INDEX_XRES);
            let width = self.data_port.read();

            // Read max height
            self.index_port.write(VBE_DISPI_INDEX_YRES);
            let height = self.data_port.read();

            // Read max bpp
            self.index_port.write(VBE_DISPI_INDEX_BPP);
            let bpp = self.data_port.read();

            // Restore original VBE_DISPI_INDEX_ENABLE
            self.index_port.write(VBE_DISPI_INDEX_ENABLE);
            self.data_port.write(original_value);

            Capabilities { width, height, bpp }
        }
    }
}
