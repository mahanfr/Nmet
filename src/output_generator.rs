use std::error::Error;
use std::fs::{self, File};
use std::io::{BufWriter, Write};

use crate::codegen::Codegen;
use crate::utils::get_program_name;

pub fn x86_64_nasm_generator(path: String, codegen: Codegen) -> Result<(), Box<dyn Error>> {
    fs::create_dir_all("./build").unwrap();
    let out_name = get_program_name(path);
    let stream = File::create(format!("./build/{}.asm", out_name)).unwrap();
    let mut file = BufWriter::new(stream);
    println!("[info] Generating asm files...");
    file.write_all(b";; This File is Automatically Created Using The Nmet Compiler\n")?;
    file.write_all(b";; Under MIT License Copyright Mahan Farzaneh 2023-2024\n\n")?;

    if !codegen.data_buf.is_empty() {
        file.write_all(b"section .data\n")?;
        for data in &codegen.data_buf {
            file.write_all(data.as_bytes())?;
            file.write_all(b"\n")?;
        }
        file.write_all(b"\n")?;
    }

    file.write_all(b"section .text\n")?;
    file.write_all(b"global _start\n")?;
    file.write_all(codegen.text_section_asm().as_bytes())?;
    // for instruct in &codegen.instruct_buf {
    //     file.write_all(instruct.to_string().as_bytes())?;
    //     file.write_all(b"\n")?;
    // }

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
