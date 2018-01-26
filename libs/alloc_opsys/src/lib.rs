#![feature(allocator_api)]
#![feature(const_fn)]
#![feature(alloc)]
#![feature(unique)]
#![feature(ptr_internals)]
#![no_std]

extern crate spin;
extern crate alloc;

mod block;
mod allocator;
mod locked_allocator;

pub use allocator::Allocator;
pub use locked_allocator::LockedAllocator;

#[cfg(test)]
mod test;
