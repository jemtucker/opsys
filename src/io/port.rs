use x86_64::instructions::port;

pub struct Port {
    id: u16,
}

impl Port {
    pub const fn new(id: u16) -> Port {
        Port { id: id }
    }

    pub unsafe fn write(&self, byte: u8) {
        port::outb(self.id, byte);
    }

    pub unsafe fn read(&self) -> u8 {
        port::inb(self.id)
    }

    pub fn io_wait() {
        // Write some junk to port 0x80. This should take long enough for any other io to complete
        unsafe {
            port::outb(0x80, 0);
        }
    }
}
