use std::{
    collections::HashMap,
    fs::{self, File},
    io::{BufWriter, Write},
};

use crate::{
    codegen::instructions::{Instr, Opr},
    compiler::CompilerContext,
    utils::get_program_name,
};

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
    bytes
}

struct ReFillableBytes {
    index: usize,
    size: u8,
}

fn generate_text_section(cc: &CompilerContext) -> Vec<u8> {
    todo!();
    let mut bytes = Vec::new();
    let mut investigation_list = Vec::<(String, ReFillableBytes)>::new();
    let mut lables_map = HashMap::<String, usize>::new();
    let mut last_main_lable = String::new();
    bytes
}

fn section_offset(data_lists: &Vec<Vec<u8>>, index: usize) -> usize {
    let mut sum = 0;
    if index > 0 {
        for item in data_lists.iter().take(index) {
            let a = item.len();
            if a % 16 != 0 {
                sum += ((a / 16) + 1) * 16;
            } else {
                sum += a;
            }
        }
    }
    ((data_lists.len() + 1) * 64) + 64 + sum
}

// SHF_WRITE: 1
// SHF_ALLOC: 2
// SHF_EXECINSTR: 4
fn generate_section_headers(data_lists: &Vec<Vec<u8>>, shstrtab: &[u32]) -> Vec<u8> {
    let mut bytes = vec![0; 64];

    // .text
    bytes.extend(shstrtab[0].to_le_bytes());
    bytes.extend(1u32.to_le_bytes());
    bytes.extend(6u64.to_le_bytes());
    bytes.extend(0u64.to_le_bytes());
    bytes.extend(section_offset(data_lists, 0).to_le_bytes());
    bytes.extend((data_lists[0].len() as u64).to_le_bytes());
    bytes.extend(0u32.to_le_bytes());
    bytes.extend(0u32.to_le_bytes());
    bytes.extend(16u64.to_le_bytes());
    bytes.extend(0u64.to_le_bytes());

    // .shstrtab
    bytes.extend(shstrtab[1].to_le_bytes());
    bytes.extend(3u32.to_le_bytes());
    bytes.extend(0u64.to_le_bytes());
    bytes.extend(0u64.to_le_bytes());
    bytes.extend(section_offset(data_lists, 1).to_le_bytes());
    bytes.extend((data_lists[1].len() as u64).to_le_bytes());
    bytes.extend(0u32.to_le_bytes());
    bytes.extend(0u32.to_le_bytes());
    bytes.extend(1u64.to_le_bytes());
    bytes.extend(0u64.to_le_bytes());

    // .symtab
    bytes.extend(shstrtab[2].to_le_bytes());
    bytes.extend(2u32.to_le_bytes());
    bytes.extend(0u64.to_le_bytes());
    bytes.extend(0u64.to_le_bytes());
    bytes.extend(section_offset(data_lists, 2).to_le_bytes());
    bytes.extend((data_lists[2].len() as u64).to_le_bytes());
    bytes.extend(4u32.to_le_bytes());
    bytes.extend(3u32.to_le_bytes());
    bytes.extend(8u64.to_le_bytes());
    bytes.extend(24u64.to_le_bytes());

    // .strtab
    bytes.extend(shstrtab[3].to_le_bytes());
    bytes.extend(3u32.to_le_bytes());
    bytes.extend(0u64.to_le_bytes());
    bytes.extend(0u64.to_le_bytes());
    bytes.extend(section_offset(data_lists, 3).to_le_bytes());
    bytes.extend((data_lists[3].len() as u64).to_le_bytes());
    bytes.extend(0u32.to_le_bytes());
    bytes.extend(0u32.to_le_bytes());
    bytes.extend(1u64.to_le_bytes());
    bytes.extend(0u64.to_le_bytes());
    bytes
}

fn generate_symtab(strtab: Vec<u32>) -> Vec<u8> {
    let mut bytes = vec![0; 24];

    bytes.extend(strtab[0].to_le_bytes());
    bytes.push(4u8);
    bytes.push(0);
    bytes.extend(0xfff1u16.to_le_bytes());
    bytes.extend(0u64.to_le_bytes());
    bytes.extend(0u64.to_le_bytes());

    bytes.extend(0u32.to_le_bytes());
    bytes.push(3u8);
    bytes.push(0);
    bytes.extend(1u16.to_le_bytes());
    bytes.extend(0u64.to_le_bytes());
    bytes.extend(0u64.to_le_bytes());

    bytes.extend(strtab[1].to_le_bytes());
    bytes.push(0x10);
    bytes.push(0);
    bytes.extend(1u16.to_le_bytes());
    bytes.extend(0u64.to_le_bytes());
    bytes.extend(0u64.to_le_bytes());

    // if bytes.len() % 16 != 0 {
    //     let padding_size = 16 - (bytes.len() % 16);
    //     bytes.extend(repeat(0).take(padding_size));
    // }
    bytes
}

fn generate_strtab() -> (Vec<u8>, Vec<u32>) {
    let mut table = Vec::<u32>::new();
    let mut data = [0].to_vec();

    table.push(data.len() as u32);
    data.extend(b"./tests/elf.nmt");
    data.push(0);

    table.push(data.len() as u32);
    data.extend(b"_start");
    data.push(0);

    // if data.len() % 16 != 0 {
    //     let padding_size = 16 - (data.len() % 16);
    //     data.extend(repeat(0).take(padding_size));
    // }

    (data, table)
}

fn generate_shsrttab() -> (Vec<u8>, Vec<u32>) {
    let mut tab = Vec::<u32>::new();
    let mut data = [0].to_vec();

    tab.push(data.len() as u32);
    data.extend(b".text");
    data.push(0);

    tab.push(data.len() as u32);
    data.extend(b".shstrtab");
    data.push(0);

    tab.push(data.len() as u32);
    data.extend(b".symtab");
    data.push(0);

    tab.push(data.len() as u32);
    data.extend(b".strtab");
    data.push(0);

    // if data.len() % 16 != 0 {
    //     let padding_size = 16 - (data.len() % 16);
    //     data.extend(repeat(0).take(padding_size));
    // }

    (data, tab)
}

pub fn generate_elf(path: String, cc: &CompilerContext) {
    let text_sec = generate_text_section(cc);
    let (shstr_data, shstr_rows) = generate_shsrttab();
    let (strtab_data, str_rows) = generate_strtab();
    let symtab = generate_symtab(str_rows);
    let final_data = vec![text_sec, shstr_data, symtab, strtab_data];
    let section_headers = generate_section_headers(&final_data, &shstr_rows);
    let elf_header = generate_header();

    let mut bytes = Vec::new();
    bytes.extend(elf_header);
    bytes.extend(section_headers);
    for data in final_data.iter() {
        bytes.extend(data);
        while bytes.len() % 16 != 0 {
            bytes.push(0);
        }
    }

    fs::create_dir_all("./build").unwrap();
    let program_name = format!("test_{}", get_program_name(path));
    let stream = File::create(format!("./build/{program_name}.o")).unwrap();
    let mut file = BufWriter::new(stream);
    file.write_all(&bytes).unwrap();
    file.flush().unwrap();
    println!("[info] Elf Object file Generated!");
}
