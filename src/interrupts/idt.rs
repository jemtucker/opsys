use x86::segmentation::{self, SegmentSelector};

// TODO - use from x86_64 package
pub struct ExceptionStackFrame {
    pub instruction_pointer: u64,
    pub code_segment: u64,
    pub cpu_flags: u64,
    pub stack_pointer: u64,
    pub stack_segment: u64,
}

pub struct Idt {
    table: [Entry; 255],
}

impl Idt {
    pub fn new() -> Idt {
        Idt { table: [Entry::empty(); 255] }
    }

    pub fn set_handler(&mut self, entry: u8, handler: HandlerFunc) -> &mut EntryOptions {
        self.table[entry as usize] = Entry::new(segmentation::cs(), handler);
        &mut self.table[entry as usize].options
    }

    pub fn set_handler_with_error(&mut self, entry: u8, handler: HandlerFuncWithError) -> &mut EntryOptions {
        self.table[entry as usize] = Entry::new_with_error(segmentation::cs(), handler);
        &mut self.table[entry as usize].options
    }

    pub fn load(&'static self) {
        use x86::dtables::{DescriptorTablePointer, lidt};
        use core::mem::size_of;

        let ptr = DescriptorTablePointer {
            base: self as *const _ as u64,
            limit: (size_of::<Self>() - 1) as u16,
        };

        unsafe { lidt(&ptr) };
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Entry {
    pointer_low: u16,
    gdt_selector: SegmentSelector,
    options: EntryOptions,
    pointer_middle: u16,
    pointer_high: u32,
    reserved: u32,
}

impl Entry {
    fn new(gdt_selector: SegmentSelector, handler: HandlerFunc) -> Self {
        let pointer = handler as u64;
        Entry {
            gdt_selector: gdt_selector,
            pointer_low: pointer as u16,
            pointer_middle: (pointer >> 16) as u16,
            pointer_high: (pointer >> 32) as u32,
            options: EntryOptions::new(),
            reserved: 0,
        }
    }

    fn new_with_error(gdt_selector: SegmentSelector, handler: HandlerFuncWithError) -> Self {
        let pointer = handler as u64;
        Entry {
            gdt_selector: gdt_selector,
            pointer_low: pointer as u16,
            pointer_middle: (pointer >> 16) as u16,
            pointer_high: (pointer >> 32) as u32,
            options: EntryOptions::new(),
            reserved: 0,
        }
    }

    fn empty() -> Self {
        Entry {
            gdt_selector: SegmentSelector::from_raw(0),
            pointer_low: 0,
            pointer_middle: 0,
            pointer_high: 0,
            options: EntryOptions::empty(),
            reserved: 0,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct EntryOptions(u16);

impl EntryOptions {
    fn empty() -> Self {
        // Start with everything zero except for the 'must be set' bits (9-11).
        EntryOptions(0x0e00)
    }

    fn new() -> Self {
        let mut opts = EntryOptions::empty();

        // Set the present and disable interrupts flags
        opts.set_present(true);
        opts.disable_interrupts(true);
        opts
    }

    pub fn set_present(&mut self, present: bool) {
        let bit = if present { 0x8000 } else { 0 };
        self.0 = (self.0 & 0x7fff) | bit;
    }

    pub fn disable_interrupts(&mut self, disable: bool) {
        let bit = if disable { 0 } else { 0x0100 };
        self.0 = (self.0 & 0xfeff) | bit;
    }

    #[allow(dead_code)]
    pub fn set_privilege_level(&mut self, dpl: u16) {
        self.0 = (self.0 & 0x9fff) | ((dpl << 13) & 0x6000);
    }

    #[allow(dead_code)]
    pub fn set_stack_index(&mut self, index: u16) {
        self.0 = (self.0 & 0xfff8) | (index & 0x07);
    }
}

pub type HandlerFunc = extern "x86-interrupt" fn(&ExceptionStackFrame);
pub type HandlerFuncWithError = extern "x86-interrupt" fn(&ExceptionStackFrame, u64);
