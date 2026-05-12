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
