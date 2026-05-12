#![no_std] //disable Rust's standard library
#![no_main] //disable Rust's main function

mod vga_buffer; //include the VGA buffer module

use core::panic::PanicInfo;

// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("Panic occurred: {}", info); //print the panic information to the VGA buffer
    loop {}
}

#[unsafe(no_mangle)] //don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    println!("Hello, world!"); //use the println! macro to print a message to the VGA buffer with a newline
    panic!("panic message"); //trigger a panic to demonstrate the panic handler
    //loop {}
}