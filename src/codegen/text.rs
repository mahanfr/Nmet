use std::error::Error;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

use crate::codegen::Codegen;

pub fn x86_64_nasm_generator(output: &Path, codegen: Codegen) -> Result<(), Box<dyn Error>> {
    let stream = File::create(output.with_extension("asm")).unwrap();
    let mut file = BufWriter::new(stream);
    file.write_all(b";; This File is Automatically Created Using The Nmet Compiler\n")?;
    file.write_all(b";; Under MIT License Copyright Mahan Farzaneh 2023-2024\n\n")?;


    file.write_all(b"section .text\n")?;
    file.write_all(b"global _start\n")?;
    file.write_all(codegen.text_section_asm().as_bytes())?;
    // for instruct in &codegen.instruct_buf {
    //     file.write_all(instruct.to_string().as_bytes())?;
    //     file.write_all(b"\n")?;
    // }

    if !codegen.data_buf.is_empty() {
        file.write_all(b"section .data\n")?;
        for data in &codegen.data_buf {
            file.write_all(data.to_string().as_bytes())?;
            file.write_all(b"\n")?;
        }
        file.write_all(b"\n")?;
    }
    if !codegen.bss_buf.is_empty() {
        file.write_all(b"\nsection .bss\n")?;
        for bss in codegen.bss_buf {
            file.write_all(bss.to_string().as_bytes())?;
            file.write_all(b"\n")?;
        }
    }

    file.flush().unwrap();
    Ok(())
}
