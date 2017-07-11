use spin::Mutex;

use super::allocator::Allocator;
use alloc::allocator::{Alloc, Layout, AllocErr};
use core::slice;

pub struct LockedAllocator {
    allocator: Mutex<Allocator>,
}

impl LockedAllocator {
    /// Create an empty `LockedAllocator`
    ///
    /// Creates a `LockedAllocator` with an underlying empty memory buffer. This should not be
    /// used for allocations.
    pub const fn empty() -> LockedAllocator {
        LockedAllocator { allocator: Mutex::new(Allocator::empty()) }
    }

    /// Initialize an `Allocator`
    ///
    /// Initialize an `Allocator` with a memory buffer at address `heap` of size `size`.
    ///
    /// # Safety
    ///
    /// Memory address `start` must be valid and contain at least `size` bytes of usable memory.
    /// The buffer must be at least `MIN_BLOCK_SIZE` bytes long.
    pub unsafe fn init(&self, start: usize, size: usize) {
        let heap_ptr = start as *mut u8;
        let mut heap_ref = slice::from_raw_parts_mut(heap_ptr, size);
        self.allocator.lock().init(heap_ref, size);
    }
}

unsafe impl<'a> Alloc for &'a LockedAllocator {
    unsafe fn alloc(&mut self, layout: Layout) -> Result<*mut u8, AllocErr> {
        self.allocator.lock().alloc(layout)
    }

    unsafe fn dealloc(&mut self, ptr: *mut u8, layout: Layout) {
        self.allocator.lock().dealloc(ptr, layout);
    }
}
