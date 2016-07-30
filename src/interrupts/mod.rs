mod idt;
mod pic;

use x86;
use core::intrinsics;
use super::vga_buffer;

macro_rules! add_handler {
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
                    format_args!("EXCEPTION: Unhandled Interrupt (0x{:#})", $int)
                );
            }

            loop {}
        }

        add_handler!($idt, $int, handler);
    }}
}

lazy_static! {
    static ref IDT: idt::Idt = {
        let mut idt = idt::Idt::new();

        // Set all the handlers. Set default handler if a specific is not defined
        // to help debugging
        add_handler!(idt, 0, divide_by_zero_handler);
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
        default_handler!(idt, 14);
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
        default_handler!(idt, 32);
        //default_handler!(idt, 33);
        add_handler!(idt, 33, keyboard_handler);
        default_handler!(idt, 34);
        default_handler!(idt, 35);
        default_handler!(idt, 36);
        default_handler!(idt, 37);
        default_handler!(idt, 38);
        default_handler!(idt, 39);
        default_handler!(idt, 40);
        default_handler!(idt, 41);
        default_handler!(idt, 42);
        default_handler!(idt, 43);
        default_handler!(idt, 44);
        default_handler!(idt, 45);

        idt
    };
}

static PIC: pic::Pic = pic::Pic::new();

pub fn init() {
    PIC.init();

    // Enable some pic interrupts
    //PIC.clear_mask(0);
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

fn divide_by_zero_handler() {
    unsafe {
        vga_buffer::print_error(format_args!("EXCEPTION: Divide By Zero"));
    }

    loop {}
}

fn keyboard_handler() {
    unsafe {
        let code = x86::io::inb(0x60);
        vga_buffer::print_error(format_args!("Keypress: {}", code));
        PIC.send_end_of_interrupt(1);
    }
}