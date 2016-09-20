#![feature(allocator)]
#![feature(const_fn)]

#![allocator]
#![no_std]

mod block;
mod allocator;

use allocator::Allocator;

pub const HEAP_START: usize = 0o_000_001_000_000_0000;
pub const HEAP_SIZE: usize = 100 * 1024; // 100 Kb

#[cfg(not(test))]
#[no_mangle]
pub extern fn __rust_allocate(size: usize, align: usize) -> *mut u8 {
    unimplemented!();
}

#[cfg(not(test))]
#[no_mangle]
pub extern fn __rust_usable_size(size: usize, align: usize) -> usize {
    unimplemented!();
}

#[cfg(not(test))]
#[no_mangle]
pub extern fn __rust_deallocate(ptr: *mut u8, size: usize, align: usize) {
    unimplemented!();
}

#[cfg(not(test))]
#[no_mangle]
pub extern fn __rust_reallocate(ptr: *mut u8, size: usize, new_size: usize,
                                align: usize) -> *mut u8 {
    unimplemented!();
}

#[cfg(not(test))]
#[no_mangle]
pub extern fn __rust_reallocate_inplace(ptr: *mut u8, size: usize,
                                        new_size: usize, align: usize)
                                        -> usize {
    unimplemented!();
}

#[cfg(test)]
mod test;
