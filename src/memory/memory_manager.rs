use super::paging;
use super::paging::Page;
use super::paging::ActivePageTable;

use super::area_frame_allocator::AreaFrameAllocator;

use super::stack_allocator::StackAllocator;
use super::stack_allocator::Stack;

use multiboot2;

/// Manager object for all kernel memory.
pub struct MemoryManager {
    frame_allocator: AreaFrameAllocator,
    active_table: ActivePageTable,
    stack_allocator: StackAllocator,
}

impl MemoryManager {
    /// Constructs a new `MemoryManager` using the multiboot header at the passed address.
    ///
    /// # Safety
    /// Only ONE `MemoryManager` object should ever be instantiated for the lifetime of the kernel.
    /// This is because the `MemoryManager` new call initializes and remaps the kernel memory.
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
            stack_allocator: StackAllocator::new(next_page),
        }
    }

    /// Allocate a new kernel stack.
    ///
    /// # Safety
    /// Any stacks allocated by the memory manager must be returned when finised with to avoid
    /// leaking resources.
    /// TODO This is super un-rusty, we should use RAII or something to ensure stacks deallocate
    /// themselves.
    pub fn allocate_stack(&mut self) -> Stack {
        self.stack_allocator.allocate(&mut self.active_table, &mut self.frame_allocator)
    }
}
