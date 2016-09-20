

pub struct Block {
    pub prev: Option<*mut Block>,
    pub next: Option<*mut Block>,
    pub size: usize,
    pub free: bool,
}

impl Block {

    pub fn free(&mut self) {
        // TODO
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
        let header_size = ::core::mem::size_of::<Block>() as isize;
        let mut self_ptr = self as *mut Block;
        self_ptr.offset(header_size) as *mut u8
    }
}
