mod idt;

use super::vga_buffer;

lazy_static! {
    static ref IDT: idt::Idt = {
        let mut idt = idt::Idt::new();

        idt.set_handler(0, divide_by_zero_handler);

        idt
    };
}

pub fn init() {
    IDT.load();
}

// Some handlers...

extern "C" fn divide_by_zero_handler() -> ! {
    unsafe {
        vga_buffer::print_error(format_args!("EXCEPTION: DIVIDE BY ZERO"));
    }

    loop {}
}