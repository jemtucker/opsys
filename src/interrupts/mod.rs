mod idt;
mod pic;

use super::vga_buffer;

// Default interrupt handler to simply log the interrupt id.
macro_rules! default_handler {
    ($idt:expr, $int:expr) => {{
        extern "C" fn handler() -> ! {
            unsafe {
                vga_buffer::print_error(
                    format_args!("EXCEPTION: Unhandled Interrupt ({:#})", $int)
                );
            }

            loop {}
        }

        $idt.set_handler($int, handler);
    }}
}

lazy_static! {
    static ref IDT: idt::Idt = {
        let mut idt = idt::Idt::new();

        // Set all the handlers
        idt.set_handler(0, divide_by_zero_handler);

        // Set default handlers for others
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
        default_handler!(idt, 33);
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
    PIC.clear_mask(0);
    PIC.clear_mask(1);
    PIC.clear_mask(2);
    PIC.clear_mask(3);
    PIC.clear_mask(4);
    PIC.clear_mask(5);

    IDT.load();

    //unsafe { asm!("int $33"); }
}

// Some handlers...

extern "C" fn divide_by_zero_handler() -> ! {
    unsafe {
        vga_buffer::print_error(format_args!("EXCEPTION: Divide By Zero"));
    }

    loop {}
}