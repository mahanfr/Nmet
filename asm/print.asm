print:
    push    rbp
    mov     rbp, rsp
    sub     rsp, 64
    mov     qword [rbp-56], rdi
    mov     qword [rbp-8], 1
    mov     eax, 32
    sub     rax, qword [rbp-8]
    mov     BYTE [rbp-48+rax], 10
.L3:
    mov     rcx, qword [rbp-56]
    mov     rdx, -3689348814741910323
    mov     rax, rcx
    mul     rdx
    shr     rdx, 3
    mov     rax, rdx
    sal     rax, 2
    add     rax, rdx
    add     rax, rax
    sub     rcx, rax
    mov     rdx, rcx
    mov     eax, edx
    lea     edx, [rax+48]
    mov     eax, 31
    sub     rax, qword [rbp-8]
    mov     byte [rbp-48+rax], dl
    add     qword [rbp-8], 1
    mov     rax, qword [rbp-56]
    mov     rdx, -3689348814741910323
    mul     rdx
    mov     rax, rdx
    shr     rax, 3
    mov     qword [rbp-56], rax
    cmp     qword [rbp-56], 0
    jne     .L3

    ;; rsi pointer to first memory
    mov     eax, 32
    sub     rax, qword [rbp-8]
    lea     rdx, [rbp-48]
    add     rax, rdx
    mov     rsi, rax
    ;; rdx size
    mov     rbx, qword [rbp-8]
    mov     rdx, rbx
    ;; file discriptor
    mov     rdi, 1
    ;; syscall number
    mov     rax, 1
    syscall
    ;; <IF FUNC HAS ARGS> pop the pushed rbp back to original
    leave
    ;; exit process hand back to before the call
    ret
