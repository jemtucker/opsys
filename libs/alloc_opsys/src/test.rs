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

// #[test]
// fn usable_size_0() {
//     // This will change in the future but for now just test that it returns the size
//     // we asked for
//     let mut heap = [0; HEAP_SIZE];
//     let mut allocator = Allocator::new(&mut heap, HEAP_SIZE);
//
//     let size = 100;
//     let align = 0;
//
//     let _ = allocator.alloc(size, 0);
//     assert!(allocator.usable_size(size, align) == size);
// }
//
// #[test]
// fn realloc_0() {
//     // Should be able to allocate one thing, then reallocate it an unlimited number of times
//     let mut heap = [0; HEAP_SIZE];
//     let mut allocator = Allocator::new(&mut heap, HEAP_SIZE);
//
//     let size = 100;
//     let mut ptr = allocator.alloc(size, 0).unwrap();
//
//     for _ in 1..500 {
//         let new_ptr = allocator.realloc(ptr, size, size, 0).unwrap();
//         ptr = new_ptr;
//     }
// }
//
// #[test]
// #[should_panic(expected = "Out Of Memory")]
// fn realloc_1() {
//     // This test should currently panic because the memory will become fragmented into lots of
//     // small blocks. When mereging is implemented this should be fixed.
//     let mut heap = [0; HEAP_SIZE];
//     let mut allocator = Allocator::new(&mut heap, HEAP_SIZE);
//
//     let size = 100;
//     let mut ptr = allocator.alloc(size, 0).unwrap();
//
//     for i in 1..500 {
//         let new_ptr = allocator.realloc(ptr, size + i - 1, size + i, 0).unwrap();
//         ptr = new_ptr;
//     }
// }
//
// #[test]
// fn realloc_inplace_0() {
//     // For now this do nothing...
//     let mut heap = [0; HEAP_SIZE];
//     let mut allocator = Allocator::new(&mut heap, HEAP_SIZE);
//
//     let size = 10;
//     let ptr = allocator.alloc(size, 0).unwrap();
//     assert!(allocator.realloc_inplace(ptr, size, 100, 0) == size);
// }
