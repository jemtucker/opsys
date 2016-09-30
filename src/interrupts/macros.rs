// Some helper macros for setting up handlers

macro_rules! add_exp_handler {
    ($idt:expr, $int:expr, $handler:ident) => {{
        #[naked]
        extern "C" fn isr() -> ! { unsafe {
            // TODO - Currently the error code (if present) is left on the stack. We need to pop
            //        this off before returning to normal execution if the exception can be handled.
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

macro_rules! add_irq_handler_1 {
    // Call a handler with a single argument.
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
                  mov rdi, rsp

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
                  iretq" :: "s"($handler as fn(_)) : "rdi" : "volatile", "intel");
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
