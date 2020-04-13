use super::pci::{find_pci_device, PciDevice, PciHeader};
use crate::drawing::{Bresenham, Point, Rectangle};
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
const VBE_DISPI_BPP_32: u16 = 0x20;

#[derive(Debug, Copy, Clone, Default)]
pub struct Resolution {
    width: usize,
    height: usize,
}

impl Resolution {
    pub fn new(width: usize, height: usize) -> Resolution {
        Resolution { width, height }
    }
}

#[derive(Debug)]
pub struct BochsDevice {
    index_port: Port<u16>,
    data_port: Port<u16>,
    pci_device: PciDevice,
    physical_address: PhysAddr,
    virtual_address: VirtAddr,
    current_resolution: Resolution,
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
                current_resolution: Resolution::default(),
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

    pub fn capabilities(&mut self) -> Resolution {
        unsafe {
            // Save original value of VBE_DISPI_INDEX_ENABLE
            self.index_port.write(VBE_DISPI_INDEX_ENABLE);
            let original_value = self.data_port.read();
            self.data_port.write(VBE_DISPI_GETCAPS);

            // Read max width
            self.index_port.write(VBE_DISPI_INDEX_XRES);
            let width = self.data_port.read() as usize;

            // Read max height
            self.index_port.write(VBE_DISPI_INDEX_YRES);
            let height = self.data_port.read() as usize;

            // Restore original VBE_DISPI_INDEX_ENABLE
            self.index_port.write(VBE_DISPI_INDEX_ENABLE);
            self.data_port.write(original_value);

            Resolution { width, height }
        }
    }

    pub fn clear_screen(&self, color: u32) {
        let screen_size = self.current_resolution.width * self.current_resolution.height;
        let frame_buffer = self.virtual_address.as_mut_ptr::<u32>();
        for offset in 0..screen_size {
            unsafe {
                frame_buffer.add(offset).write_volatile(color);
            }
        }
    }

    pub fn draw_line(&self, start: Point<isize>, end: Point<isize>, color: u32) {
        for (x, y) in Bresenham::new(start, end) {
            self.set_pixel(x as usize, y as usize, color);
        }
    }

    pub fn draw_rectangle(&self, rectangle: &Rectangle, color: u32) {
        let p1 = (rectangle.left as isize, rectangle.top as isize);
        let p2 = (rectangle.left as isize, rectangle.bottom as isize);
        let p3 = (rectangle.right as isize, rectangle.bottom as isize);
        let p4 = (rectangle.right as isize, rectangle.top as isize);
        self.draw_line(p1, p2, color);
        self.draw_line(p2, p3, color);
        self.draw_line(p3, p4, color);
        self.draw_line(p4, p1, color);
    }

    pub fn fill_rectangle(&self, rectangle: &Rectangle, color: u32) {
        for y in rectangle.top..rectangle.bottom {
            for x in rectangle.left..rectangle.right {
                self.set_pixel(x, y, color);
            }
        }
    }

    pub fn set_pixel(&self, x: usize, y: usize, color: u32) {
        let offset = (y * self.current_resolution.width) + x;
        unsafe {
            self.virtual_address
                .as_mut_ptr::<u32>()
                .add(offset)
                .write_volatile(color);
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

    pub fn get_resolution(&mut self) -> Resolution {
        let width = self.get_width();
        let height = self.get_height();
        Resolution { width, height }
    }

    pub fn set_resolution(&mut self, resolution: Resolution) {
        self.disable_display();
        self.set_width(resolution.width);
        self.set_height(resolution.height);
        self.set_bpp();
        self.enable_display();
        self.current_resolution = resolution;
    }

    pub fn get_width(&mut self) -> usize {
        unsafe {
            self.index_port.write(VBE_DISPI_INDEX_XRES);
            self.data_port.read() as usize
        }
    }

    pub fn get_height(&mut self) -> usize {
        unsafe {
            self.index_port.write(VBE_DISPI_INDEX_YRES);
            self.data_port.read() as usize
        }
    }

    fn set_width(&mut self, width: usize) {
        unsafe {
            self.index_port.write(VBE_DISPI_INDEX_XRES);
            self.data_port.write(width as u16);
        }
    }

    fn set_height(&mut self, height: usize) {
        unsafe {
            self.index_port.write(VBE_DISPI_INDEX_YRES);
            self.data_port.write(height as u16);
        }
    }

    fn set_bpp(&mut self) {
        unsafe {
            self.index_port.write(VBE_DISPI_INDEX_BPP);
            self.data_port.write(VBE_DISPI_BPP_32);
        }
    }
}
