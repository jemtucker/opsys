use super::allocator::Allocator;

// Bunch of different heaps as the tests run concurrently
static mut TEST_HEAP_0: [u8; 1000] = [0; 1000];
static mut TEST_HEAP_1: [u8; 1000] = [0; 1000];
static mut TEST_HEAP_2: [u8; 1000] = [0; 1000];
static mut TEST_HEAP_3: [u8; 1000] = [0; 1000];
static mut TEST_HEAP_4: [u8; 1000] = [0; 1000];
static mut TEST_HEAP_5: [u8; 1000] = [0; 1000];

// Alloc tests

#[test]
fn alloc_0() {
    let mut allocator = unsafe { Allocator::new(&mut TEST_HEAP_0, 1000) };
    let _ = allocator.alloc(26, 0);
}

#[test]
fn alloc_1() {
    let mut allocator = unsafe { Allocator::new(&mut TEST_HEAP_1, 1000) };

    // Allocate the entire heap
    for _ in 0..10 {
        let _ = allocator.alloc(100 - ::core::mem::size_of::<::block::Block>(), 0);
    }
}

#[test]
#[should_panic(expected = "Out Of Memory")]
fn panics_on_oom_0() {
    let mut allocator = unsafe { Allocator::new(&mut TEST_HEAP_2, 1000) };
    let _ = allocator.alloc(1001, 0);
}

#[test]
#[should_panic(expected = "Out Of Memory")]
fn panics_on_oom_1() {
    let mut allocator = unsafe { Allocator::new(&mut TEST_HEAP_3, 1000) };

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
    let mut allocator = unsafe { Allocator::new(&mut TEST_HEAP_4, 1000) };
    let size = 100 - ::core::mem::size_of::<::block::Block>();

    // Allocate some memory
    let p1 = allocator.alloc(size, 0);

    // Deallocate it...
    allocator.dealloc(p1.unwrap(), size, 0);
}

#[test]
fn dealloc_1() {
    let mut allocator = unsafe { Allocator::new(&mut TEST_HEAP_5, 1000) };
    let size = 500 - ::core::mem::size_of::<::block::Block>();

    // Allocate the entire heap in two allocs
    let p1 = allocator.alloc(size, 0);
    let _ = allocator.alloc(size, 0);

    // Deallocate the first
    allocator.dealloc(p1.unwrap(), size, 0);

    // Try to reallocate the same memory freed from p1
    let _ = allocator.alloc(size, 0);
}
