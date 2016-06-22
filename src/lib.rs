#![feature(lang_items)]
#![feature(const_fn)]
#![feature(unique)]

#![no_std]

extern crate rlibc;
extern crate spin;

#[macro_use]
mod vga_buffer;

// Main entry point, need no_mangle so we can call from assembly
// Extern to abide with C calling convention
#[no_mangle]
pub extern fn kernel_main() {
	vga_buffer::clear_screen();

	kprintln!("Hello World!");

	loop {}
}

// For stack-unwinding, not supported currently
#[lang = "eh_personality"] extern fn eh_personality() {}

// For panic!, just a non-returning function
#[lang = "panic_fmt"] extern fn panic_fmt() -> ! { loop{} }