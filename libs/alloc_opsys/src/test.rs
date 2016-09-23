use super::allocator::Allocator;

static mut TEST_HEAP: [u8; 1000] = [0; 1000];

// Alloc tests

#[test]
fn alloc_0() {
    let mut allocator = unsafe { Allocator::new(&mut TEST_HEAP, 1000) };
    let _ = allocator.alloc(26, 0);
}

#[test]
fn alloc_1() {
    let mut allocator = unsafe { Allocator::new(&mut TEST_HEAP, 1000) };

    // Allocate the entire heap
    for _ in 0..10 {
        let _ = allocator.alloc(100 - ::core::mem::size_of::<::block::Block>(), 0);
    }
}

#[test]
#[should_panic(expected = "Out Of Memory")]
fn panics_on_oom_0() {
    let mut allocator = unsafe { Allocator::new(&mut TEST_HEAP, 1000) };
    let _ = allocator.alloc(1001, 0);
}

#[test]
#[should_panic(expected = "Out Of Memory")]
fn panics_on_oom_1() {
    let mut allocator = unsafe { Allocator::new(&mut TEST_HEAP, 1000) };

    // Allocate the entire heap
    for _ in 0..10 {
        let _ = allocator.alloc(100 - ::core::mem::size_of::<::block::Block>(), 0);
    }

    // And OOM it...
    let _ = allocator.alloc(1, 0);
}

// Dealloc tests

#[test]
fn deallocation_works() {
    // TODO
}
