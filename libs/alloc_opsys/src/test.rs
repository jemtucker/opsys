use super::allocator::Allocator;

static TEST_HEAP: [u8; 1000] = [0; 1000];

#[test]
fn it_works() {
    let allocator = Allocator::new(&TEST_HEAP, 1000);


}
