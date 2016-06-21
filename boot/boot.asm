global start

section .text
bits 32 ; Currently the CPU is in protected mode so only 32 bits available
start:
    ; print `OK` to screen
    mov dword [0xb8000], 0x2f4b2f4f
    hlt