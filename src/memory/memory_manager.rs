use super::paging::ActivePageTable;

use super::area_frame_allocator::AreaFrameAllocator;

use super::stack_allocator::StackAllocator;
use super::stack_allocator::Stack;

/// Manager object for all kernel memory.
pub struct MemoryManager {
    frame_allocator: AreaFrameAllocator,
    active_table: ActivePageTable,
    stack_allocator: StackAllocator,
}

impl MemoryManager {
    /// Constructs a new `MemoryManager`
    pub fn new(
        frame_allocator: AreaFrameAllocator,
        active_table: ActivePageTable,
        stack_allocator: StackAllocator,
    ) -> MemoryManager {
        MemoryManager {
            frame_allocator: frame_allocator,
            active_table: active_table,
            stack_allocator: stack_allocator,
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
        self.stack_allocator.allocate(
            &mut self.active_table,
            &mut self.frame_allocator,
        )
    }

    /// Deallocate a kernel stack.
    ///
    /// Marks a kernel stack as free and available for use by another thread.
    pub fn deallocate_stack(&mut self, stack: &Stack) {
        self.stack_allocator.deallocate(stack);
    }
}
