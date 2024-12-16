use std::{collections::HashMap, fs};

use crate::{codegen::elf::{header::ElfHeader, sections::SectionHeader}, log_error, log_warn};

#[derive(Debug, Clone)]
pub struct ElfSection {}

#[derive(Debug, Clone)]
pub struct ElfFile {
    pub header: ElfHeader,
    pub sections: HashMap<String, ElfSection>,
    pub sec_headers: Vec<SectionHeader>
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
            sections: HashMap::new(),
            sec_headers: Vec::new()
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
    let mut ep = ElfParser::new(source.clone(), cur);
    let header = ElfHeader::parse(&mut ep).unwrap();
    let mut elf_file = ElfFile::new(header);
    println!("{:X?}", header);
    
    ep.jump_to_byte(header.e_shoff as usize);
    for _ in 0..header.e_shnum {
        let before_cur = ep.cur;
        elf_file.sec_headers.push(SectionHeader::parse(&mut ep).unwrap());
        if ep.cur - before_cur != header.e_shentsize as usize {
            log_error!("size of cur is wrong {} but is {}",
                header.e_shentsize, ep.cur - before_cur);
        }
    }
    println!("{:#?}",elf_file.sec_headers);
    elf_file
}
