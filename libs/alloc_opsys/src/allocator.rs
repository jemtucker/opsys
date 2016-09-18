const MIN_BLOCK_SIZE: usize = 16;
const HEADER_SIZE: usize = 16;

pub struct Allocator {
    heap_start: usize,
    heap_size: usize,
    position: usize
}

impl Allocator {
    pub const fn new(heap_start: usize, heap_size: usize) -> Allocator {
        Allocator {
            heap_start: heap_start,
            heap_size: heap_size,
            position: heap_start
        }
    }

    pub fn alloc(&mut self, size: usize, align: usize) -> Option<*mut u8> {
        // Get the size to allocate (the block size).
        let bsize = get_req_size(size);

        // Find the next fitting block and allocate if we can. Otherwise OOM.
        // TODO - Maybe on failure try some kind of compression / de-fragmentation
        if self.next_fit(bsize) {
            self.allocate_position(bsize);
            Some(self.position as *mut u8)
        } else {
            None
        }
    }

    pub fn dealloc(&mut self, ptr: *mut u8, size: usize, align: usize) {
        // All we do for now is set the block header flag.
        // TODO - We need to handle merging
        //let header = ptr as *mut usize;

        unsafe {
            //assert!(block_size(*header) == size);
            //*header = dealloc_header(*header);
        }
    }

    fn next_fit(&mut self, size: usize) -> bool {
        // Starting at self.position find the next hole that fits. If we pass where we started then
        // we are out of memory and must fail.
        let start = self.position;

        loop {
            // If the next block is free and big enough return here. Because blocks must be at least
            // a multiple of the MIN_BLOCK_SIZE we can be sure the rest of the free space will be
            // at least this big.
            let current_header = unsafe { *(self.position as *const usize) };
            let bsize = block_size(current_header);

            if block_is_free(current_header) {
                // If the next block size is zero it has never been allocated yet. Check we have
                // enough heap left then allocate.
                if bsize == 0 && self.position + size < self.heap_size {
                    return true;
                }

                // Previously freed block that is big enough. Success.
                if bsize > size {
                    return true;
                }
            }

            // Onto the next block...
            let mut new_position = (bsize / MIN_BLOCK_SIZE) + self.position;

            // If we have reached the end, time to go back to the start.
            if new_position > self.heap_size {
                new_position = self.heap_start;
            }

            // Are we passing the initial start? If so then we have failed in our quest to allocate
            // memory. Return false.
            if self.position < start && new_position >= start {
                return false;
            }

            // Update the position and try again...
            self.position = new_position;
        }
    }

    fn allocate_position(&self, size: usize) {
        // Perform the allocation
        let header = alloc_header(size + HEADER_SIZE);
        let position = self.position as *mut usize;
        unsafe {
            *position = header;
        }

        // Push the position pointer past the header
        self.position + HEADER_SIZE;
    }

}

const ALLOCATED_MASK: usize = 0x8000_0000_0000_0000;

#[inline]
fn block_is_free(header: usize) -> bool {
    // Free if the first bit is not set
    (header & ALLOCATED_MASK) == 0
}

#[inline]
fn block_size(header: usize) -> usize {
    // Free if the first bit is not set
    (header ^ ALLOCATED_MASK) as usize
}

#[inline]
fn alloc_header(header: usize) -> usize {
    header | ALLOCATED_MASK
}

#[inline]
fn dealloc_header(header: usize) -> usize {
    // Exactly the same as block_size
    block_size(header)
}

fn get_req_size(size: usize) -> usize {
    // Block size must be at-least the minimum block size or the smallest multiple.
    if size < MIN_BLOCK_SIZE {
        MIN_BLOCK_SIZE
    } else {
        ((size / MIN_BLOCK_SIZE) + 1) * MIN_BLOCK_SIZE
    }
}
