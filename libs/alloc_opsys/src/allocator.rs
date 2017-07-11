use block::Block;

use core::mem::size_of;
use alloc::allocator::{Alloc, Layout, AllocErr};

//const MIN_ALLOCATION: usize = 1;
//const MIN_BLOCK_SIZE: usize = size_of::<Block>() + MIN_ALLOCATION;
const MIN_BLOCK_SIZE: usize = 50;

// WIP NOTES...
// 1. Start with one massive block, break this up with every allocation. Walk for first fit.
// 2. Deallocate just flag as free
// 3. Merge free neighbours? - Maybe a GC function that runs periodically??
//
//
//

pub struct Allocator {
    block_head: *mut Block,
}

impl Allocator {
    // Create a new Allocator for a given heap. The heap must be at-least the size of a block
    // header.
    pub fn new(heap: &mut [u8], size: usize) -> Allocator {
        assert!(size > size_of::<Block>());
        let block = unsafe {
            let mut b = (&mut heap[0] as *mut u8) as *mut Block;
            (*b).prev = None;
            (*b).next = None;
            (*b).size = size - size_of::<Block>();
            (*b).free = true;
            b
        };

        Allocator { block_head: block }
    }

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

            self.next_fit(size, block)
        }
    }
}

unsafe impl Alloc for Allocator {
    unsafe fn alloc(&mut self, layout: Layout) -> Result<*mut u8, AllocErr> {
        let size = layout.size();

        // TODO implement the alignment side of things

        // Find the next fitting block
        let block_head = self.block_head;
        let block_ptr = self.next_fit(size, block_head)?;
        let mut block = block_ptr.as_mut().expect("Null Block Pointer");

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
            let mut next_block = block.next_ptr();
            (*next_block).size = next_block_size;
            (*next_block).prev = Some(block as *mut Block);
            (*next_block).next = block.next;
            (*next_block).free = true;

            // Finally set the allocated block to point to our new block and complete the chain.
            block.next = Some(next_block);
        }

        // Finally we mark the allocated block as used and return the data_pointer to the caller
        block.free = false;
        let alloc_pointer = block.data_pointer();
        Ok(alloc_pointer)
    }

    unsafe fn dealloc(&mut self, ptr: *mut u8, layout: Layout) {
        let mut block = get_block_ptr(ptr);

        // This is only for the unit tests...
        debug_assert!(block.size == layout.size());

        // Set the block as free
        block.free = true;

        // TODO Merge neighbours.
    }
}


// pub fn usable_size(&mut self, size: usize, _: usize) -> usize {
//     // TODO Implement this properly...
//     size
// }
//
// pub fn realloc(&mut self, ptr: *mut u8, size: usize, new_size: usize,
//     align: usize) -> Option<*mut u8> {
//     // TODO We should check here wether the new size is smaller, if so maybe just return the
//     // original pointer?
//
//     self.dealloc(ptr, size, align);
//
//     self.alloc(new_size, align)
// }
//
// pub fn realloc_inplace(&mut self, _: *mut u8, size: usize,
//     _: usize, _: usize) -> usize {
//     // TODO Implement this properly...
//     size
// }

// Get a pointer to the block that is encapsulating a given u8 pointer
unsafe fn get_block_ptr<'a>(ptr: *mut u8) -> &'a mut Block {
    let block_ptr = ptr.offset(-(size_of::<Block>() as isize)) as *mut Block;
    block_ptr.as_mut().expect("Null Block Pointer")
}
