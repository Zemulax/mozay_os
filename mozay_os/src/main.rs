#![no_std] //disable Rust's standard library
#![no_main] //disable Rust's main function

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)] //don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    loop {}
}