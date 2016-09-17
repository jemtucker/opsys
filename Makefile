LINKER = $(HOME)/opt/bin/x86_64-pc-elf-ld
ASSEMBLER = /usr/local/bin/nasm
GRUB_MKRESCUE = $(HOME)/opt/bin/grub-mkrescue

TARGET = x86_64-OpSys-target

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

target/kernel.bin: target/multiboot_header.o target/boot.o target/long_mode.o src/asm/linker.ld cargo
	$(LINKER) -n -o target/kernel.bin -T src/asm/linker.ld target/multiboot_header.o target/boot.o target/long_mode.o target/$(TARGET)/release/libOpSys.a

target/os.iso: target/kernel.bin src/asm/grub.cfg
	mkdir -p target/isofiles/boot/grub
	cp src/asm/grub.cfg target/isofiles/boot/grub
	cp target/kernel.bin target/isofiles/boot/
	$(GRUB_MKRESCUE) -o target/os.iso target/isofiles

libs/libcore/target/$(TARGET)/libcore.rlib: $(TARGET).json
	cp $(TARGET).json libs/libcore
	cd libs/libcore && cargo build --release --features disable_float --target=$(TARGET).json

libs/liballoc/target/$(TARGET)/liballoc.rlib: $(TARGET).json libs/libcore/target/$(TARGET)/libcore.rlib
	cp $(TARGET).json libs/liballoc
	cd libs/liballoc && RUSTFLAGS="-L ../libcore/target/$(TARGET)/release" cargo build --release --target=$(TARGET).json

libs/libcollections/target/$(TARGET)/libcollections.rlib: $(TARGET).json libs/libcore/target/$(TARGET)/libcore.rlib libs/liballoc/target/$(TARGET)/liballoc.rlib libs/librustc_unicode/target/$(TARGET)/librustc_unicode.rlib
	cp $(TARGET).json libs/libcollections
	cd libs/libcollections && RUSTFLAGS="-L ../libcore/target/$(TARGET)/release -L ../liballoc/target/$(TARGET)/release -L ../librustc_unicode/target/$(TARGET)/release" cargo build --release --target=$(TARGET).json

libs/librustc_unicode/target/$(TARGET)/librustc_unicode.rlib: $(TARGET).json libs/libcore/target/$(TARGET)/libcore.rlib
	cp $(TARGET).json libs/librustc_unicode
	cd libs/librustc_unicode && RUSTFLAGS="-L ../libcore/target/$(TARGET)/release" cargo build --release --target=$(TARGET).json

cargo: libs/libcore/target/$(TARGET)/libcore.rlib libs/librustc_unicode/target/$(TARGET)/librustc_unicode.rlib libs/liballoc/target/$(TARGET)/liballoc.rlib libs/libcollections/target/$(TARGET)/libcollections.rlib
	RUSTFLAGS="-L libs/libcore/target/$(TARGET)/release -L libs/librustc_unicode/target/$(TARGET)/release -L libs/liballoc/target/$(TARGET)/release -L libs/libcollections/target/$(TARGET)/release" cargo build --release --target=$(TARGET).json


run: target/os.iso
	qemu-system-x86_64 -cdrom target/os.iso

debug: target/os.iso
	qemu-system-x86_64 -cdrom target/os.iso -s

debugstop: target/os.iso
	qemu-system-x86_64 -cdrom target/os.iso -s -S

clean: 
	cargo clean

cleanall: clean
	cd libs/libcore && cargo clean
	rm libs/libcore/$(TARGET).json
	cd libs/liballoc && cargo clean
	rm libs/liballoc/$(TARGET).json
	cd libs/libcollections && cargo clean
	rm libs/libcollections/$(TARGET).json
	cd libs/librustc_unicode && cargo clean
	rm libs/librustc_unicode/$(TARGET).json

