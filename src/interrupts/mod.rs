mod idt;
mod pic;
mod exception;

use x86;
use core::intrinsics;
use drivers;
use vga_buffer;

use kernel::kget;

use self::exception::Exception;
use self::exception::ExceptionWithError;

macro_rules! add_exp_handler {
    ($idt:expr, $int:expr, $handler:ident) => {{
        #[naked]
        extern "C" fn isr() -> ! { unsafe {
            asm!("mov rdi, rsp
                  call $0
                  iretq" :: "s"($handler as fn(_)) : "rdi" : "volatile", "intel");
            intrinsics::unreachable();
        }}

        $idt.set_handler($int, isr);
    }};
}

macro_rules! add_irq_handler {
    ($idt:expr, $int:expr, $handler:ident) => {{
        #[naked]
        extern "C" fn isr() -> ! { unsafe {
            asm!("push rbp
                  push r15
                  push r14
                  push r13
                  push r12
                  push r11
                  push r10
                  push r9
                  push r8
                  push rsi
                  push rdi
                  push rdx
                  push rcx
                  push rbx
                  push rax
                  mov rsi, rsp
                  push rsi

                  call $0
                  add rsp, 8
                  pop rax
                  pop rbx
                  pop rcx
                  pop rdx
                  pop rdi
                  pop rsi
                  pop r8
                  pop r9
                  pop r10
                  pop r11
                  pop r12
                  pop r13
                  pop r14
                  pop r15
                  pop rbp
                  iretq" :: "s"($handler as fn()) :: "volatile", "intel");
            intrinsics::unreachable();
        }}

        $idt.set_handler($int, isr);
    }};
}

// Default interrupt handler to simply log the interrupt id.
macro_rules! default_handler {
    ($idt:expr, $int:expr) => {{
        fn handler() {
            unsafe {
                vga_buffer::print_error(
                    format_args!("EXCEPTION: Unhandled Interrupt ({:x})", $int)
                );
            }

            loop {}
        }

        add_irq_handler!($idt, $int, handler);
    }}
}

lazy_static! {
    static ref IDT: idt::Idt = {
        let mut idt = idt::Idt::new();

        // Set all the handlers. Set default handler if a specific is not defined
        // to help debugging
        add_exp_handler!(idt, 0, exept_00);
        default_handler!(idt, 1);
        default_handler!(idt, 2);
        default_handler!(idt, 3);
        default_handler!(idt, 4);
        default_handler!(idt, 5);
        default_handler!(idt, 6);
        default_handler!(idt, 7);
        default_handler!(idt, 8);
        default_handler!(idt, 9);
        default_handler!(idt, 10);
        default_handler!(idt, 11);
        default_handler!(idt, 12);
        default_handler!(idt, 13);
        add_exp_handler!(idt, 14, exept_14);
        default_handler!(idt, 15);
        default_handler!(idt, 16);
        default_handler!(idt, 17);
        default_handler!(idt, 18);
        default_handler!(idt, 19);
        default_handler!(idt, 20);
        default_handler!(idt, 21);
        default_handler!(idt, 22);
        default_handler!(idt, 23);
        default_handler!(idt, 24);
        default_handler!(idt, 25);
        default_handler!(idt, 26);
        default_handler!(idt, 27);
        default_handler!(idt, 28);
        default_handler!(idt, 29);
        default_handler!(idt, 30);
        default_handler!(idt, 31);
        add_irq_handler!(idt, 32, irq0_handler); // IRQ 0
        add_irq_handler!(idt, 33, irq1_handler); // IRQ 1
        default_handler!(idt, 34); // IRQ 3
        default_handler!(idt, 35); // IRQ 4
        default_handler!(idt, 36); // IRQ 5
        default_handler!(idt, 37); // IRQ 6
        default_handler!(idt, 38); // IRQ 7
        default_handler!(idt, 39); // IRQ 8
        default_handler!(idt, 40); // IRQ 9

        idt
    };
}

static PIC: pic::Pic = pic::Pic::new();

pub fn init() {
    PIC.init();

    // Enable some pic interrupts
    PIC.clear_mask(0);
    PIC.clear_mask(1);
    //PIC.clear_mask(2);
    //PIC.clear_mask(3);
    //PIC.clear_mask(4);
    //PIC.clear_mask(5);

    IDT.load();

    // Enable interrupts
    unsafe { x86::irq::enable(); }
}

// Some handlers...



// Divide by zero
fn exept_00(exception: *const Exception) {
    unsafe {
        vga_buffer::print_error(format_args!("EXCEPTION: Divide By Zero\n{:#?}", *exception));
    };

    loop {}
}

// Page fault
fn exept_14(exception: *const ExceptionWithError) {

    unsafe {
        vga_buffer::print_error(format_args!("EXCEPTION: Page Fault\n{:#?}", *exception));

        let code = (*exception).error_code;
        let err = x86::irq::PageFaultError::from_bits(code);

        vga_buffer::print_error(format_args!("{:#?}", err));
    };

    loop {}
}

// IRQ Handlers...

// Handler for IRQ0 - the PIT interrupt
fn irq0_handler() {
    let ref mut scheduler = unsafe { &mut *kget().scheduler.get() };
    scheduler.tick();
    PIC.send_end_of_interrupt(0);
}

// Handler for IRQ1 - the keyboard interrupt
fn irq1_handler() {
    unsafe {
        drivers::KEYBOARD.handle_keypress();
        PIC.send_end_of_interrupt(1);
    }
}
