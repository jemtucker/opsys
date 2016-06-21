NASM_LOC = /usr/local/bin/nasm

assemble: boot.asm multiboot_header.asm long_mode.asm
	$(NASM_LOC) -f elf64 multiboot_header.asm
	$(NASM_LOC) -f elf64 boot.asm
	$(NASM_LOC) -f elf64 long_mode.asm

clean:
	rm *.o