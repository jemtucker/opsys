#![feature(allocator)]
#![feature(const_fn)]

#![allocator]
#![no_std]

extern crate spin;

mod block;
mod allocator;

use spin::Mutex;
use allocator::Allocator;

pub const HEAP_START: usize = 0o_000_001_000_000_0000;
pub const HEAP_SIZE: usize = 100 * 1024; // 100 Kb

static ALLOCATOR: Mutex<Allocator> = Mutex::new(Allocator::new(HEAP_START, HEAP_SIZE));

#[no_mangle]
pub extern fn __rust_allocate(size: usize, align: usize) -> *mut u8 {
    ALLOCATOR.lock().alloc(size, align).expect("Out of memory")
}

#[no_mangle]
pub extern fn __rust_usable_size(size: usize, align: usize) -> usize {
    // TODO
    size
}

#[no_mangle]
pub extern fn __rust_deallocate(ptr: *mut u8, size: usize, align: usize) {
    ALLOCATOR.lock().dealloc(ptr, size, align);
}

#[no_mangle]
pub extern fn __rust_reallocate(ptr: *mut u8, size: usize, new_size: usize,
                                align: usize) -> *mut u8 {
    ALLOCATOR.lock().dealloc(ptr, size, align);
    ALLOCATOR.lock().alloc(new_size, align).expect("Out of memory")
}

#[no_mangle]
pub extern fn __rust_reallocate_inplace(ptr: *mut u8, size: usize,
                                        new_size: usize, align: usize)
                                        -> usize {
    // Unsupported so just return size
    size
}
