#!/bin/sh

set -xe

nasm -felf64 -o output.o ./output.asm
ld -o output ./output.o
./output
echo $?
