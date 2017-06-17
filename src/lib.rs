#![feature(lang_items)]
#![feature(const_fn)]
#![feature(unique)]
#![feature(alloc)]
#![feature(asm)]
#![feature(abi_x86_interrupt)]
#![feature(collections)]
#![feature(drop_types_in_const)]
#![feature(box_syntax)]
#![feature(naked_functions)]

#![no_std]

extern crate x86;
extern crate x86_64;
extern crate rlibc;
extern crate spin;
extern crate multiboot2;
extern crate alloc_opsys;
extern crate alloc;

extern crate collections;

#[macro_use]
extern crate bitflags;

#[macro_use]
extern crate once;

#[macro_use]
mod vga_buffer;

#[macro_use]
mod cpu;

mod memory;
mod interrupts;
mod drivers;
mod io;
mod schedule;
mod kernel;

// Main entry point, need no_mangle so we can call from assembly
// Extern to abide with C calling convention
#[no_mangle]
pub extern "C" fn kernel_main(multiboot_info_address: usize) {
    // Initialise the hardware
    init_cpu();

    // Initialise the memory paging and instantiate a new memory manager
    let memory_manager = memory::init(multiboot_info_address);

    // Setup the heap allocator
    alloc_opsys::init();

    vga_buffer::clear_screen();

    // Setup the kernel
    kernel::init(memory_manager);

    // Initialize interrupts
    interrupts::init();

    kprintln!("opsys v{}", "0.0.1");

    // TODO consider switching contexts here so we no-longer have a dangling task.
    // Also, put this in a function somewhere...
    hang!();
}

fn init_cpu() {
    // Configure the CPU flags as required
    enable_nxe_bit(); // Allow the NO_EXECUTABLE flag
    enable_write_protect_bit(); // Ensure non-writable by default
}

fn enable_nxe_bit() {
    use x86::msr::{IA32_EFER, rdmsr, wrmsr};

    let nxe_bit = 1 << 11;
    unsafe {
        let efer = rdmsr(IA32_EFER);
        wrmsr(IA32_EFER, efer | nxe_bit);
    }
}

fn enable_write_protect_bit() {
    use x86::controlregs::{cr0, cr0_write};

    let wp_bit = 1 << 16;
    unsafe { cr0_write(cr0() | wp_bit) };
}

// For stack-unwinding, not supported currently
#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

// For panic!
#[no_mangle]
#[lang = "panic_fmt"]
pub extern "C" fn panic_fmt(fmt: core::fmt::Arguments, file: &str, line: u32) -> ! {
    kprintln!("\n\nPANIC in {} at line {}:", file, line);
    kprintln!("    {}", fmt);

    // Hang here.
    loop {}
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn _Unwind_Resume() -> ! {
    // Hang here.
    loop {}
}
