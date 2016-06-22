#![feature(lang_items)]
#![feature(const_fn)]
#![feature(unique)]

#![no_std]

extern crate rlibc;
extern crate spin;
extern crate multiboot2;

#[macro_use]
mod vga_buffer;

// Main entry point, need no_mangle so we can call from assembly
// Extern to abide with C calling convention
#[no_mangle]
pub extern fn kernel_main(multiboot_info_address: usize) {
	vga_buffer::clear_screen();
	
	log_boot_info(multiboot_info_address);

	loop { }
}

fn log_boot_info(multiboot_info_address: usize) {
	let boot_info = unsafe { multiboot2::load(multiboot_info_address) };

	// Memory areas
	let memory_map_tag = boot_info.memory_map_tag()
	    .expect("Memory map tag required");

	kprintln!("memory areas:");
	
	for area in memory_map_tag.memory_areas() {
	    kprintln!("    start: 0x{:x}, length: 0x{:x}",
	        area.base_addr, area.length);
	}

	// ELF Sections
	let elf_sections_tag = boot_info.elf_sections_tag()
	    .expect("Elf-sections tag required");

	kprintln!("kernel sections:");

	for section in elf_sections_tag.sections() {
	    kprintln!("    addr: 0x{:x}, size: 0x{:x}, flags: 0x{:x}",
	        section.addr, section.size, section.flags);
	}

	// Start and end of kernel
	let kernel_start = elf_sections_tag.sections().map(|s| s.addr)
	    .min().unwrap();
	let kernel_end = elf_sections_tag.sections().map(|s| s.addr + s.size)
	    .max().unwrap();

	kprintln!("Kernel Start: {}, Kernel End: {}", kernel_start, kernel_end);

	// Start and end of multiboot
	let multiboot_start = multiboot_info_address;
	let multiboot_end = multiboot_start + (boot_info.total_size as usize);

	kprintln!("Multiboot Start: {}, Multiboot End: {}", multiboot_start, multiboot_end);
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