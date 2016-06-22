global long_mode_start

; Finally into 64 bit mode :)
bits 64
long_mode_start:

    extern kernel_main
    call kernel_main

    cli
    hlt