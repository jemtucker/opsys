mod pic;

#[macro_use]
mod macros;

use x86;
use drivers;
use vga_buffer;

use kernel::kget;
use schedule::task::TaskContext;

use x86_64::structures::idt::{ExceptionStackFrame, Idt, PageFaultErrorCode};

lazy_static! {
    static ref IDT: Idt = {
        let mut idt = Idt::new();

        // Set all the handlers. Set default handler if a specific is not defined
        // to help debugging
        idt.divide_by_zero.set_handler_fn(except_00);
        idt.page_fault.set_handler_fn(except_14);

        // Interrupts
        irq_handler!(idt, 0, irq0);
        irq_handler!(idt, 1, irq1);

        idt
    };
}

static PIC: pic::Pic = pic::Pic::new();

/// Initialise kernel interrupt handling
///
/// Initialises the PIC and IDT. Enables CPU interrupts.
pub fn init() {
    PIC.init();

    // Enable some pic interrupts
    PIC.clear_mask(0);
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

// Exception Handlers

/// Divide by zero handler
///
/// Prints out details of the exception then sleeps the CPU forever.
extern "x86-interrupt" fn except_00(_: &mut ExceptionStackFrame) {
    unsafe {
        vga_buffer::print_error(format_args!("EXCEPTION: Divide By Zero\n"));
    };

    hang!();
}

/// Page fault handler
///
/// Prints out details of the exception then sleeps the CPU forever.
extern "x86-interrupt" fn except_14(
    stack_frame: &mut ExceptionStackFrame,
    error_code: PageFaultErrorCode,
) {
    unsafe {
        vga_buffer::print_error(format_args!(
            "EXCEPTION: Page Fault accessing {:#x} \nerror code: {:?}\n{:#?}",
            x86::controlregs::cr2(),
            error_code,
            stack_frame.instruction_pointer
        ));
    };

    hang!();
}

// IRQ Handlers...

/// Handler for IRQ0 - The PIT interrupt
///
/// Ticks the system clock once.
unsafe fn irq0() {
    let clock = &mut *kget().clock.get();
    clock.tick();
}

/// Handler for IRQ1 - The keyboard interrupt
///
/// Instantiates and queues up a new keyboard driver bottom half.
unsafe fn irq1() {
    let scheduler = &*kget().scheduler.get();

    let bh_manager = scheduler.bh_manager();
    let keyboard = box drivers::Keyboard::new();

    bh_manager.add_bh(keyboard);
}
