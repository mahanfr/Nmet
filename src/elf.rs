use std::{
    fs::File,
    io::{BufWriter, Write}, path::Path,
};

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
fn generate_section_headers(data_lists: &Vec<Vec<u8>>, shstrtab: &[u32], symtab_num: usize) -> Vec<u8> {
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
    // TODO: Make this dynamic
    bytes.extend((symtab_num as u32).to_le_bytes());
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

fn generate_symtab(cc: &mut CompilerContext, strtab: Vec<u32>) -> (Vec<u8>, usize) {
    let mut bytes = vec![0; 24];

    let mut indx = 0;
    bytes.extend(strtab[indx].to_le_bytes());
    bytes.push(4u8);
    bytes.push(0);
    bytes.extend(0xfff1u16.to_le_bytes());
    bytes.extend(0u64.to_le_bytes());
    bytes.extend(0u64.to_le_bytes());
    indx += 1;

    // Sections
    bytes.extend(0u32.to_le_bytes());
    bytes.push(3u8);
    bytes.push(0);
    bytes.extend(1u16.to_le_bytes());
    bytes.extend(0u64.to_le_bytes());
    bytes.extend(0u64.to_le_bytes());
    // Sections

    // Lables
    for item in cc.codegen.rel_map.iter() {
        if item.0 == "_start" {
            continue;
        }
        bytes.extend(strtab[indx].to_le_bytes());
        bytes.push(0u8);
        bytes.push(0);
        bytes.extend(1u16.to_le_bytes());
        let Some(val) = item.1 else {
            panic!("symbol ({}) not found",item.0);
        };
        bytes.extend((*val as u64).to_le_bytes());
        bytes.extend(0u64.to_le_bytes());
        indx += 1;
    }
    // Labels

    // Global
    bytes.extend(strtab[indx].to_le_bytes());
    bytes.push(0x10);
    bytes.push(0);
    bytes.extend(1u16.to_le_bytes());
    bytes.extend(0u64.to_le_bytes());
    bytes.extend(0u64.to_le_bytes());
    // Gloval
    (bytes, indx + 2)
}

fn generate_strtab(cc: &CompilerContext) -> (Vec<u8>, Vec<u32>) {
    let mut table = Vec::<u32>::new();
    let mut data = vec![0];

    table.push(data.len() as u32);
    data.extend(b"./tests/elf.nmt\0");

    for label in cc.codegen.rel_map.iter() {
        if label.0 == "_start" {
            continue;
        }
        table.push(data.len() as u32);
        data.extend(label.0.bytes());
        data.push(0);
    }

    table.push(data.len() as u32);
    data.extend(b"_start\0");
    
    (data, table)
}

fn generate_shsrttab() -> (Vec<u8>, Vec<u32>) {
    let mut tab = Vec::<u32>::new();
    let mut data = [0].to_vec();

    tab.push(data.len() as u32);
    data.extend(b".text\0");

    tab.push(data.len() as u32);
    data.extend(b".shstrtab\0");

    tab.push(data.len() as u32);
    data.extend(b".symtab\0");

    tab.push(data.len() as u32);
    data.extend(b".strtab\0");

    (data, tab)
}

pub fn generate_elf(out_path: &Path, cc: &mut CompilerContext) {
    let text_sec = cc.codegen.text_section_bytes();
    let (shstr_data, shstr_rows) = generate_shsrttab();
    let (strtab_data, str_rows) = generate_strtab(&cc);
    let (symtab, symtab_num) = generate_symtab(cc, str_rows);
    let final_data = vec![text_sec, shstr_data, symtab, strtab_data];
    let section_headers = generate_section_headers(&final_data, &shstr_rows, symtab_num);
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

    let stream = File::create(out_path.with_extension("o")).unwrap();
    let mut file = BufWriter::new(stream);
    file.write_all(&bytes).unwrap();
    file.flush().unwrap();
}
