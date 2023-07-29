;; This File is Automatically Created Useing Nemet Parser
;; Under MIT License Copyright MahanFarzaneh 2023-2024

section .text
global _start
print:
    ;; <IF FUNC HAS ARGS> Maintain rpb
    push    rbp
    ;; <IF FUNC HAS ARGS> move stack pointer to rbp
    mov     rbp, rsp
    ;; allocate 40 bytes 8 for rdi(input of function) and 32 chars
    ;; mov rdi to the first alocated memory
    mov     qword [rbp- 8], rdi
    mov     qword [rbp-16], 1
    ;; rax holds dividend
    mov     eax, 4
    ;; rdx holds modulo
    xor     rdx, rdx
    ;; rcx holds divisor before and result after the div operation
    mov     rcx, 2
    ;; rcx = rax / rcx and rdx = rax % rcx
    div     rcx
    ;; turn result into character
    mov     rax, rdx
    add     rax, 48
    mov     qword [rbp-48], rax
    mov     qword [rbp-47], 10
    ;; rsi pointer to first memory
    mov     rax, rbp
    sub     rax, 48
    mov     rsi, rax
    ;; rdx size
    mov     rdx, 2
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
    mov  rdi, 35
    call print
    mov rax, 60
    mov rdi, 0
    syscall
