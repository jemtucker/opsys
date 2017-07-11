#![feature(allocator_api)]
#![feature(const_fn)]
#![feature(alloc)]

#![no_std]

extern crate spin;
extern crate alloc;

mod block;
mod allocator;
mod locked_allocator;

pub use locked_allocator::LockedAllocator;

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
