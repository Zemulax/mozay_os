`CPU Exceptions — High‑Yield Notes (Pre‑Implementation)`

1. **What CPU Exceptions Are**
   Exceptions signal errors during instruction execution (e.g., divide‑by‑zero, invalid memory access).

When an exception occurs, the CPU interrupts the current instruction and jumps to a specific handler based on the exception type.

Common Exception Types
Page Fault — illegal memory access (unmapped page, write to read‑only page).

Invalid Opcode — instruction not supported by CPU.

General Protection Fault — broad category: privilege violations, writing reserved fields, etc.

Double Fault — an exception occurs while handling another exception or no handler exists.

Triple Fault — exception during double‑fault handling → CPU resets (fatal).

2. **Interrupt Descriptor Table (IDT)**
   The IDT is a hardware‑defined table mapping exception numbers → handler functions.

IDT Entry Structure (16 bytes)
Each entry contains:

Function pointer (split across multiple fields)

GDT selector (which code segment to use)

Options (privilege level, gate type, present bit, stack switching rules)

Important Option Bits
IST Index (0–2) — choose alternate stack (optional).

Gate Type — interrupt gate disables interrupts; trap gate doesn’t.

DPL — required privilege level to invoke handler.

Present — must be 1 or CPU raises a double fault.

Exception Vector Numbers
Each exception has a fixed index (e.g., Page Fault = 14).
CPU uses this index to fetch the correct IDT entry.

3. **What Happens When an Exception Occurs**
   The CPU performs these steps:

Pushes registers (RIP, RFLAGS, etc.)

Reads IDT entry for the exception vector

Checks “present” bit

Disables interrupts if using an interrupt gate

Loads CS from GDT selector

Jumps to handler function

This is all automatic hardware behavior.

4. **Handler Function Types**
   The x86_64 crate provides an InterruptDescriptorTable struct with typed fields:

HandlerFunc — for exceptions without error codes

HandlerFuncWithErrCode — for exceptions that push an error code

PageFaultHandlerFunc — special signature for page faults

Example type:

Code
type HandlerFunc = extern "x86-interrupt" fn(\_: InterruptStackFrame);

5. **Why a Special Calling Convention? (x86‑interrupt)**
   Normal function calls rely on:

Caller‑saved registers

Known argument passing rules

Return address pushed by call

Exceptions break these assumptions because:

They can occur at any instruction

You cannot prepare the stack or save registers beforehand

You cannot clobber registers that the interrupted code expects

Solution: x86‑interrupt ABI
Guarantees all registers are preserved

Knows how to read arguments from the interrupt stack frame

Returns using iretq, not ret

Handles stack alignment and error‑code differences

6. **Preserved vs Scratch Registers (Normal ABI)**
   (Useful for understanding why exceptions need special rules)

Preserved (callee‑saved)
rbp, rbx, rsp, r12–r15

Scratch (caller‑saved)
rax, rcx, rdx, rsi, rdi, r8–r11

Normal functions rely on this split.
Exception handlers cannot, because the interrupted code didn’t get a chance to save anything.

7. **Interrupt Stack Frame**
   When an exception occurs, the CPU pushes:

SS and RSP (old stack pointer)

RFLAGS

CS and RIP

Error code (only for some exceptions)

This forms the Interrupt Stack Frame, passed to handlers as InterruptStackFrame.

Why this matters
Lets the handler know where the exception happened

Allows the CPU to restore the exact previous state on return

Enables debugging (e.g., breakpoint exception prints RIP)

8. **Behind the Scenes (What x86‑interrupt ABI Handles for You)**
   The ABI automatically manages:

Argument retrieval from the stack

Using iretq instead of ret

Error code handling (only for some exceptions)

Stack realignment (16‑byte alignment requirement)

This hides the complexity of raw exception handling.

**Summary (Ultra‑Condensed)**
Exceptions are CPU‑detected errors that trigger automatic jumps to handlers.

Handlers are stored in the IDT, which must follow a strict hardware format.

The x86‑interrupt calling convention ensures all registers are preserved and uses iretq to return.

The CPU builds an interrupt stack frame containing RIP, RFLAGS, and possibly an error code.

Some exceptions push error codes; handler signatures must match.

The x86_64 crate provides a safe, typed abstraction for building the IDT.

**CPU Exceptions — Implementation Summary (From “Implementation” to End)**
(Based on the page you’re reading)

1. Creating the interrupts Module
   A new module src/interrupts.rs is introduced.

Inside it, you start with a function init_idt() that creates a fresh InterruptDescriptorTable.

rust
let mut idt = InterruptDescriptorTable::new();
This is the foundation: you’re building the IDT your CPU will use.

2. Adding the Breakpoint Handler
   The first real handler you implement is for the breakpoint exception (vector 3).

Why breakpoint?

It’s safe, simple, and easy to trigger manually using int3.

Perfect for testing your exception pipeline.

Handler Setup
rust
idt.breakpoint.set_handler_fn(breakpoint_handler);
Handler Function
rust
extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}
This prints:

A message

The interrupt stack frame (RIP, RFLAGS, etc.)

Compiler Error
You get an error because extern "x86-interrupt" is unstable.
Fix: add this to the top of lib.rs:

rust #![feature(abi_x86_interrupt)] 3. Loading the IDT
You try:

rust
idt.load();
But Rust complains:
“idt does not live long enough”

Why?

load() requires a &'static self.

The CPU will keep using this IDT forever.

A stack‑allocated IDT would be dropped after init_idt() returns → unsafe.

4. Why Not static?
   You try:

rust
static IDT: InterruptDescriptorTable = InterruptDescriptorTable::new();
But statics are immutable, so you can’t modify entries.

Then you try:

rust
static mut IDT: InterruptDescriptorTable = InterruptDescriptorTable::new();
This works but is:

Unsafe

Error‑prone

Requires unsafe every time you touch it

5. The Correct Solution: lazy_static!
   lazy_static! lets you create a static, mutable, safely-initialized IDT.

rust
lazy_static! {
static ref IDT: InterruptDescriptorTable = {
let mut idt = InterruptDescriptorTable::new();
idt.breakpoint.set_handler_fn(breakpoint_handler);
idt
};
}
Then:

rust
pub fn init_idt() {
IDT.load();
}
This gives you:

'static lifetime

No unsafe

Clean initialization logic

6. Hooking It Into the Kernel
   You add a general init() function in lib.rs:

rust
pub fn init() {
interrupts::init_idt();
}
Then call it from \_start():

rust
blog_os::init();
x86_64::instructions::interrupts::int3();
Result
When you run QEMU:

The breakpoint exception fires

Your handler prints the stack frame

Execution returns normally

"It did not crash!" prints

This confirms your exception pipeline works.

7. Adding a Test
   You update the test \_start to also call init() so the IDT is loaded during tests.

Then you add:

rust #[test_case]
fn test_breakpoint_exception() {
x86_64::instructions::interrupts::int3();
}
If execution continues after int3(), the test passes.

8. “Too Much Magic?”
   The page acknowledges that:

The x86-interrupt ABI

The InterruptDescriptorTable abstraction

…hide a lot of complexity.

If you want the raw, low‑level version:

There’s a separate series using naked functions and a manually‑constructed IDT.

But for now, the abstraction is clean and safe.

9. What’s Next
   The next post covers:

Double faults

How to avoid triple faults (which reboot the system)

Setting up a dedicated stack for double‑fault handling

Ultra‑Condensed Implementation Summary
Create interrupts module

Build IDT using InterruptDescriptorTable

Add breakpoint handler using extern "x86-interrupt"

Enable unstable ABI feature

Use lazy_static! to store IDT with 'static lifetime

Load IDT in init()

Trigger int3 to test

Add test case to ensure handler works
