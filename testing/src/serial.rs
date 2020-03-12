use conquer_once::spin::Lazy;
use spinning_top::Spinlock;
use uart_16550::SerialPort;

pub static SERIAL1: Lazy<Spinlock<SerialPort>> = Lazy::new(|| {
    let mut serial_port = unsafe { SerialPort::new(0x3F8) };
    serial_port.init();
    Spinlock::new(serial_port)
});

pub fn print(args: ::core::fmt::Arguments) {
    use core::fmt::Write;
    SERIAL1
        .lock()
        .write_fmt(args)
        .expect("Printing to serial failed");
}

/// Prints to the host through the serial interface.
#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => {
        $crate::serial::print(format_args!($($arg)*));
    };
}

/// Prints to the host through the serial interface, appending a newline.
#[macro_export]
macro_rules! serial_println {
    () => ($crate::serial_print!("\n"));
    ($fmt:expr) => ($crate::serial_print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::serial_print!(concat!($fmt, "\n"), $($arg)*));
}
