pub struct Block {
    prev: *u8,
    next: *u8,
    size: usize,
    free: bool,
}

impl Block {
    pub fn alloc_new(size: usize, prev: *u8, next: *u8) -> Block {
        // TODO
    }

    pub fn free(&mut self) {
        // TODO
    }

    pub fn merge_next(&mut self) {
        self.size = self.next.size;
        self.next = self.next.next;
    }
}
