#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![reexport_test_harness_main = "test_main"]
#![test_runner(testing::test_runner)]

use core::panic::PanicInfo;
use testing::{gdt, interrupts, serial_print, serial_println};
use vga::{VideoMode, VGA};

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
fn set_mode_80x25() {
    serial_print!("mode 80x25... ");

    let mut vga = VGA.lock();
    vga.set_video_mode(VideoMode::Mode80x25);

    serial_println!("[ok]");
}
