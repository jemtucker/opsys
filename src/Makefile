CC = $(HOME)/opt/bin/x86_64-pc-elf-gcc

.PHONY: clean

kernel.o:
	$(CC) -c kernel.c -o kernel.o -std=gnu99 -ffreestanding -O2 -Wall -Wextra

clean:
	rm *.o