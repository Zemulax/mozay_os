#![no_std] //disable Rust's standard library
#![no_main] //disable Rust's main function

use core::panic::PanicInfo;

// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"-_Welcome to Mozay_OS_-"; //a byte string literal

#[unsafe(no_mangle)] //don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    //the entry point of our kernel
    //named _start because that's the default entry point for a binary
    let vga_buffer = 0xb8000 as *mut u8; //the VGA text buffer is located at this memory address
    
    for (i, &byte) in HELLO.iter().enumerate(){
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte; //write the byte to the VGA buffer
            *vga_buffer.offset(i as isize * 2 + 1) = 0xdb; //set the color attribute (white on black)
        }
    }
    loop {}
}