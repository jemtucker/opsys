mod pic;

use x86;
use drivers;
use vga_buffer;

use kernel::kget;
use x86_64::VirtualAddress;
use x86_64::structures::idt::{Idt, ExceptionStackFrame, PageFaultErrorCode};

use schedule::task::TaskContext;

lazy_static! {
    static ref IDT: Idt = {
        let mut idt = Idt::new();

        // Set all the handlers. Set default handler if a specific is not defined
        // to help debugging
        idt.divide_by_zero.set_handler_fn(except_00);
        idt.page_fault.set_handler_fn(except_14);

        // Interrupts
        idt.interrupts[0].set_handler_fn(irq0_handler); // IRQ 1
        idt.interrupts[1].set_handler_fn(irq1_handler); // IRQ 2

        idt
    };
}

static PIC: pic::Pic = pic::Pic::new();

pub fn init() {
    PIC.init();

    // Enable some pic interrupts
    //PIC.clear_mask(0);
    PIC.clear_mask(1);
    // PIC.clear_mask(2);
    // PIC.clear_mask(3);
    // PIC.clear_mask(4);
    // PIC.clear_mask(5);

    IDT.load();

    // Enable interrupts
    unsafe {
        x86::irq::enable();
    }
}

// Some handlers...

// Divide by zero
extern "x86-interrupt" fn except_00(_: &mut ExceptionStackFrame) {
    unsafe {
        vga_buffer::print_error(format_args!("EXCEPTION: Divide By Zero\n"));
    };

    loop {}
}

// Page fault
extern "x86-interrupt" fn except_14(stack_frame: &mut ExceptionStackFrame,
                                    error_code: PageFaultErrorCode) {
    unsafe {
        vga_buffer::print_error(format_args!("EXCEPTION: Page Fault accessing {:#x} \nerror \
                                              code: {:?}\n{:#?}",
                                             x86::controlregs::cr2(),
                                             error_code,
                                             stack_frame.instruction_pointer));

    };

    loop {}
}

// IRQ Handlers...

// Handler for IRQ0 - the PIT interrupt
extern "x86-interrupt" fn irq0_handler(stack_frame: &mut ExceptionStackFrame) {
    let VirtualAddress(address) = stack_frame.stack_pointer;
    let context = address as *mut TaskContext;
    let context_ref = unsafe { &mut *context };
    let scheduler = unsafe { &mut *kget().scheduler.get() };

    scheduler.tick(context_ref);

    PIC.send_end_of_interrupt(0);
}

// Handler for IRQ1 - the keyboard interrupt
extern "x86-interrupt" fn irq1_handler(_: &mut ExceptionStackFrame) {
    unsafe {
        drivers::KEYBOARD.handle_keypress();
        PIC.send_end_of_interrupt(1);
    }
}
