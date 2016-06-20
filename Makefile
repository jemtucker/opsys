boot.bin: boot.asm
	nasm -f bin boot.asm -o boot.bin

run: boot.bin
	qemu-system-x86_64 boot.bin

clean:
	rm *.bin