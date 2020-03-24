#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![reexport_test_harness_main = "test_main"]
#![test_runner(testing::test_runner)]

use core::panic::PanicInfo;
use testing::{gdt, interrupts, serial_print, serial_println};
use vga::colors::{DEFAULT_PALETTE, PALETTE_SIZE};
use vga::configurations::{
    VgaConfiguration, MODE_40X25_CONFIGURATION, MODE_40X50_CONFIGURATION,
    MODE_640X480X16_CONFIGURATION, MODE_80X25_CONFIGURATION,
};
use vga::vga::{Vga, VideoMode, VGA};

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    init();
    test_main();

    loop {}
}

fn init() {
    gdt::init();
    interrupts::init_idt();
    unsafe { interrupts::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    testing::test_panic_handler(info)
}

#[test_case]
fn set_mode_40x25() {
    serial_print!("mode 40x25... ");

    let mut vga = VGA.lock();
    vga.set_video_mode(VideoMode::Mode40x25);
    check_registers(&mut vga, &MODE_40X25_CONFIGURATION);

    serial_println!("[ok]");
}

#[test_case]
fn set_mode_40x50() {
    serial_print!("mode 40x50... ");

    let mut vga = VGA.lock();
    vga.set_video_mode(VideoMode::Mode40x50);
    check_registers(&mut vga, &MODE_40X50_CONFIGURATION);

    serial_println!("[ok]");
}

#[test_case]
fn set_mode_80x25() {
    serial_print!("mode 80x25... ");

    let mut vga = VGA.lock();
    vga.set_video_mode(VideoMode::Mode80x25);
    check_registers(&mut vga, &MODE_80X25_CONFIGURATION);

    serial_println!("[ok]");
}

#[test_case]
fn set_mode_640x480x16() {
    serial_print!("mode 640x480x16... ");

    let mut vga = VGA.lock();
    vga.set_video_mode(VideoMode::Mode640x480x16);
    check_registers(&mut vga, &MODE_640X480X16_CONFIGURATION);

    serial_println!("[ok]");
}

#[test_case]
fn load_palette() {
    serial_print!("load palette... ");

    let mut palette = [0u8; PALETTE_SIZE];
    let mut vga = VGA.lock();
    vga.color_palette_registers.load_palette(&DEFAULT_PALETTE);
    vga.color_palette_registers.read_palette(&mut palette);

    for i in 0..PALETTE_SIZE {
        assert_eq!(palette[i], DEFAULT_PALETTE[i]);
    }

    serial_println!("[ok]");
}

fn check_registers(vga: &mut Vga, configuration: &VgaConfiguration) {
    let emulation_mode = vga.get_emulation_mode();
    assert_eq!(
        vga.general_registers.read_msr(),
        configuration.miscellaneous_output
    );

    for (index, value) in configuration.sequencer_registers {
        assert_eq!(vga.sequencer_registers.read(*index), *value);
    }

    for (index, value) in configuration.crtc_controller_registers {
        assert_eq!(
            vga.crtc_controller_registers.read(emulation_mode, *index),
            *value
        );
    }

    for (index, value) in configuration.graphics_controller_registers {
        assert_eq!(vga.graphics_controller_registers.read(*index), *value);
    }

    for (index, value) in configuration.attribute_controller_registers {
        assert_eq!(
            vga.attribute_controller_registers
                .read(emulation_mode, *index),
            *value
        );
    }
}
