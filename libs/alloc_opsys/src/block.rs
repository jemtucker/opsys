use ::core::mem::size_of;

pub struct Block {
    pub prev: Option<*mut Block>,
    pub next: Option<*mut Block>,
    pub size: usize, // Can probably make this smaller?
    pub free: bool,
}

impl Block {

    pub fn free(&mut self) {
        // TODO
    }

    // Pointer to the next block. This could be invalid and is just calculated as the
    // pointer after the size of this block.
    pub unsafe fn next_ptr(&mut self) -> *mut Block {
        let offset = (size_of::<Block>() + self.size) as isize;
        (self as *mut Block).offset(offset)
    }

    pub unsafe fn merge_next(&mut self) {
        match self.next {
            Some(next) => {
                self.size = (*next).size;
                self.next = (*next).next;
            }
            None => {}
        };
    }

    pub unsafe fn data_pointer(&mut self) -> *mut u8 {
        let offset = size_of::<Block>() as isize;
        let mut self_ptr = self as *mut Block;
        self_ptr.offset(offset) as *mut u8
    }
}
