;; This File is Automatically Created Useing Nemet Parser
;; Under MIT License Copyright MahanFarzaneh 2023-2024

section .data
msg db "hello world", 0xa
len equ $ - msg

section .text
global _start
print:
    ;; <IF FUNC HAS ARGS> Maintain rpb
    push    rbp
    ;; <IF FUNC HAS ARGS> move stack pointer to rbp
    mov     rbp, rsp
    ;; allocate 40 bytes 8 for rdi(input of function) and 32 chars
    ;; mov rdi to the first alocated memory
    mov     QWORD [rbp-40], rdi
    ;; move from memory to rax
    mov     rax, QWORD [rbp-40]
    ;; rsi pointer to first memory
    mov     rsi, msg
    ;; rdx size
    mov     rdx, rax
    ;; file discriptor
    mov     rdi, 1
    ;; syscall number
    mov     rax, 1
    syscall
    ;; <IF FUNC HAS ARGS> pop the pushed rbp back to original
    pop     rbp
    ;; exit process hand back to before the call
    ret

_start:
    mov  rdi, 10 
    call print
    mov rax, 60
    mov rdi, 0
    syscall
