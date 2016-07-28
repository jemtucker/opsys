use x86::io;

const PIC1: u8 = 0x20;
const PIC2: u8 = 0xa0;

const PIC_EOI: u8 = 0x20;

const ICW1_ICW4: u8 = 0x01;         // ICW4 (not) needed
const ICW1_SINGLE: u8 = 0x02;       // Single (cascade) mode
const ICW1_INTERVAL4: u8 = 0x04;    // Call address interval 4 (8)
const ICW1_LEVEL: u8 = 0x08;        // Level triggered (edge) mode
const ICW1_INIT: u8 = 0x10;         // Initialization - required!

const ICW4_8086: u8 = 0x01;         // 8086/88 (MCS-80/85) mode
const ICW4_AUTO: u8 = 0x02;         // Auto (normal) EOI
const ICW4_BUF_SLAVE: u8 = 0x08;    // Buffered mode/slave
const ICW4_BUF_MASTER: u8 = 0x0C;   // Buffered mode/master
const ICW4_SFNM: u8 = 0x10;         // Special fully nested (not)

const PIC_READ_IRR: u8 = 0x0a;
const PIC_READ_ISR: u8 = 0x0b;

struct Port {
    com: u16,
    data: u16
}

impl Port {
    pub const fn new(id: u8) -> Port {
        Port {
            com: id as u16,
            data: id as u16 + 1
        }
    }

    pub fn command(&self, byte: u8) {
        unsafe { io::outb(self.com, byte); }
    }

    pub fn result(&self) -> u8 {
        unsafe { io::inb(self.com) }
    }

    pub fn write(&self, byte: u8) {
        unsafe { io::outb(self.data, byte);}
    }

    pub fn read(&self) -> u8 {
        let b = unsafe { io::inb(self.data) };
        kprintln!("Read: 0x{:x}", b);
        b
    }
}

pub struct Pic {
    pic1: Port,
    pic2: Port
}

impl Pic {
    pub const fn new() -> Pic {
        Pic {
            pic1: Port::new(PIC1),
            pic2: Port::new(PIC2)
        }
    }

    pub fn init(&self) {
        // Save the initial masks
        let mask1 = self.pic1.read();
        let mask2 = self.pic2.read();

        // Start the initialization sequences
        self.pic1.command(ICW1_INIT + ICW1_ICW4);
        io_wait();
        self.pic2.command(ICW1_INIT + ICW1_ICW4);
        io_wait();
        self.pic1.write(0x20);  // ICW2: Master PIC vector offset
        io_wait();
        self.pic2.write(0x28);  // ICW2: Slave PIC vector offset
        io_wait();
        self.pic1.write(4);     // ICW3: tell Master PIC that there is a slave PIC at IRQ2 (0100)
        io_wait();
        self.pic2.write(2);     // ICW3: tell Slave PIC its cascade identity (0000 0010)
        io_wait();

        self.pic1.write(ICW4_8086);
        io_wait();
        self.pic2.write(ICW4_8086);
        io_wait();

        self.pic1.write(0xff);   // restore saved masks.
        self.pic2.write(0xff);
    }

    pub fn send_end_of_interrupt(&self, irq: u8) {
        assert!(irq < 16);

        if irq > 7 {
            self.pic2.command(PIC_EOI);
        }

        self.pic1.command(PIC_EOI);
    }

    pub fn set_mask(&self, irq: u8) {
        assert!(irq < 16);

        if irq < 8 {
            let mask = self.pic1.read() | (1 << irq);
            self.pic1.write(mask);
            kprintln!("PIC1 Mask: 0x{:x}", mask);
        } else {
            let mask = self.pic2.read() | (1 << (irq - 8));
            self.pic2.write(mask);
            kprintln!("PIC2 Mask: 0x{:x}", mask);
        }

    }

    pub fn clear_mask(&self, irq: u8) {
        assert!(irq < 16);

        if irq < 8 {
            let mask = self.pic1.read() & !(1 << irq);
            self.pic1.write(mask);
        } else {
            let mask = self.pic2.read() & !(1 << (irq - 8));
            self.pic2.write(mask);
        }
    }

    pub fn get_interrupt_request_reg(&self) -> u16 {
        self.get_register(PIC_READ_IRR)
    }

    pub fn get_in_service_reg(&self) -> u16 {
        self.get_register(PIC_READ_ISR)
    }

    fn get_register(&self, ocw: u8) -> u16 {
        // Write the command words to notify the PICs that we want to read their registers.
        // The next byte we read from the command we read will hold the result.
        self.pic1.command(ocw);
        self.pic2.command(ocw);

        let reg1 = self.pic1.result();
        let reg2 = self.pic2.result();

        reg1 as u16 | ((reg2 as u16) << 8)
    }
}

fn io_wait() {
    // Write some junk to port 0x80. This should take long enough for other io to complete
    unsafe { io::outb(0x80, 0); }
}

