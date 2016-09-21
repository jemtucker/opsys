use super::allocator::Allocator;

static mut TEST_HEAP: [u8; 1000] = [0; 1000];

#[test]
fn allocation_works() {
    unsafe {
        let mut allocator = Allocator::new(&mut TEST_HEAP, 1000);
        let ptr1 = allocator.alloc(10, 0);
    }
}

#[test]
#[should_panic(expected = "Out Of Memory")]
fn panics_on_oom() {
    unsafe {
        let mut allocator = Allocator::new(&mut TEST_HEAP, 1000);
        let ptr1 = allocator.alloc(1001, 0);
    }
}

#[test]
fn deallocation_works() {
    // TODO
}

#[test]
fn multile_alloc_dealloc_works() {
    // TODO
}
