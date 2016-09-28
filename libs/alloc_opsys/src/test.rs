use super::allocator::Allocator;

// Bunch of different heaps as the tests run concurrently
const HEAP_SIZE: usize = 1000;

// Alloc tests

#[test]
fn alloc_0() {
    let mut heap = [0; HEAP_SIZE];
    let mut allocator = Allocator::new(&mut heap, HEAP_SIZE);
    let _ = allocator.alloc(26, 0);
}

#[test]
fn alloc_1() {
    let mut heap = [0; HEAP_SIZE];
    let mut allocator = Allocator::new(&mut heap, HEAP_SIZE);

    // Allocate the entire heap
    for _ in 0..10 {
        let _ = allocator.alloc(100 - ::core::mem::size_of::<::block::Block>(), 0);
    }
}

#[test]
#[should_panic(expected = "Out Of Memory")]
fn panics_on_oom_0() {
    let mut heap = [0; HEAP_SIZE];
    let mut allocator = Allocator::new(&mut heap, HEAP_SIZE);
    let _ = allocator.alloc(1001, 0);
}

#[test]
#[should_panic(expected = "Out Of Memory")]
fn panics_on_oom_1() {
    let mut heap = [0; HEAP_SIZE];
    let mut allocator = Allocator::new(&mut heap, HEAP_SIZE);

    // Allocate the entire heap
    for _ in 0..10 {
        let _ = allocator.alloc(100 - ::core::mem::size_of::<::block::Block>(), 0);
    }

    // And OOM it...
    let _ = allocator.alloc(1, 0);
}

// Dealloc tests

#[test]
fn dealloc_0() {
    let mut heap = [0; HEAP_SIZE];
    let mut allocator = Allocator::new(&mut heap, HEAP_SIZE);

    let size = 100 - ::core::mem::size_of::<::block::Block>();

    // Allocate some memory
    let p1 = allocator.alloc(size, 0);

    // Deallocate it...
    allocator.dealloc(p1.unwrap(), size, 0);
}

#[test]
fn dealloc_1() {
    let mut heap = [0; HEAP_SIZE];
    let mut allocator = Allocator::new(&mut heap, HEAP_SIZE);

    let size = 500 - ::core::mem::size_of::<::block::Block>();

    // Allocate the entire heap in two allocs
    let p1 = allocator.alloc(size, 0);
    let _ = allocator.alloc(size, 0);

    // Deallocate the first
    allocator.dealloc(p1.unwrap(), size, 0);

    // Try to reallocate the same memory freed from p1
    let _ = allocator.alloc(size, 0);
}

#[test]
fn usable_size_0() {
    // This will change in the future but for now just test that it returns the size
    // we asked for
    let mut heap = [0; HEAP_SIZE];
    let mut allocator = Allocator::new(&mut heap, HEAP_SIZE);

    let size = 100;
    let align = 0;

    let _ = allocator.alloc(size, 0);
    assert!(allocator.usable_size(size, align) == size);
}

#[test]
fn realloc_0() {
    // Should be able to allocate one thing, then reallocate it an unlimited number of times
    let mut heap = [0; HEAP_SIZE];
    let mut allocator = Allocator::new(&mut heap, HEAP_SIZE);

    let size = 100;
    let mut ptr = allocator.alloc(size, 0).unwrap();

    for _ in 1..500 {
        let new_ptr = allocator.realloc(ptr, size, size, 0).unwrap();
        ptr = new_ptr;
    }
}

#[test]
#[should_panic(expected = "Out Of Memory")]
fn realloc_1() {
    // This test should currently panic because the memory will become fragmented into lots of
    // small blocks. When mereging is implemented this should be fixed.
    let mut heap = [0; HEAP_SIZE];
    let mut allocator = Allocator::new(&mut heap, HEAP_SIZE);

    let size = 100;
    let mut ptr = allocator.alloc(size, 0).unwrap();

    for i in 1..500 {
        let new_ptr = allocator.realloc(ptr, size + i - 1, size + i, 0).unwrap();
        ptr = new_ptr;
    }
}

#[test]
fn realloc_inplace_0() {
    // For now this do nothing...
    let mut heap = [0; HEAP_SIZE];
    let mut allocator = Allocator::new(&mut heap, HEAP_SIZE);

    let size = 10;
    let ptr = allocator.alloc(size, 0).unwrap();
    assert!(allocator.realloc_inplace(ptr, size, 100, 0) == size);
}
