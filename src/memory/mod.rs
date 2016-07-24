mod paging;
pub mod area_frame_allocator;

pub use self::paging::test_paging;

pub use memory::area_frame_allocator::AreaFrameAllocator;

use multiboot2;
use self::paging::PhysicalAddress;

pub fn init(multiboot_info_address: usize) {
    // TODO - Ensure this function can only ever be called once.

    let boot_info = unsafe { multiboot2::load(multiboot_info_address) };

    let memory_map_tag = boot_info.memory_map_tag().expect("Memory map tag required");
    let elf_sections_tag = boot_info.elf_sections_tag().expect("Elf sections tag required");

    let kernel_start = elf_sections_tag.sections()
        .filter(|s| s.is_allocated())
        .map(|s| s.addr)
        .min()
        .unwrap();

    let kernel_end = elf_sections_tag.sections()
        .filter(|s| s.is_allocated())
        .map(|s| s.addr + s.size)
        .max()
        .unwrap();

    let multiboot_start = multiboot_info_address;
    let multiboot_end = multiboot_start + (boot_info.total_size as usize);

    kprintln!("kernel start: 0x{:x}, kernel end: 0x{:x}", kernel_start, kernel_end);
    kprintln!("multiboot start: 0x{:x}, multiboot end: 0x{:x}", multiboot_start, multiboot_end);

    let mut frame_allocator = AreaFrameAllocator::new(
        kernel_start as usize, kernel_end as usize, multiboot_start,
        multiboot_end, memory_map_tag.memory_areas());

    let mut active_table = paging::remap_the_kernel(&mut frame_allocator, boot_info);

    use self::paging::Page;
    use jem_alloc::{HEAP_START, HEAP_SIZE};

    let heap_start_page = Page::containing_address(HEAP_START);
    let heap_end_page = Page::containing_address(HEAP_START + HEAP_SIZE-1);

    for page in Page::range_inclusive(heap_start_page, heap_end_page) {
        active_table.map(page, paging::WRITABLE, &mut frame_allocator);
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Frame {
    number: usize,
}

pub const PAGE_SIZE: usize = 4096;

impl Frame {
	// TODO change to from_address?
    fn containing_address(address: usize) -> Frame {
        Frame{ number: address / PAGE_SIZE }
    }

    fn start_address(&self) -> PhysicalAddress {
	    self.number * PAGE_SIZE
	}

    fn clone(&self) -> Frame {
        Frame { number: self.number }
    }

    fn range_inclusive(start: Frame, end: Frame) -> FrameIter {
        FrameIter {
            start: start,
            end: end,
        }
    }
}

pub trait FrameAllocator {
    fn allocate_frame(&mut self) -> Option<Frame>;
    fn deallocate_frame(&mut self, frame: Frame);
}

struct FrameIter {
    start: Frame,
    end: Frame,
}

impl Iterator for FrameIter {
    type Item = Frame;

    fn next(&mut self) -> Option<Frame> {
        if self.start <= self.end {
            let frame = self.start.clone();
            self.start.number += 1;
            Some(frame)
        } else {
            None
        }
    }
 }