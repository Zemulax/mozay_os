#1st step is to create a Rust executable that does not link to the Rust standard library. This makes it possible to run the code on the bare metal without an operating system, which is essential for building an OS from scratch.

To write an OS Kernel, we require code that does not depend on any OS features. This means we can t use threads, heap memory, or any other features that require an operating system. We need to write code that can run on the bare metal, which is why we need to create a Rust executable that does not link to the Rust standard library. we will also write our own drivers.

despite that we cant use most of Rust standard libraries, we can still use some of the core features of Rust, such as its powerful type system, iterators, closures, pattern matching,option and result, string formating and memory safety guarantees. This allows us to write code that is both efficient and safe, which is essential for building a reliable operating system.

An executable that runs without an underlying operating system is called a "bare-metal" executable.

#Disabling the standard library
By default, Rust crates link the standard library which depends on the OS for features sch as threads, files or networking. Libc is also another dependency standard C library thats used to interact with the OS services. We can not use any OS-dependent libraries as our goal is to write an OS which means we must disable the automated linking of the Rust standard library and the C standard library.

We begin by creating a new Rust project using Cargo, the Rust package manager. We can do this by running the following command in our terminal:

`cargo new mozay_os --bin --edition=2024`

```
in the command, the --bin flag indicates that we want to create a binary executable, and the --edition flag specifies the Rust edition we want to use (in this case, 2024). This will create a new directory called mozay_os with the basic structure of a Rust project.
```

cargo toml contains the crate(a crate is a package of Rust code) configuration such as crate name, author, semantic version and dependencies. The src/main.rs file contains the main function which is the entry point of the Rust program. cargo build command compiles the Rust code and produces an executable file in the target/debug directory. We can run the executable using the cargo run command, which will execute the compiled binary.

The no_std attribute tells the Rust compiler not to link the standard library, and the no_main attribute tells it not to use the standard main function as the entry point of the program. This allows us to write our own entry point and avoid any dependencies on the standard library.
its written as follows: #[no_std]

Panic Implementation
When we disable the standard library, we also lose access to the default panic handler, which is responsible for handling panics (unexpected errors) in Rust. Therefore, we need to provide our own implementation of the panic handler. The panic handler is a function that takes a reference to a PanicInfo struct, which contains information about the panic, such as the file and line number where it occurred. In our implementation, we simply enter an infinite loop to prevent the program from crashing.

The eh_personality function is another required function when we disable the standard library. It is used by the Rust compiler to handle unwinding during panics. Since we are not using the standard library, we can provide an empty implementation of this function. unwind is a process that occurs when a panic happens, and it involves cleaning up the stack and resources before the program terminates. By providing an empty implementation of the eh_personality function, we are essentially telling the Rust compiler that we do not want to perform any unwinding during panics.

Language Item
In Rust, a language item is a special function or type that is required by the Rust compiler to perform certain operations. When we disable the standard library, we also lose access to some of the language items that are provided by the standard library. Therefore, we need to provide our own implementations of these language items. For example, the eh_personality function is a language item that is required for unwinding during panics.

Disabling Unwinding
unwind requires OS specific libraries therefore we dont want it for our OS. Rust provides an option to abort on panic instead of unwinding, which is more suitable for our use case. We can enable this option by adding the following line to our Cargo.toml file:

`[profile.dev]`
`panic = "abort"`

`[profile.release]`
`panic = "abort"`

This tells the Rust compiler to abort the program immediately when a panic occurs, instead of trying to unwind the stack. This is a more efficient way to handle panics in a bare-metal environment, where we do not have access to the resources needed for unwinding.

The Start Attribute
When we disable the standard library, we also lose access to the default entry point of the program, which is the main function. Therefore, we need to provide our own entry point for the program. We can do this by using the start attribute, which allows us to specify a custom entry point for our program. The start attribute is used to define a function that will be called when the program starts executing. This function is responsible for initializing the system and calling the main function of our OS kernel.
we need to define our own entry point directly by ovewriting the crt0 default Rust runtime. we do this by adding the following code to our src/main.rs file:
`#![no_main]`
The main doesnt make sense without an underlying runtime that calls it. We are now overwriting the os entrypoint with our own \_start function.

The start function uses an attribute called #[no_mangle], which tells the Rust compiler not to mangle the name of the function. This is important because we want to be able to call this function from our assembly code, and we need to ensure that the name of the function is preserved. The function is also marked as `extern C` to tell the compiler to use the C calling convention, which is necessary for interoperability with assembly code.
The ! return type of the function indicates that it does not return a value, which is appropriate for an entry point function. The function is responsible for initializing the system and calling the main function of our OS kernel, which will contain the main logic of our operating system.

Linker Errors
The linker is responsible for combining the compiled Rust code with any necessary libraries and creating the final executable. When we disable the standard library, we may encounter linker errors because the linker cannot find the necessary symbols that are provided by the standard library. To resolve these errors, we need to provide our own implementations of the required symbols, such as the panic handler and the entry point function. By providing these implementations, we can ensure that the linker can successfully create the final executable for our OS kernel. The default linker config assumes our program depends on the C runtime. We need to tell it not to include it. Recommended way to do this is to pass sets of arguments by building a bare metal target. another way it so pass the args to the linker.

Building for a Bare-Metal Target
Rust tries to build an executable for the currently platform by default. We ned to describe a different system environment by using a string called target triple. run rustc --version --verbose to see the triple for the host machine.
if we compile for target triple, linker and rust compiler assume an OS is present hence the linker errors. To avid these, we compile for a diff env with no underlying OS.
an example for such an env is the thumbv7em-none-eabihf target, which is a bare-metal target for ARM Cortex-M microcontrollers. add it to rustup by running the following command:
`rustup target add thumbv7em-none-eabihf`
and then we can compile our code for this target by running the following command:
`cargo build --target thumbv7em-none-eabihf`

    CHAPTER 2: BOOT PROCESS

A computer stores firmware code in a special type of memory called ROM (Read-Only Memory). This firmware code is responsible for initializing the hardware and loading the operating system into memory when the computer is turned on. The process of loading the operating system is called the boot process.
It then looks for a bootable device, such as a hard drive or a USB drive, and loads the bootloader from that device into memory. The bootloader is a small program that is responsible for loading the operating system kernel into memory and transferring control to it.

on x86, there are two main types of bootloaders: the legacy BIOS bootloader and the newer UEFI bootloader. The BIOS bootloader is the traditional bootloader that has been used for many years, while the UEFI bootloader is a newer standard that provides more features and capabilities.
The BIOS bootloader is responsible for initializing the hardware and loading the operating system kernel into memory.

BIOS BOOT
All x86 systems have support for BIOS booting inc newer UEFI systems. The BIOS boot process starts when the computer is powered on and the CPU begins executing code from a specific memory address, which is typically 0xFFFF0. This code is part of the BIOS firmware and is responsible for initializing the hardware and performing a power-on self-test (POST) to ensure that the system is functioning properly. Once BIOS finds bootable disks, it transfers control to the bootloader, which is typically located in the Master Boot Record (MBR) of the disk. The MBR is a special area of the disk that contains the partition table and the bootloader code. The bootloader then loads the operating system kernel into memory and transfers control to it. Most bootloaders are 512 bytes in size, which is the size of the MBR. The bootloader can be written in assembly language or in a high-level language like C or Rust, as long as it can fit within the 512-byte limit of the MBR. The bootloader is responsible for loading the operating system kernel into memory and transferring control to it, which allows the operating system to start running on the computer.

The Bootloaders has to determine the location of the kernel image on the disk and load it into memory. It also needs to switch the CPU from 16-bit real mode to 32-bit protected mode, and then to the 64-bit long mode where 64-bit registers and the complete main memory are available. It also queries certain information such as memory map from the BIOS and passes it to the OS kernel.

Multiboot Standard
The Multiboot standard is a specification for bootloaders that allows them to load and execute operating system kernels in a standardized way. It defines a set of requirements for bootloaders, such as how they should load the kernel into memory, how they should pass information to the kernel, and how they should handle different types of kernels. By following the Multiboot standard, bootloaders can ensure that they are compatible with a wide range of operating system kernels, which makes it easier for developers to create and distribute their own operating systems. The Multiboot standard is widely used in the OS development community and is supported by many popular bootloaders, such as GRUB and Limine.
to make a kernel multiboot compliant, we simply inser Multiboot header at the top of the kernel file.

A Minima Kernel
A minimal kernel is a simple operating system kernel that provides only the basic functionality needed to run a computer. It typically includes a bootloader, a basic memory manager, and a simple scheduler for managing processes. A minimal kernel is often used as a starting point for developing more complex operating systems, as it provides a foundation upon which additional features can be built. By starting with a minimal kernel, developers can focus on implementing the core functionality of the operating system before adding more advanced features.

Target Specification
Rust allows us to define our own target specification, which is a JSON file that describes the target architecture and environment for our OS kernel. This allows us to customize the build process and ensure that our kernel is built correctly for the target platform. The target specification can include information such as the CPU architecture, the memory layout, and the required linker flags. By defining our own target specification, we can ensure that our kernel is built correctly and can run on the intended hardware platform.
To define our own target specification, we can create a JSON file called `x86_64-blog_os.json` with the following content:

```json
{
  "llvm-target": "x86_64-unknown-none",
  "data-layout": "e-m:e-i64:64-f80:128-n8:16:32:64-S128",
  "arch": "x86_64",
  "os": "none",
  "vendor": "unknown",
  "linker-flavor": "ld.lld",
  "linker": "rust-lld"
}
```

VGA Text Mode
VGA Text mode is a simple way to print the screen. It allows us to write text directly to the screen by writing to a specific memory address. The VGA Text mode uses a specific format for the characters and their attributes, which allows us to control the color and style of the text. By using VGA Text mode, we can create a simple user interface for our OS kernel and display important information to the user.

VGA Text Buffer
To print to the screen in VGA Text mode, we need to write to a specific memory address called the VGA text buffer. The VGA text buffer is located at the memory address 0xB8000 and is used to store the characters and their attributes that are displayed on the screen. Each character is represented by a 16-bit value, where the lower 8 bits represent the ASCII code of the character and the upper 8 bits represent the color and style attributes. By writing to the VGA text buffer, we can control what is displayed on the screen and create a simple user interface for our OS kernel.
Each character in the VGA text buffer is represented by a 16-bit value, where:
0 - 7: ASCII code of the character
8 - 11: Foreground color
12 - 14: Background color
15: Blinking attribute
