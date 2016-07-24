mod idt;

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

        idt
    };
}

pub fn init() {
    IDT.load();
}

// Some handlers...

extern "C" fn divide_by_zero_handler() -> ! {
    unsafe {
        vga_buffer::print_error(format_args!("EXCEPTION: Divide By Zero"));
    }

    loop {}
}