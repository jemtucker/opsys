GRUB_MKRESCUE_LOC = $(HOME)/opt/bin/grub-mkrescue
LD_LOC = $(HOME)/opt/bin/x86_64-pc-elf-ld

BOOT_DIR = boot
KERNEL_DIR = src

.PHONY: boot
.PHONY: kernel
.PHONY: clean

opsys.iso: link
	$(GRUB_MKRESCUE_LOC) -o opsys.iso isofiles

link: boot kernel
	$(LD_LOC) -n -o isofiles/boot/kernel.bin -T linker.ld $(BOOT_DIR)/multiboot_header.o $(BOOT_DIR)/boot.o $(BOOT_DIR)/long_mode.o $(KERNEL_DIR)/kernel.o

boot:
	$(MAKE) -C $(BOOT_DIR)

kernel:
	$(MAKE) -C $(KERNEL_DIR)

run: opsys.iso
	qemu-system-x86_64 --cdrom opsys.iso

clean:
	rm *.iso
	rm isofiles/boot/kernel.bin
	$(MAKE) -C $(BOOT_DIR) clean
	$(MAKE) -C $(KERNEL_DIR) clean