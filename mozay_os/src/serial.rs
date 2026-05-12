use uart_16550::{Config, Uart16550Tty, backend::PioBackend}; //import the Uart16550 type from the uart_16550 crate, which provides an implementation of the 16550 UART for serial communication
use spin::Mutex; //import the Mutex type from the spin crate, which provides a simple mutex implementation that can be used in a no_std environment
use lazy_static::lazy_static; //import the lazy_static macro from the lazy_static crate, which allows us to define lazily initialized static variables

lazy_static! {
    pub static ref SERIAL1: Mutex<Uart16550Tty<PioBackend>> = 
    Mutex::new(unsafe{Uart16550Tty::new_port(0x3F8, Config::default()).expect("Failed to initialise UART")}); //a lazily initialized static instance of the Uart16550Tty type that represents the first serial port (COM1) at I/O port 0x3F8
}

#[doc(hidden)]
pub fn _print(args: ::core::fmt::Arguments) { //an internal function that takes formatted arguments and writes them to the serial port
    use core::fmt::Write;
    SERIAL1.lock().write_fmt(args).expect("Printing to serial failed");
}

/// Prints to the host through the serial interface.
#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => {
        $crate::serial::_print(format_args!($($arg)*)); //use the format_args! macro to format the arguments and pass them to the internal _print function, which will write the formatted string to the serial port
    };
}

/// Prints to the host through the serial interface, appending a newline.
#[macro_export]
macro_rules! serial_println { //a macro for printing to the serial port with a newline
    () => ($crate::serial_print!("\n"));
    ($fmt:expr) => ($crate::serial_print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::serial_print!(
        concat!($fmt, "\n"), $($arg)*));
}