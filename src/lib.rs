#![feature(lang_items)]
#![no_std]

// Main entry point, need no_mangle so we can call from assembly
// Extern to abide with C calling convention
#[no_mangle]
pub extern fn kernel_main() {
	unsafe {
        let vga = 0xb8000 as *mut u64;

        *vga = 0x2f592f412f4b2f4f;
    };
}

// For stack-unwinding, not supported currently
#[lang = "eh_personality"] extern fn eh_personality() {}

// For panic!, just a non-returning function
#[lang = "panic_fmt"] extern fn panic_fmt() -> ! { loop{} }