#!/usr/bin/bash
cargo run 
nasm -f elf64 test.asm
gcc -no-pie -o test test.o
