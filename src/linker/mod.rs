use std::{collections::HashMap, fs};

use crate::formats::elf::{
    header::ElfHeader,
    sections::{parse_section, STRTABSec, Section, SectionHeader},
};

#[derive(Debug, Clone)]
pub struct ElfFile {
    pub header: ElfHeader,
    pub sections: HashMap<String, Box<dyn Section>>,
    pub sec_headers: Vec<SectionHeader>,
}

impl ElfFile {
    pub fn new(header: ElfHeader) -> Self {
        Self {
            header,
            sections: HashMap::new(),
            sec_headers: Vec::new(),
        }
    }
}

fn find_elf_start(bytes: &[u8]) -> usize {
    bytes
        .windows(4)
        .position(|x| x == b"\x7FELF")
        .expect("Provided file has no ELF file!")
}

#[allow(dead_code)]
pub fn parse_elf_objfile(file_path: String) -> ElfFile {
    let source = fs::read(file_path).unwrap();
    let elf_start = find_elf_start(&source);
    let elf_bytes = &source[elf_start..];
    let header = ElfHeader::from_bytes(elf_bytes);
    let mut elf_file = ElfFile::new(header);

    for i in 0..header.e_shnum {
        let start = (header.e_shoff + (i * header.e_shentsize) as u64) as usize;
        let end = start + header.e_shentsize as usize;
        elf_file
            .sec_headers
            .push(SectionHeader::from_bytes(&elf_bytes[start..end]));
    }
    let shst_header = elf_file.sec_headers[elf_file.header.e_shstrndx as usize];
    let shstrtab = STRTABSec::new_from_bytes(
        ".shstrtab",
        &elf_bytes[shst_header.sh_offset as usize
            ..(shst_header.sh_offset + shst_header.sh_size) as usize],
    );
    for sh in elf_file.sec_headers.iter() {
        if sh.sh_type == 0 {
            continue;
        }
        let name = shstrtab.get(sh.sh_name as usize);
        let index = sh.sh_offset as usize;
        let size = sh.sh_size as usize;
        if let Some(sec) = parse_section(sh, &name, &elf_bytes[index..index + size]) {
            elf_file.sections.insert(name.clone(), sec);
        }
    }

    println!("{:#?}", elf_file.sections);
    elf_file
}

#[allow(dead_code)]
pub fn generate_elf_exec() {}
