use ::block::Block;

fn header_size() -> isize {
    ::core::mem::size_of::<Block>() as isize
}

pub struct Allocator {
    heap: &'static mut [u8],
    size: usize,
    block_head: *mut Block,
    block_tail: *mut Block
}

impl Allocator {

    // Create a new Allocator for a given heap. The heap must be at-least 1000 bytes.
    pub fn new(heap: &'static mut [u8], size: usize) -> Allocator {
        assert!(size > header_size() as usize);

        let mut block = unsafe {
            let mut b = (&mut heap[0] as *mut u8) as *mut Block;
            (*b).prev = None;
            (*b).next = None;
            (*b).size = 0;
            (*b).free = true;
            b
        };

        Allocator {
            heap: heap,
            size: size,
            block_head: block,
            block_tail: block
        }
    }

    // Allocate 'size' byes
    pub unsafe fn alloc(&mut self, size: usize, align: usize) -> Option<*mut u8> {

        // TODO calculate size from alignment

        let offset = header_size() + (*self.block_tail).size as isize;
        let mut next_block = self.block_tail.offset(offset as isize);
        (*next_block).prev = Some(self.block_tail);
        (*next_block).next = None;
        (*next_block).size = 0;
        (*next_block).free = true;

        (*self.block_tail).next = Some(next_block);
        (*self.block_tail).size = size;
        (*self.block_tail).free = false;

        Some(self.block_tail.offset(header_size()) as *mut u8)
    }

    pub unsafe fn dealloc(&mut self, ptr: *mut u8, size: usize, align: usize) {
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
}
