macro_rules! irq_handler {
    ($idt:expr, $irq:expr, $irq_handler:ident) => {{
        extern "x86-interrupt" fn base_handler(_: &mut ExceptionStackFrame) {
            // Base handler. Push and pop context between call to handler implementation.
            unsafe {
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
                      mov rdi, rsp

                      call $0

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
                      pop rbp" :: "s"(base_handler_impl as unsafe fn(_)) :: "volatile", "intel");
            }
        }

        unsafe fn base_handler_impl(context: *mut TaskContext) {
            // Call the interrupt specific handler
            $irq_handler();

            // Perform any rescheduling thats required
            let scheduler = &mut *kget().scheduler.get();
            if scheduler.need_resched() {
                let context_ref = &mut *context;
                scheduler.schedule(context_ref);
            }

            PIC.send_end_of_interrupt($irq);
        }

        $idt.interrupts[$irq].set_handler_fn(base_handler);
    }}
}
