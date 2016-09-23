use ::block::Block;

use ::core::mem::size_of;

//const MIN_ALLOCATION: usize = 1;
//const MIN_BLOCK_SIZE: usize = size_of::<Block>() + MIN_ALLOCATION;
const MIN_BLOCK_SIZE: usize = 32;

// WIP NOTES...
// 1. Start with one massive block, break this up with every allocation. Walk for first fit.
// 2. Deallocate just flag as free
// 3. Merge free neighbours? - Maybe a GC function that runs periodically??
//
//
//

pub struct Allocator {
    heap: &'static mut [u8],
    size: usize,
    block_head: *mut Block
}

impl Allocator {

    // Create a new Allocator for a given heap. The heap must be at-least the size of a block
    // header.
    pub fn new(heap: &'static mut [u8], size: usize) -> Allocator {
        assert!(size > size_of::<Block>());
        let mut block = unsafe {
            let mut b = (&mut heap[0] as *mut u8) as *mut Block;
            (*b).prev = None;
            (*b).next = None;
            (*b).size = size - size_of::<Block>();
            (*b).free = true;
            b
        };

        Allocator {
            heap: heap,
            size: size,
            block_head: block
        }
    }

    // Allocate 'size' byes
    pub fn alloc(&mut self, size: usize, align: usize) -> Option<*mut u8> {

        // TODO implement the alignment side of things

        // Find the next fitting block
        let mut block_head = self.block_head;
        let mut block_ptr = self.next_fit(size, block_head);
        let mut block = unsafe { block_ptr.as_mut().expect("Null Block Pointer") };

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
            let mut next_block = unsafe { block.next_ptr() };
            unsafe {
                (*next_block).size = next_block_size;
                (*next_block).prev = Some(block as *mut Block);
                (*next_block).next = block.next;
                (*next_block).free = true;
            }

            // Finally set the allocated block to point to our new block and complete the chain.
            block.next = Some(next_block);
        }

        // Finally we mark the allocated block as used and return the data_pointer to the caller
        block.free = false;
        let mut alloc_pointer = unsafe { block.data_pointer() };
        Some(alloc_pointer)
    }

    pub fn dealloc(&mut self, ptr: *mut u8, size: usize, align: usize) {
        unimplemented!();
    }

    pub fn usable_size(&mut self, size: usize, align: usize) -> usize {
        unimplemented!();
    }

    pub fn realloc(&mut self, ptr: *mut u8, size: usize,
        new_size: usize, align: usize) -> *mut u8 {
        unimplemented!();
    }

    pub fn realloc_inplace(&mut self, ptr: *mut u8, size: usize,
        new_size: usize, align: usize) -> usize {
        unimplemented!();
    }

    fn next_fit(&mut self, size: usize, current: *mut Block) -> *mut Block {
        let current_ref = unsafe { current.as_ref().expect("Null Block Pointer") };

        if current_ref.free && current_ref.size >= size {
            current
        } else {
            let mut block = match current_ref.next {
                Some(next) => next,
                None => panic!("Out Of Memory"),
            };

            self.next_fit(size, block)
        }
    }
}
