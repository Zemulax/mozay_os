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

```

```
