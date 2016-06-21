GRUB_MKRESCUE_LOC = $(HOME)/opt/bin/grub-mkrescue

.PHONY: boot

opsys.iso: boot
	cp boot/kernel.bin isofiles/boot/kernel.bin
	$(GRUB_MKRESCUE_LOC) -o opsys.iso isofiles

boot:
	$(MAKE) -C boot

run: opsys.iso
	qemu-system-x86_64 opsys.iso

clean:
	rm *.iso
	rm isofiles/boot/kernel.bin
	$(MAKE) -C boot clean