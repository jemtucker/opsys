use super::*;

use memory::paging::Page;
use memory::paging::ActivePageTable;

pub struct MemoryManager {
    frame_allocator: AreaFrameAllocator,
    active_table: ActivePageTable,
    next_page: Page,
}

impl MemoryManager {
    pub fn new(multiboot_info_address: usize) -> MemoryManager {
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

        kprintln!("kernel start: 0x{:x}, kernel end: 0x{:x}",
                  kernel_start,
                  kernel_end);
        kprintln!("multiboot start: 0x{:x}, multiboot end: 0x{:x}",
                  multiboot_start,
                  multiboot_end);

        let mut frame_allocator = AreaFrameAllocator::new(kernel_start as usize,
                                                          kernel_end as usize,
                                                          multiboot_start,
                                                          multiboot_end,
                                                          memory_map_tag.memory_areas());

        let mut active_table = paging::remap_the_kernel(&mut frame_allocator, boot_info);

        use super::paging::Page;
        use alloc_opsys::{HEAP_START, HEAP_SIZE};

        let heap_start_page = Page::containing_address(HEAP_START);
        let heap_end_page = Page::containing_address(HEAP_START + HEAP_SIZE - 1);

        for page in Page::range_inclusive(heap_start_page, heap_end_page) {
            active_table.map(page, paging::WRITABLE, &mut frame_allocator);
        }

        let next_page = heap_end_page.next_page();

        MemoryManager {
            frame_allocator: frame_allocator,
            active_table: active_table,
            next_page: next_page,
        }
    }

    pub fn allocate_pages_with_guard(&mut self, num: u8) -> usize {
        // Allocate the pages
        let start = self.allocate_pages(num);

        // Allocate a guard page by skipping to the next page.
        let next_page = self.next_page.next_page();
        self.next_page = next_page;

        start
    }

    fn allocate_pages(&mut self, num: u8) -> usize {
        let start = self.allocate_page();
        for _ in 1..num {
            self.allocate_page();
        }

        // Return the start address of the page we just mapped
        start
    }

    fn allocate_page(&mut self) -> usize {
        let next_page = self.next_page.next_page();
        let page = self.next_page;
        self.next_page = next_page;

        // Map the new page
        self.active_table.map(page, paging::WRITABLE, &mut self.frame_allocator);

        self.next_page.start_address()
    }
}
