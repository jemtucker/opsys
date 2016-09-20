use ::block::Block;

fn header_size() -> isize {
    ::core::mem::size_of::<Block>() as isize
}


const MIN_HEAP_SIZE: usize = 1000;

pub struct Allocator {
    heap: &'static mut [u8],
    size: usize,
    block_head: *mut Block,
    block_tail: *mut Block
}

impl Allocator {

    // Create a new Allocator for a given heap. The heap must be at-least 1000 bytes.
    pub fn new(heap: &'static mut [u8], size: usize) -> Allocator {
        assert!(size > MIN_HEAP_SIZE);

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
    pub fn alloc(&mut self, size: usize, align: usize) -> Option<*mut u8> {
        // TODO calculate size from alignment
        let mut next_block = unsafe {
            let offset = header_size() + (*self.block_tail).size as isize;
            let mut b = self.block_tail.offset(offset as isize);
            (*b).prev = Some(self.block_tail);
            (*b).next = None;
            (*b).size = 0;
            (*b).free = true;
            b
        };

        unsafe {
            (*self.block_tail).next = Some(next_block);
            (*self.block_tail).size = size;
            (*self.block_tail).free = false;
        }

        unsafe {
            Some(self.block_tail.offset(header_size()) as *mut u8)
        }
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
}
