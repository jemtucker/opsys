NASM_LOC = /usr/local/bin/nasm
LD_LOC = $(HOME)/opt/bin/x86_64-pc-elf-ld

kernel.bin: boot.asm multiboot_header.asm
	$(NASM_LOC) -f elf64 multiboot_header.asm
	$(NASM_LOC) -f elf64 boot.asm
	$(LD_LOC) -n -o kernel.bin -T linker.ld multiboot_header.o boot.o

clean:
	rm *.o
	rm *.bin