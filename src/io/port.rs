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

    pub fn write(&self, byte: u8) {
        unsafe { io::outb(self.id, byte);}
    }

    pub fn read(&self) -> u8 {
        unsafe { io::inb(self.id) }
    }
}
