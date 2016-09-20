use ::block::Block;

pub struct Allocator {
    heap: &'static [u8],
    size: usize,
    blocks: Option<*mut Block>,
}

impl Allocator {
    pub fn new(heap: &'static [u8], size: usize) -> Allocator {
        Allocator {
            heap: heap,
            size: size,
            blocks: None
        }
    }

    pub fn alloc(size: usize, align: usize) -> Option<*mut u8> {
        unimplemented!();
    }

    pub fn dealloc(ptr: *mut u8, size: usize, align: usize) {
        unimplemented!();
    }

    pub fn usable_size(size: usize, align: usize) -> usize {
        unimplemented!();
    }

    pub fn realloc(ptr: *mut u8, size: usize, new_size: usize, align: usize) -> *mut u8 {
        unimplemented!();
    }

    pub fn realloc_inplace(ptr: *mut u8, size: usize, new_size: usize, align: usize) -> usize {
        unimplemented!();
    }
}
