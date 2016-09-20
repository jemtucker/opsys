use super::allocator::Allocator;

static TEST_HEAP: [u8; 1000] = [0; 1000];

#[test]
fn allocation_works() {
    let allocator = Allocator::new(&TEST_HEAP, 1000);
    let _ = allocator.alloc(10, 0);
}

#[test]
fn deallocation_works() {
    // TODO
}

#[test]
fn multile_alloc_dealloc_works() {
    // TODO
}
