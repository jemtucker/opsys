#![feature(lang_items)]
#![feature(const_fn)]
#![feature(unique)]
#![feature(alloc, collections)]

#![no_std]

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
mod vga_buffer;
mod memory;

use memory::FrameAllocator;

// Main entry point, need no_mangle so we can call from assembly
// Extern to abide with C calling convention
#[no_mangle]
pub extern fn kernel_main(multiboot_info_address: usize) {

    vga_buffer::clear_screen();
    kprintln!("OpSys v{}", "0.0.1");

    let boot_info = unsafe {
        multiboot2::load(multiboot_info_address)
    };
    let memory_map_tag = boot_info.memory_map_tag()
        .expect("Memory map tag required");
    let elf_sections_tag = boot_info.elf_sections_tag()
        .expect("Elf sections tag required");

    let kernel_start = elf_sections_tag.sections().map(|s| s.addr)
        .min().unwrap();
    let kernel_end = elf_sections_tag.sections().map(|s| s.addr + s.size)
        .max().unwrap();

    let multiboot_start = multiboot_info_address;
    let multiboot_end = multiboot_start + (boot_info.total_size as usize);

    kprintln!("kernel start: 0x{:x}, kernel end: 0x{:x}",
        kernel_start, kernel_end);
    kprintln!("multiboot start: 0x{:x}, multiboot end: 0x{:x}",
        multiboot_start, multiboot_end);

    let mut frame_allocator = memory::AreaFrameAllocator::new(
        kernel_start as usize, kernel_end as usize, multiboot_start,
        multiboot_end, memory_map_tag.memory_areas());

	log_boot_info(multiboot_info_address);

    // Configure the CPU flags as required
    enable_nxe_bit(); // Allow the NO_EXECUTABLE flag
    enable_write_protect_bit(); // Ensure non-writable by default

    memory::remap_the_kernel(&mut frame_allocator, boot_info);

    kprintln!("It did not crash!");

    use alloc::boxed::Box;
    let heap_test = Box::new(123);

    kprintln!("It still did not crash!");

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
	    kprintln!("    addr: 0x{:x}, size: {}, flags: 0x{:x}",
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

	let mut frame_allocator = memory::AreaFrameAllocator::new(
	    kernel_start as usize, kernel_end as usize, multiboot_start,
	    multiboot_end, memory_map_tag.memory_areas());

	// Test the paging code
	// memory::test_paging(&mut frame_allocator);
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