use std::iter::repeat;

use crate::compiler::CompilerContext;

pub fn generate_header() -> Vec<u8> {
    let mut bytes = Vec::new();
    let e_ident = vec![0x7f, b'E', b'L', b'F', 2, 1, 1];
    bytes.extend(e_ident);
    bytes.resize(16, 0);
    bytes.extend(1u16.to_le_bytes());
    bytes.extend(0x3eu16.to_le_bytes());
    bytes.extend(1u32.to_le_bytes());
    bytes.extend(0u64.to_le_bytes());
    // Program Hedear start
    bytes.extend(0u64.to_le_bytes());
    // Section Hedear start
    bytes.extend(64u64.to_le_bytes());
    // Flags
    bytes.extend(0u32.to_le_bytes());
    // This header size
    bytes.extend(64u16.to_le_bytes());
    // size of program headers
    bytes.extend(0u16.to_le_bytes());
    // number of program headers
    bytes.extend(0u16.to_le_bytes());
    // size of sections headers
    bytes.extend(64u16.to_le_bytes());
    // DYNAMIC: number of section headers
    bytes.extend(5u16.to_le_bytes());
    // DYNAMIC: section header string table index
    bytes.extend(2u16.to_le_bytes());
    return bytes;
}

fn generate_section_headers() -> Vec<u8> {
    Vec::new()    
}

fn generate_program_headers() -> Vec<u8> {
    repeat(0).take(64).collect::<Vec<u8>>()
}

pub fn generate_elf(cc: &CompilerContext) {
    let mut elf_content = Vec::<u8>::new();
    let elf_header = generate_header();
    let program_headers = generate_program_headers();
}


#[test]
fn e_ident_test() {
    // let mut elf_content = Vec::<u8>::new();
    // generate_header(&mut elf_content);
    // assert_eq!(64, elf_content.len());
    // generate_program_headers(&mut elf_content);
    // assert_eq!(128, elf_content.len());
}
