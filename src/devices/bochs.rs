use super::pci::{find_pci_device, PciDevice, PciHeader};
use x86_64::{instructions::port::Port, PhysAddr, VirtAddr};

const BOCHS_ID: u32 = 0x1111_1234;
const BOCHS_INDEX_PORT_ADDRESS: u16 = 0x01CE;
const BOCHS_DATA_PORT_ADDRESS: u16 = 0x01CF;

const VBE_DISPI_INDEX_XRES: u16 = 0x01;
const VBE_DISPI_INDEX_YRES: u16 = 0x02;
const VBE_DISPI_INDEX_BPP: u16 = 0x03;
const VBE_DISPI_INDEX_ENABLE: u16 = 0x04;

const VBE_DISPI_DISABLED: u16 = 0x00;
const VBE_DISPI_ENABLED: u16 = 0x01;
const VBE_DISPI_GETCAPS: u16 = 0x02;
const VBE_DISPI_LFB_ENABLED: u16 = 0x40;

#[derive(Debug, Copy, Clone)]
#[repr(u8)]
pub enum BitsPerPixel {
    Bpp4 = 0x04,
    Bpp8 = 0x08,
    Bpp15 = 0x0F,
    Bpp16 = 0x10,
    Bpp24 = 0x18,
    Bpp32 = 0x20,
}

impl From<u16> for BitsPerPixel {
    fn from(value: u16) -> BitsPerPixel {
        match value {
            0x04 => BitsPerPixel::Bpp4,
            0x08 => BitsPerPixel::Bpp8,
            0x0F => BitsPerPixel::Bpp15,
            0x10 => BitsPerPixel::Bpp16,
            0x18 => BitsPerPixel::Bpp24,
            0x20 => BitsPerPixel::Bpp32,
            _ => panic!("invalid bits per pixel value: {}", value),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Mode {
    width: u16,
    height: u16,
    bpp: BitsPerPixel,
}

impl Mode {
    pub const fn new(width: u16, height: u16, bpp: BitsPerPixel) -> Mode {
        Mode { width, height, bpp }
    }
}

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
    physical_address: PhysAddr,
    virtual_address: VirtAddr,
}

impl BochsDevice {
    pub fn new() -> Option<BochsDevice> {
        if let Some(pci_device) = find_pci_device(BOCHS_ID) {
            let index_port = Port::new(BOCHS_INDEX_PORT_ADDRESS);
            let data_port = Port::new(BOCHS_DATA_PORT_ADDRESS);
            let base_address = match pci_device.pci_header {
                PciHeader::PciHeaderType0 { base_addresses, .. } => base_addresses[0] & 0xFFFF_FFF0,
            };
            let physical_address = PhysAddr::new(base_address as u64);
            let virtual_address = VirtAddr::new(base_address as u64);
            Some(BochsDevice {
                pci_device,
                index_port,
                data_port,
                physical_address,
                virtual_address,
            })
        } else {
            None
        }
    }

    pub fn physical_address(&self) -> PhysAddr {
        self.physical_address
    }

    pub fn virtual_address(&self) -> VirtAddr {
        self.virtual_address
    }

    pub fn set_virtual_address(&mut self, virtual_address: VirtAddr) {
        self.virtual_address = virtual_address;
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

    fn disable_display(&mut self) {
        unsafe {
            self.index_port.write(VBE_DISPI_INDEX_ENABLE);
            self.data_port.write(VBE_DISPI_DISABLED);
        }
    }

    fn enable_display(&mut self) {
        unsafe {
            self.index_port.write(VBE_DISPI_INDEX_ENABLE);
            self.data_port
                .write(VBE_DISPI_ENABLED | VBE_DISPI_LFB_ENABLED);
        }
    }

    pub fn get_mode(&mut self) -> Mode {
        let width = self.get_width();
        let height = self.get_height();
        let bpp = self.get_bpp();
        Mode { width, height, bpp }
    }

    pub fn set_mode(&mut self, mode: Mode) {
        self.disable_display();
        self.set_width(mode.width);
        self.set_height(mode.height);
        self.set_bpp(mode.bpp);
        self.enable_display();
    }

    pub fn get_width(&mut self) -> u16 {
        unsafe {
            self.index_port.write(VBE_DISPI_INDEX_XRES);
            self.data_port.read()
        }
    }

    pub fn get_height(&mut self) -> u16 {
        unsafe {
            self.index_port.write(VBE_DISPI_INDEX_YRES);
            self.data_port.read()
        }
    }

    pub fn get_bpp(&mut self) -> BitsPerPixel {
        unsafe {
            self.index_port.write(VBE_DISPI_INDEX_BPP);
            BitsPerPixel::from(self.data_port.read())
        }
    }

    fn set_width(&mut self, width: u16) {
        unsafe {
            self.index_port.write(VBE_DISPI_INDEX_XRES);
            self.data_port.write(width);
        }
    }

    fn set_height(&mut self, height: u16) {
        unsafe {
            self.index_port.write(VBE_DISPI_INDEX_YRES);
            self.data_port.write(height);
        }
    }

    fn set_bpp(&mut self, bpp: BitsPerPixel) {
        unsafe {
            self.index_port.write(VBE_DISPI_INDEX_BPP);
            self.data_port.write(bpp as u16);
        }
    }
}
