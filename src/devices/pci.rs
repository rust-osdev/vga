use x86_64::instructions::port::Port;

const BUSSES: u32 = 256;
const SLOTS: u32 = 32;
const FUNCTIONS: u32 = 8;

const CONFIG_ADDRESS: u16 = 0xCF8;
const DATA_ADDRESS: u16 = 0xCFC;

#[derive(Debug, Copy, Clone)]
pub(crate) struct PciDevice {
    vendor_id: u16,
    device_id: u16,
    command: u16,
    status: u16,
    revision_id: u8,
    prog_if: u8,
    sub_class: u8,
    base_class: u8,
    cache_line_size: u8,
    latency_timer: u8,
    header_type: u8,
    bist: u8,
    pub(crate) base_addresses: [u32; 6],
    cis: u32,
    sub_vendor_id: u16,
    sub_system_id: u16,
    rom_base_address: u32,
    reserved_2: [u32; 2],
    interrupt_line: u8,
    interrupt_pin: u8,
    minimum_grant: u8,
    maximum_latency: u8,
}

pub(crate) fn find_pci_device(device_id: u32) -> Option<PciDevice> {
    for bus in 0..BUSSES {
        for slot in 0..SLOTS {
            for function in 0..FUNCTIONS {
                let address = (bus << 16) | (slot << 11) | (function << 8) | 0x8000_0000;
                if read_offset(address, 0) == device_id {
                    return Some(read_device(address));
                }
            }
        }
    }
    None
}

fn read_device(address: u32) -> PciDevice {
    let (device_id, vendor_id) = read_2_words(address, 0x00);
    let (status, command) = read_2_words(address, 0x04);
    let (base_class, sub_class, prog_if, revision_id) = read_4_bytes(address, 0x08);
    let (bist, header_type, latency_timer, cache_line_size) = read_4_bytes(address, 0x0C);
    let mut base_addresses = [0u32; 6];
    base_addresses[0] = read_offset(address, 0x10);
    base_addresses[1] = read_offset(address, 0x14);
    base_addresses[2] = read_offset(address, 0x18);
    base_addresses[3] = read_offset(address, 0x1C);
    base_addresses[4] = read_offset(address, 0x20);
    base_addresses[5] = read_offset(address, 0x24);
    let cis = read_offset(address, 0x28);
    let (sub_system_id, sub_vendor_id) = read_2_words(address, 0x2C);
    let rom_base_address = read_offset(address, 0x30);
    let mut reserved_2 = [0u32; 2];
    reserved_2[0] = read_offset(address, 0x34);
    reserved_2[1] = read_offset(address, 0x38);
    let (maximum_latency, minimum_grant, interrupt_pin, interrupt_line) =
        read_4_bytes(address, 0x3C);

    PciDevice {
        vendor_id,
        device_id,
        status,
        command,
        base_class,
        sub_class,
        prog_if,
        revision_id,
        bist,
        header_type,
        latency_timer,
        cache_line_size,
        base_addresses,
        cis,
        sub_system_id,
        sub_vendor_id,
        rom_base_address,
        reserved_2,
        maximum_latency,
        minimum_grant,
        interrupt_pin,
        interrupt_line,
    }
}

fn read_2_words(address: u32, offset: u32) -> (u16, u16) {
    let value = read_offset(address, offset);
    let high_word = (value >> 16) as u16;
    let low_word = value as u16;
    (high_word, low_word)
}

fn read_4_bytes(address: u32, offset: u32) -> (u8, u8, u8, u8) {
    let value = read_offset(address, offset);
    let high_word_high_byte = (value >> 24) as u8;
    let high_word_low_byte = (value >> 16) as u8;
    let low_word_high_byte = (value >> 8) as u8;
    let low_wird_low_byte = value as u8;
    (
        high_word_high_byte,
        high_word_low_byte,
        low_word_high_byte,
        low_wird_low_byte,
    )
}

fn read_offset(mut address: u32, offset: u32) -> u32 {
    let mut config_address = Port::new(CONFIG_ADDRESS);
    let mut data_address = Port::new(DATA_ADDRESS);
    address |= offset & 0xFC;
    unsafe {
        config_address.write(address);
        data_address.read()
    }
}
