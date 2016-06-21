opsys.iso: kernel.bin
	cp kernel.bin isofiles/boot/kernel.bin
	/Users/jemtucker/opt/bin/grub-mkrescue -o opsys.iso isofiles

kernel.bin: boot.asm multiboot_header.asm
	/usr/local/bin/nasm -f elf64 multiboot_header.asm
	/usr/local/bin/nasm -f elf64 boot.asm
	~/opt/bin/x86_64-pc-elf-ld -n -o kernel.bin -T linker.ld multiboot_header.o boot.o

run: opsys.iso
	qemu-system-x86_64 opsys.iso

clean:
	rm *.o
	rm *.bin
	rm *.iso
	rm isofiles/boot/kernel.bin