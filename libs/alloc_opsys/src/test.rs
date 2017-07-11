use super::allocator::Allocator;

use alloc::allocator::{Alloc, Layout};

// Bunch of different heaps as the tests run concurrently
const HEAP_SIZE: usize = 1000;

// Alloc tests

#[test]
fn alloc_0() {
    let mut heap = [0; HEAP_SIZE];

    unsafe {
        let mut allocator = Allocator::new(&mut heap, HEAP_SIZE);

        let layout = Layout::from_size_align_unchecked(26, 0);
        let result = allocator.alloc(layout);
        assert!(result.is_ok());
    }
}

#[test]
fn alloc_1() {
    let mut heap = [0; HEAP_SIZE];

    unsafe {
        let mut allocator = Allocator::new(&mut heap, HEAP_SIZE);

        // Allocate the entire heap
        for _ in 0..10 {
            let size = 100 - ::core::mem::size_of::<::block::Block>();
            let layout = Layout::from_size_align_unchecked(size, 0);
            let result = allocator.alloc(layout);
            assert!(result.is_ok());
        }
    }
}

#[test]
fn returns_err_on_oom_01() {
    let mut heap = [0; HEAP_SIZE];

    unsafe {
        let mut allocator = Allocator::new(&mut heap, HEAP_SIZE);

        let layout = Layout::from_size_align_unchecked(HEAP_SIZE + 1, 0);
        let result = allocator.alloc(layout);
        assert!(result.is_err());
    }
}

#[test]
fn returns_err_on_oom_02() {
    let mut heap = [0; HEAP_SIZE];

    unsafe {
        let mut allocator = Allocator::new(&mut heap, HEAP_SIZE);

        // Allocate the entire heap
        for _ in 0..10 {
            let size = 100 - ::core::mem::size_of::<::block::Block>();
            let layout = Layout::from_size_align_unchecked(size, 0);
            let result = allocator.alloc(layout);
            assert!(result.is_ok());
        }

        // And OOM it...
        let layout = Layout::from_size_align_unchecked(1, 0);
        let result = allocator.alloc(layout);
        assert!(result.is_err());
    }
}

// Dealloc tests

#[test]
fn dealloc_0() {
    let mut heap = [0; HEAP_SIZE];
    let size = 100 - ::core::mem::size_of::<::block::Block>();

    unsafe {
        let mut allocator = Allocator::new(&mut heap, HEAP_SIZE);

        // Allocate some memory
        let layout_a = Layout::from_size_align_unchecked(size, 0);
        let p1 = allocator.alloc(layout_a);

        // Deallocate it...
        let layout_d = Layout::from_size_align_unchecked(size, 0);
        allocator.dealloc(p1.unwrap(), layout_d);
    }
}

#[test]
fn dealloc_1() {
    let mut heap = [0; HEAP_SIZE];
    let size = 500 - ::core::mem::size_of::<::block::Block>();

    unsafe {
        let mut allocator = Allocator::new(&mut heap, HEAP_SIZE);

        // Allocate the entire heap in two allocs
        let layout_a1 = Layout::from_size_align_unchecked(size, 0);
        let p1 = allocator.alloc(layout_a1);
        assert!(p1.is_ok());

        let layout_a2 = Layout::from_size_align_unchecked(size, 0);
        let p2 = allocator.alloc(layout_a2);
        assert!(p2.is_ok());

        // Deallocate the first
        let layout_d = Layout::from_size_align_unchecked(size, 0);
        allocator.dealloc(p1.unwrap(), layout_d);

        // Try to reallocate the same memory freed from p1
        let layout_a3 = Layout::from_size_align_unchecked(size, 0);
        let p3 = allocator.alloc(layout_a3);
        assert!(p3.is_ok());
    }
}
