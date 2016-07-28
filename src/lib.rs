#![feature(lang_items)]
#![feature(const_fn)]
#![feature(unique)]
#![feature(alloc)]
#![feature(asm)]
#![feature(core_intrinsics)]

#![no_std]

#[macro_use]
extern crate x86;
extern crate rlibc;
extern crate spin;
extern crate multiboot2;
extern crate jem_alloc;
extern crate alloc;

//#[macro_use]
//extern crate collections;

#[macro_use]
extern crate bitflags;

#[macro_use]
extern crate once;

#[macro_use]
mod vga_buffer;
mod memory;
mod interrupts;

use memory::FrameAllocator;

// Main entry point, need no_mangle so we can call from assembly
// Extern to abide with C calling convention
#[no_mangle]
pub extern fn kernel_main(multiboot_info_address: usize) {

    vga_buffer::clear_screen();
    kprintln!("OpSys v{}", "0.0.1");

    init_cpu();
    memory::init(multiboot_info_address);
    interrupts::init();

    kprintln!("It did not crash!");

    use alloc::boxed::Box;
    let heap_test = Box::new(123);

    kprintln!("It still did not crash! {}", heap_test);

//    // Divide by zero
//    unsafe {
//        asm!("mov dx, 0; div dx" ::: "ax", "dx" : "volatile", "intel")
//    }

	loop { }
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
extern fn eh_personality() { }

// For panic!
#[lang = "panic_fmt"]
extern fn panic_fmt(fmt: core::fmt::Arguments, file: &str, line: u32) -> ! {
    kprintln!("\n\nPANIC in {} at line {}:", file, line);
    kprintln!("    {}", fmt);

    // Hang here.
    loop { }
}

#[no_mangle]
pub extern fn _Unwind_Resume() -> ! {
    loop {}
}