use x86::io;

pub struct Port {
    id: u16
}

impl Port {
    pub const fn new(id: u16) -> Port {
        Port {
            id: id
        }
    }

    pub unsafe fn write(&self, byte: u8) {
        io::outb(self.id, byte);
    }

    pub unsafe fn read(&self) -> u8 {
        io::inb(self.id)
    }

    pub fn io_wait() {
        // Write some junk to port 0x80. This should take long enough for any other io to complete
        unsafe { io::outb(0x80, 0); }
    }
}
