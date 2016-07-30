use io;

const PIC1: u16 = 0x20;
const PIC2: u16 = 0xa0;

const PIC_EOI: u8 = 0x20;

const ICW1_ICW4: u8 = 0x01;         // ICW4 (not) needed
const ICW1_INIT: u8 = 0x10;         // Initialization - required!
const ICW4_8086: u8 = 0x01;         // 8086/88 (MCS-80/85) mode

const PIC_READ_IRR: u8 = 0x0a;
const PIC_READ_ISR: u8 = 0x0b;

struct PicPort {
    com: io::Port,
    data: io::Port
}

impl PicPort {
    pub const fn new(id: u16) -> PicPort {
        PicPort {
            com: io::Port::new(id),
            data: io::Port::new(id + 1)
        }
    }

    pub fn command(&self, byte: u8) {
        unsafe { self.com.write(byte); }
    }

    pub fn result(&self) -> u8 {
        unsafe { self.com.read() }
    }

    pub fn write(&self, byte: u8) {
        unsafe { self.data.write(byte); }
    }

    pub fn read(&self) -> u8 {
        unsafe { self.data.read() }
    }
}

pub struct Pic {
    pic1: PicPort,
    pic2: PicPort
}

impl Pic {
    pub const fn new() -> Pic {
        Pic {
            pic1: PicPort::new(PIC1),
            pic2: PicPort::new(PIC2)
        }
    }

    pub fn init(&self) {
        // Save the initial masks
        let mask1 = self.pic1.read();
        let mask2 = self.pic2.read();

        // Start the initialization sequences
        self.pic1.command(ICW1_INIT + ICW1_ICW4);
        io::Port::io_wait();
        self.pic2.command(ICW1_INIT + ICW1_ICW4);
        io::Port::io_wait();
        self.pic1.write(0x20);  // ICW2: Master PIC vector offset
        io::Port::io_wait();
        self.pic2.write(0x28);  // ICW2: Slave PIC vector offset
        io::Port::io_wait();
        self.pic1.write(4);     // ICW3: tell Master PIC that there is a slave PIC at IRQ2 (0100)
        io::Port::io_wait();
        self.pic2.write(2);     // ICW3: tell Slave PIC its cascade identity (0000 0010)
        io::Port::io_wait();

        self.pic1.write(ICW4_8086);
        io::Port::io_wait();
        self.pic2.write(ICW4_8086);
        io::Port::io_wait();

        // Start with all interrupts off
        self.pic1.write(0xff);
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

