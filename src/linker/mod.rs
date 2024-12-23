use std::{collections::HashMap, fs::{self, File}, io::{Write, BufWriter}};

use crate::{formats::elf::{
    flags::{STB_GLOBAL, STT_NOTYPE, STT_SECTION, STV_DEFAULT}, header::{EType, ElfHeader}, program::ProgramHeader, sections::{parse_section, STRTABSec, Section, SectionHeader, SymItem}, ElfObject
}, st_info, st_visibility};

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

pub fn parse_elf_objfile(file_path: String) -> ElfFile {
    let source = fs::read(file_path).unwrap();
    let elf_start = find_elf_start(&source);
    let elf_bytes = &source[elf_start..];
    let header = ElfHeader::from_bytes(&elf_bytes);
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

pub fn generate_elf_exec(object: &mut ElfObject) {
    let mut header = object.get_header();
    header.e_type = EType::Exec;
    header.e_entry = 0x401000;
    header.e_phoff = 64;
    header.e_phnum = 2;
    header.e_phensize = 56;
    object.strtab.insert("_bss_start");
    object.strtab.insert("_edata");
    object.strtab.insert("_end");
    for (index, sym) in object.symtab.data.iter_mut().enumerate() {
        if index == object.symtab.start_of_global {
            break;
        }
        if sym.st_shndx > 1 && sym.st_shndx < 0xfff0 {
            sym.st_value += 0x400000;
        }
    }
    object.symtab.data.retain(|x| x.st_info & 0xf == STT_SECTION);
    object.symtab.insert(SymItem {
        st_name: object.strtab.index("_bss_start").unwrap(),
        st_info: st_info!(STB_GLOBAL, STT_NOTYPE),
        st_other: st_visibility!(STV_DEFAULT),
        st_shndx: object.get_sec_index(".text") as u16,
        st_size: 0,
        st_value: 0x402000,
    });
    object.symtab.insert(SymItem {
        st_name: object.strtab.index("_edata").unwrap(),
        st_info: st_info!(STB_GLOBAL, STT_NOTYPE),
        st_other: st_visibility!(STV_DEFAULT),
        st_shndx: object.get_sec_index(".text") as u16,
        st_size: 0,
        st_value: 0x402000,
    });
    object.symtab.insert(SymItem {
        st_name: object.strtab.index("_end").unwrap(),
        st_info: st_info!(STB_GLOBAL, STT_NOTYPE),
        st_other: st_visibility!(STV_DEFAULT),
        st_shndx: object.get_sec_index(".text") as u16,
        st_size: 0,
        st_value: 0x402000,
    });
    object.symtab.data[object.symtab.start_of_global].st_value = 0x401000;
    let mut sec_headers = object.section_headers();
    for (index ,sec) in sec_headers.iter_mut().enumerate() {
        if index == 0 {
            continue;
        }
        sec.sh_offset += 0x1000 - sec.sh_offset;
    }
    let size_of_segment = (sec_headers.len() * 64) + object.section_sizes();
    let ph_header = ProgramHeader::new_default(1, 0, 0b100, 0x400000, 0xb0);
    let ph_program = ProgramHeader::new_default(1, 0, 0b101, 0x401000, size_of_segment as u64);
    
    let stream = File::create("./build/test").unwrap();
    let mut file = BufWriter::new(stream);
    let mut bytes = Vec::<u8>::new();
    bytes.extend(&header.to_bytes());
    bytes.extend(&ph_header.to_bytes());
    bytes.extend(&ph_program.to_bytes());
    bytes.resize(0x1000, 0);
    bytes.extend(&object.sections_bytes());
    for sh in sec_headers.iter() {
        bytes.extend(&sh.to_bytes());
    }
    file.write_all(&bytes).unwrap();
    file.flush().unwrap();
}
