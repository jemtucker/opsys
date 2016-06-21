global long_mode_start

; Finally into 64 bit mode :)
bits 64
long_mode_start:

	mov rax, 0x2f592f412f4b2f4f
    mov qword [0xb8000], rax

    extern kernel_main
    call kernel_main

    cli
    hlt