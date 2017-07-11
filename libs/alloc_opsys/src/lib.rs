#![feature(allocator_api)]
#![feature(const_fn)]
#![feature(alloc)]

#![no_std]

mod block;
mod allocator;

extern crate spin;
extern crate alloc;

#[cfg(not(test))]
pub const HEAP_START: usize = 0o_000_001_000_000_0000;

#[cfg(not(test))]
pub const HEAP_SIZE: usize = 100 * 1024; // 100 Kb

#[cfg(not(test))]
mod internal {

    use allocator::Allocator;
    use spin::Mutex;
    use core::slice::from_raw_parts_mut;

    pub static mut ALLOCATOR: Option<Mutex<Allocator>> = None;

    pub fn _init() {
        let heap_ptr = ::HEAP_START as *mut u8;
        let mut heap: &mut [u8] = unsafe { from_raw_parts_mut::<u8>(heap_ptr, ::HEAP_SIZE) };

        unsafe {
            ALLOCATOR = Some(Mutex::new(Allocator::new(heap, ::HEAP_SIZE)));
        }
    }

}

#[cfg(not(test))]
use internal::*;

#[cfg(not(test))]
pub fn init() {
    _init();
}

// #[cfg(not(test))]
// #[no_mangle]
// pub extern fn __rust_allocate(size: usize, align: usize) -> *mut u8 {
//     unsafe {
//         let alloc_ref = ALLOCATOR.as_ref();
//         alloc_ref.expect("Allocator Not Initialized").lock().alloc(size, align).unwrap()
//     }
// }
//
// #[cfg(not(test))]
// #[no_mangle]
// pub extern fn __rust_usable_size(size: usize, align: usize) -> usize {
//     unsafe {
//         let alloc_ref = ALLOCATOR.as_ref();
//         alloc_ref.expect("Allocator Not Initialized").lock().usable_size(size, align)
//     }
// }
//
// #[cfg(not(test))]
// #[no_mangle]
// pub extern fn __rust_deallocate(ptr: *mut u8, size: usize, align: usize) {
//     unsafe {
//         let alloc_ref = ALLOCATOR.as_ref();
//         alloc_ref.expect("Allocator Not Initialized").lock().dealloc(ptr, size, align);
//     }
// }
//
// #[cfg(not(test))]
// #[no_mangle]
// pub extern fn __rust_reallocate(ptr: *mut u8, size: usize, new_size: usize,
//     align: usize) -> *mut u8 {
//     unsafe {
//         let alloc_ref = ALLOCATOR.as_ref();
//         alloc_ref.expect("Allocator Not Initialized").lock()
//             .realloc(ptr, size, new_size, align).unwrap()
//     }
// }
//
// #[cfg(not(test))]
// #[no_mangle]
// pub extern fn __rust_reallocate_inplace(ptr: *mut u8, size: usize, new_size: usize,
//     align: usize) -> usize {
//     unsafe {
//         let alloc_ref = ALLOCATOR.as_ref();
//         alloc_ref.expect("Allocator Not Initialized").lock()
//             .realloc_inplace(ptr, size, new_size, align)
//     }
// }

#[cfg(test)]
mod test;
