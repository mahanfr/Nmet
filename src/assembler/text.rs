use std::error::Error;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

use crate::compiler::{CompilerContext, NSType};

pub fn x86_64_nasm_generator(output: &Path, cc: &CompilerContext) -> Result<(), Box<dyn Error>> {
    let stream = File::create(output.with_extension("asm")).unwrap();
    let mut file = BufWriter::new(stream);
    file.write_all(b";; This File is Automatically Created Using The Nmet Compiler\n")?;
    file.write_all(b";; Under MIT License Copyright Mahan Farzaneh 2023-2024\n\n")?;

    file.write_all(b"section .text\n")?;

    for mod_name in cc.namespace_map.values() {
        let NSType::Ffi(_, ff) = mod_name else {
            continue;
        };
        let exten = format!("extern {ff}\n");
        file.write_all(exten.as_bytes())?;
    }

    file.write_all(b"global _start\n")?;
    file.write_all(cc.codegen.text_section_asm().as_bytes())?;
    // for instruct in &codegen.instruct_buf {
    //     file.write_all(instruct.to_string().as_bytes())?;
    //     file.write_all(b"\n")?;
    // }

    if !cc.codegen.data_buf.is_empty() {
        file.write_all(b"section .data\n")?;
        for data in cc.codegen.data_buf.values() {
            file.write_all(data.to_string().as_bytes())?;
            file.write_all(b"\n")?;
        }
        file.write_all(b"\n")?;
    }
    if !cc.codegen.bss_buf.is_empty() {
        file.write_all(b"\nsection .bss\n")?;
        for bss in cc.codegen.bss_buf.iter() {
            file.write_all(bss.to_string().as_bytes())?;
            file.write_all(b"\n")?;
        }
    }

    file.flush().unwrap();
    Ok(())
}
