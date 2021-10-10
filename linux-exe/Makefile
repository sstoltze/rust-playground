compile-assembler: hello.asm
	nasm -f elf64 hello.asm
	ld hello.o -o hello
	@# Not really necessary
	file hello

run-assembler: compile-assembler
	@./hello
