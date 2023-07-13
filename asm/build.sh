#!/bin/sh

nasm -f elf ./test.asm
mkdir ./build
gcc -m32 -o ./build/test test.o
./build/test
