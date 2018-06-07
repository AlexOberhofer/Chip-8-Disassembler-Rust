CC=rustc
TARGET=bin/chip-8-disassembler

makechip8disassembler:
	$(CC) src/chip-8-disassembler.rs

clean:
	rm chip-8-disassembler
