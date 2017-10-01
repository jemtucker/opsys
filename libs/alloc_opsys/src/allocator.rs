use block::Block;

use core::mem::size_of;
use core::ptr::Unique;
use alloc::allocator::{Alloc, Layout, AllocErr};

/// The minimum allowed block size
pub const MIN_BLOCK_SIZE: usize = 50;

pub struct Allocator {
    block_head: Option<Unique<Block>>,
}

impl Allocator {
    /// Create an empty `Allocator`
    ///
    /// Creates an allocator with a null heap pointer. Empty Allocators must not used prior to
    /// initialization.
    pub const fn empty() -> Allocator {
        Allocator { block_head: None }
    }

    /// Create an `Allocator` for a memory buffer
    ///
    /// Creates a new `Allocator` for the memory buffer at address `heap` of size `size`.
    ///
    /// # Safety
    ///
    /// Memory address `heap` must be valid and contain at least `size` bytes of usable memory. The
    /// buffer must be at least `MIN_BLOCK_SIZE` bytes.
    pub unsafe fn new(heap: &mut [u8], size: usize) -> Allocator {
        let mut allocator = Allocator::empty();
        allocator.init(heap, size);
        allocator
    }

    /// Initialize an `Allocator`
    ///
    /// Initialize an `Allocator` with a memory buffer at address `heap` of size `size`.
    ///
    /// # Safety
    ///
    /// Memory address `heap` must be valid and contain at least `size` bytes of usable memory. The
    /// buffer must be at least `MIN_BLOCK_SIZE` bytes.
    pub unsafe fn init(&mut self, heap: &mut [u8], size: usize) {
        assert!(size > size_of::<Block>());

        let block = (&mut heap[0] as *mut u8) as *mut Block;
        (*block).prev = None;
        (*block).next = None;
        (*block).size = size - size_of::<Block>();
        (*block).free = true;

        self.block_head = Unique::new(block);
    }

    /// Return the next block of size `size` or greater.
    ///
    /// Iterates over all blocks untill one of size `size` or greater is found (that is free).
    /// Returns `AllocErr` on Out Of Memory or invalid `current` pointer.
    ///
    /// # Safety
    ///
    /// Pointer `current` must point to a valid Block.
    unsafe fn next_fit(
        &mut self,
        size: usize,
        current: *mut Block,
    ) -> Result<*mut Block, AllocErr> {
        let current_ref = match current.as_ref() {
            Some(cur) => Ok(cur),
            None => Err(AllocErr::Unsupported { details: "NULL block ptr" }),
        }?;

        if current_ref.free && current_ref.size >= size {
            Ok(current)
        } else {
            let block = match current_ref.next {
                Some(next) => Ok(next),
                None => {
                    // Out of memory.
                    let layout = Layout::from_size_align_unchecked(size, 0);
                    Err(AllocErr::Exhausted { request: layout })
                }
            }?;

            self.next_fit(size, block.as_ptr())
        }
    }
}

unsafe impl Alloc for Allocator {
    unsafe fn alloc(&mut self, layout: Layout) -> Result<*mut u8, AllocErr> {
        let size = layout.size();

        // TODO implement the alignment side of things

        // Find the next fitting block
        let block_head = self.block_head.unwrap().as_ptr();
        let block_ptr = self.next_fit(size, block_head)?;
        let block = block_ptr.as_mut().expect("Null Block Pointer");

        // Found a block. We now need to see how big it is. If after allocation it is going to
        // leave unused memory larger than MIN_BLOCK_SIZE then we chunk it up and create a new
        // free block in the space.
        if (block.size - size) >= MIN_BLOCK_SIZE {
            let next_block_size = block.size - size - size_of::<Block>();

            // The order of the next steps are crucial...

            // Set block to allocate size
            block.size = size;

            // Get a pointer to the next block and set it to point to whatever the original 'next'
            // was. This is because we are slotting this block in between the allocated block
            // and its neighbour.
            let next_block = block.next_ptr();
            (*next_block).size = next_block_size;
            (*next_block).prev = Unique::new(block as *mut Block);
            (*next_block).next = block.next;
            (*next_block).free = true;

            // Finally set the allocated block to point to our new block and complete the chain.
            block.next = Unique::new(next_block);
        }

        // Finally we mark the allocated block as used and return the data_pointer to the caller
        block.free = false;
        let alloc_pointer = block.data_pointer();
        Ok(alloc_pointer)
    }

    unsafe fn dealloc(&mut self, ptr: *mut u8, layout: Layout) {
        let block = get_block_ptr(ptr);

        // This is only for the unit tests...
        debug_assert!(block.size == layout.size());

        // Set the block as free
        block.free = true;

        // TODO Merge neighbours.
    }
}

// Get a pointer to the block that is encapsulating a given u8 pointer
unsafe fn get_block_ptr<'a>(ptr: *mut u8) -> &'a mut Block {
    let block_ptr = ptr.offset(-(size_of::<Block>() as isize)) as *mut Block;
    block_ptr.as_mut().expect("Null Block Pointer")
}
