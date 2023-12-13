use std::error::Error;
use std::fs::{self, File};
use std::io::{BufWriter, Write};

use crate::codegen::Codegen;
use crate::utils::get_program_name;

// pub fn llvm_generator(
//     path: String,
//     instruct_buf: Vec<String>,
//     data_buf: Vec<String>,
// ) -> Result<(), Box<dyn Error>> {
//     fs::create_dir_all("./build").unwrap();
//     let out_name = get_program_name(path);
//     let stream = File::create(format!("./build/{}.ll", out_name)).unwrap();
//     let mut file = BufWriter::new(stream);
//     println!("[info] Generating asm files...");
//     file.write_all(b";; TODO: NOT IMPLEMENTERD YET!\n")?;
//     if !data_buf.is_empty() {
//         file.write_all(b"section .data\n")?;
//         for data in &data_buf {
//             file.write_all(data.as_bytes())?;
//         }
//     }
//     file.write_all(b"\n")?;
//     for instruct in &instruct_buf {
//         file.write_all(instruct.as_bytes())?;
//     }
//     file.flush().unwrap();
//     Ok(())
// }

pub fn x86_64_nasm_generator(path: String, codegen: Codegen) -> Result<(), Box<dyn Error>> {
    fs::create_dir_all("./build").unwrap();
    let out_name = get_program_name(path);
    let stream = File::create(format!("./build/{}.asm", out_name)).unwrap();
    let mut file = BufWriter::new(stream);
    println!("[info] Generating asm files...");
    file.write_all(b";; This File is Automatically Created Useing Nemet Parser\n")?;
    file.write_all(b";; Under MIT License Copyright MahanFarzaneh 2023-2024\n\n")?;

    file.write_all(b"\n")?;
    if !codegen.data_buf.is_empty() {
        file.write_all(b"section .data\n")?;
        for data in &codegen.data_buf {
            file.write_all(data.as_bytes())?;
            file.write_all(b"\n")?;
        }
    }
    file.write_all(b"\n")?;

    file.write_all(b"section .text\n")?;
    file.write_all(b"global _start\n")?;
    file.write_all(b"print:\n")?;
    file.write_all(b"    push    rbp\n")?;
    file.write_all(b"    mov     rbp, rsp\n")?;
    file.write_all(b"    sub     rsp, 64\n")?;
    file.write_all(b"    mov     qword [rbp-56], rdi\n")?;
    file.write_all(b"    mov     qword [rbp-8], 1\n")?;
    file.write_all(b"    mov     eax, 32\n")?;
    file.write_all(b"    sub     rax, qword [rbp-8]\n")?;
    file.write_all(b"    mov     BYTE [rbp-48+rax], 10\n")?;
    file.write_all(b".L3:\n")?;
    file.write_all(b"    mov     rcx, qword [rbp-56]\n")?;
    file.write_all(b"    mov     rdx, -3689348814741910323\n")?;
    file.write_all(b"    mov     rax, rcx\n")?;
    file.write_all(b"    mul     rdx\n")?;
    file.write_all(b"    shr     rdx, 3\n")?;
    file.write_all(b"    mov     rax, rdx\n")?;
    file.write_all(b"    sal     rax, 2\n")?;
    file.write_all(b"    add     rax, rdx\n")?;
    file.write_all(b"    add     rax, rax\n")?;
    file.write_all(b"    sub     rcx, rax\n")?;
    file.write_all(b"    mov     rdx, rcx\n")?;
    file.write_all(b"    mov     eax, edx\n")?;
    file.write_all(b"    lea     edx, [rax+48]\n")?;
    file.write_all(b"    mov     eax, 31\n")?;
    file.write_all(b"    sub     rax, qword [rbp-8]\n")?;
    file.write_all(b"    mov     byte [rbp-48+rax], dl\n")?;
    file.write_all(b"    add     qword [rbp-8], 1\n")?;
    file.write_all(b"    mov     rax, qword [rbp-56]\n")?;
    file.write_all(b"    mov     rdx, -3689348814741910323\n")?;
    file.write_all(b"    mul     rdx\n")?;
    file.write_all(b"    mov     rax, rdx\n")?;
    file.write_all(b"    shr     rax, 3\n")?;
    file.write_all(b"    mov     qword [rbp-56], rax\n")?;
    file.write_all(b"    cmp     qword [rbp-56], 0\n")?;
    file.write_all(b"    jne     .L3\n")?;
    file.write_all(b"    mov     eax, 32\n")?;
    file.write_all(b"    sub     rax, qword [rbp-8]\n")?;
    file.write_all(b"    lea     rdx, [rbp-48]\n")?;
    file.write_all(b"    add     rax, rdx\n")?;
    file.write_all(b"    mov     rsi, rax\n")?;
    file.write_all(b"    mov     rbx, qword [rbp-8]\n")?;
    file.write_all(b"    mov     rdx, rbx\n")?;
    file.write_all(b"    mov     rdi, 1\n")?;
    file.write_all(b"    mov     rax, 1\n")?;
    file.write_all(b"    syscall\n")?;
    file.write_all(b"    leave\n")?;
    file.write_all(b"    ret\n")?;

    for instruct in &codegen.instruct_buf {
        file.write_all(instruct.as_bytes())?;
        file.write_all(b"\n")?;
    }

    if !codegen.bss_buf.is_empty() {
        file.write_all(b"\nsection .bss\n")?;
        for bss in codegen.bss_buf {
            file.write_all(bss.as_bytes())?;
            file.write_all(b"\n")?;
        }
    }

    file.flush().unwrap();
    Ok(())
}
