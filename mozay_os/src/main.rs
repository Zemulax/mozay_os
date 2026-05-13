#![no_std]
#![no_main] 
#![feature(custom_test_frameworks)]
#![test_runner(mozay_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use mozay_os::println;
use core::panic::PanicInfo;


#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Hello, world{}", "!");

    mozay_os::init();// Initialize the Interrupt Descriptor Table (IDT)
    x86_64::instructions::interrupts::int3(); // Trigger a breakpoint exception
    
    #[cfg(test)] 
    test_main(); 
    
    println!("It did not crash!");
    loop {}
}


#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}


#[cfg(test)] 
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    mozay_os::test_panic_handler(info); 
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}