LINKER = $(HOME)/opt/bin/x86_64-pc-elf-ld
ASSEMBLER = /usr/local/bin/nasm
GRUB_MKRESCUE = $(HOME)/opt/bin/grub-mkrescue

TARGET = x86_64-opsys

default: build

build: target/kernel.bin

.PHONY: clean

target/multiboot_header.o: src/asm/multiboot_header.asm
	mkdir -p target
	$(ASSEMBLER) -f elf64 src/asm/multiboot_header.asm -o target/multiboot_header.o

target/boot.o: src/asm/boot.asm
	mkdir -p target
	$(ASSEMBLER) -f elf64 src/asm/boot.asm -o target/boot.o

target/long_mode.o: src/asm/long_mode.asm
	$(ASSEMBLER) -f elf64 src/asm/long_mode.asm -o target/long_mode.o

target/kernel.bin: target/multiboot_header.o target/boot.o target/long_mode.o src/asm/linker.ld xargo
	$(LINKER) -n --gc-sections -o target/kernel.bin -T src/asm/linker.ld target/multiboot_header.o target/boot.o target/long_mode.o target/$(TARGET)/release/libopsys.a

target/os.iso: target/kernel.bin src/asm/grub.cfg
	mkdir -p target/isofiles/boot/grub
	cp src/asm/grub.cfg target/isofiles/boot/grub
	cp target/kernel.bin target/isofiles/boot/
	$(GRUB_MKRESCUE) -o target/os.iso target/isofiles

xargo:
	xargo build --release --target=$(TARGET)

run: target/os.iso
	qemu-system-x86_64 -cdrom target/os.iso

debug: target/os.iso
	qemu-system-x86_64 -cdrom target/os.iso -s

debugstop: target/os.iso
	qemu-system-x86_64 -cdrom target/os.iso -s -S

clean:
	xargo clean

lldb:
	rust-lldb "target/kernel.bin" -s "scripts/lldb_launch"
