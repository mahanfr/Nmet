use std::{collections::HashMap, fs};

use crate::codegen::elf::header::ElfHeader;

#[derive(Debug, Clone)]
pub struct ElfSection {}

#[derive(Debug, Clone)]
pub struct ElfFile {
    pub header: ElfHeader,
    pub sections: HashMap<String, ElfSection>
}

#[derive(Debug, Clone)]
pub struct ElfParser {
    pub bytes: Vec<u8>,
    pub cur: usize,
    pub start: usize,
}
impl ElfParser {
    pub fn new(bytes: Vec<u8>, cur: usize) -> Self {
        Self {
            bytes,
            cur,
            start: cur,
        }
    }

    pub fn get_range(&self, size: usize) -> &[u8] {
        &self.bytes[self.cur..self.cur + size]
    }

    pub fn jump_to_byte(&mut self, loc: usize) {
        self.cur = loc + self.start;
    }
}

impl ElfFile {
    pub fn new(header: ElfHeader) -> Self {
        Self {
            header,
            sections: HashMap::new()
        }
    }
}

pub fn parse_elf_objfile(file_path: String) -> ElfFile {
    let source = fs::read(file_path).unwrap();
    let mut cur = 0;
    for (index, bt) in source.iter().enumerate() {
        if *bt == 0x7F {
            cur = index;
            if source[index + 1] == 0x45 &&
               source[index + 2] == 0x4C &&
               source[index + 3] == 0x46 {
                break;
            }
        }
    }
    let mut elf_parser = ElfParser::new(source.clone(), cur);
    let header = ElfHeader::parse(&mut elf_parser).unwrap();
    println!("{:X?}", header);
    elf_parser.jump_to_byte(header.e_shoff as usize);
    ElfFile::new(header)
}
