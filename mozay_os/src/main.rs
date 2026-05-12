#![no_std] //disable Rust's standard library
#![no_main] //disable Rust's main function
#![feature(custom_test_frameworks)] //enable the custom test frameworks feature, which allows us to define our own test runner
#![test_runner(crate::test_runner)] //specify that our test runner function should be
#![reexport_test_harness_main = "test_main"] //re-export the test harness main function as test_main, which will be called by the test runner

mod vga_buffer; //include the VGA buffer module
mod serial; //include the serial module

use core::panic::PanicInfo;

// This function is called on panic.
#[cfg(not(test))] //only compile this function when not running tests
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info); //print the panic information to the VGA buffer
    loop {}
}

#[cfg(test)] //only compile this function when running tests
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("[Failed]"); //print the panic information to the serial port
    serial_println!("Error: {}", info); //print the panic information to the serial port
    exit_qemu(QemuExitCode::Failed); //exit QEMU with a failure code after a panic occurs during testing
    loop {}
}

#[unsafe(no_mangle)] //don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    println!("Hello, world{}", "!"); //use the println! macro to print a message to the VGA buffer with a newline
    
    test_main(); //call the test_main function, which will run our tests
    
    loop {}
}

#[cfg(test)] //only compile this function when running tests
pub fn test_runner(tests: &[&dyn Testable]) { //a simple test runner function that takes a slice of test functions and runs them
    serial_println!("Running {} tests", tests.len()); //print the number of tests being run to the serial port
    for test in tests { //iterate over each test function
        test.run(); //call the test function
    }
    exit_qemu(QemuExitCode::Success); //exit QEMU with a success code after all tests have passed
}

#[test_case] //mark this function as a test case
fn trivial_assertion() { //a simple test function that asserts that 1 equals 1, which should always pass
     assert_eq!(1, 1);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)] //derive some common traits for our Color enum
#[repr(u32)] //represent the enum as a u8, which allows us to convert the enum variants to their corresponding numeric values when creating color codes for the VGA buffer
pub enum QemuExitCode { //an enum representing the exit codes that QEMU can return when we exit the emulator
    Success = 0x10, //the exit code for a successful test run
    Failed = 0x11, //the exit code for a failed test run
}

pub fn exit_qemu(exit_code: QemuExitCode) { //a function that exits QEMU with a given exit code
    use x86_64::instructions::port::Port; //import the Port type from the x86_64 crate, which allows us to write to I/O ports
    
    unsafe {
        let mut port = Port::new(0xf4); //create a new Port for the QEMU exit port (0xf4)
        port.write(exit_code as u32); //write the exit code to the port, which will cause QEMU to exit with that code
    }
    
    loop {}
}

pub trait Testable { //a trait that represents a test case, which can be run and will print its name to the serial port
    fn run(&self) -> (); //a method that runs the test case
}

impl<T> Testable for T where T: Fn() { //implement the Testable trait for any type that implements the Fn() trait, which includes all functions that take no arguments and return nothing (i.e., test functions)
    fn run(&self) { //the run method prints the name of the test function to the serial port and then calls the function itself
        serial_print!("{}...\t", core::any::type_name::<T>()); //print the name of the test function to the serial port
        self(); //call the test function
        serial_println!("[ok]"); //print a message to the serial port indicating that the test passed
    }
}

#[test_case] //mark this function as a test case
fn test_println() { //a test function that tests the println! macro by printing some messages to the VGA buffer
    println!("test_println hdhdfhdfhd"); //print a message to the VGA buffer
}

#[test_case]
fn test_println_many() {
    for _ in 0..200 {
        println!("test_println_many output");
    }
}

