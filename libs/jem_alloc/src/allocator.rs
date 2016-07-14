use spin::Mutex;

pub struct Allocator {
    heap_start: usize,
    heap_size: usize
}

impl Allocator {
    pub const fn new(heap_start: usize, heap_size: usize) -> Allocator {
        Allocator {
            heap_start: heap_start,
            heap_size: heap_size
        }
    }

    pub fn alloc(&mut self, size: usize, align: usize) -> Option<*mut u8> {
        // TODO
        unimplemented!();
    }

    pub fn dealloc(&mut self, ptr: *mut u8, size: usize, align: usize) {
        // TODO
        unimplemented!();
    }
}