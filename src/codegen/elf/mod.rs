#![allow(dead_code)]

use std::{
    fs::File,
    io::{BufWriter, Write},
    path::Path,
};

use crate::{
    codegen::elf::sections::SectionHeader, compiler::CompilerContext, st_info, st_visibility,
    utils::IBytes,
};

mod flags;
mod header;
mod sections;

use self::{
    flags::{STB_GLOBAL, STB_LOCAL, STT_FILE, STT_NOTYPE, STT_SECTION, STV_DEFAULT},
    header::ElfHeader,
    sections::{Section, ShstrtabSec, StrtabSec, SymItem, SymtabSec, TextSec},
};

pub fn generate_elf(out_path: &Path, cc: &mut CompilerContext) {
    let mut elf_object = Elf::new();
    elf_object.add_section(&TextSec::new(cc.codegen.text_section_bytes()));
    let file_content = elf_object.bytes(cc);
    let stream = File::create(out_path.with_extension("o")).unwrap();
    let mut file = BufWriter::new(stream);
    file.write_all(&file_content).unwrap();
    file.flush().unwrap();
}

struct Elf {
    sections: Vec<Box<dyn Section>>,
    shstrtab: ShstrtabSec,
    strtab: StrtabSec,
    symtab: SymtabSec,
}

impl Elf {
    pub fn new() -> Self {
        Self {
            sections: Vec::new(),
            shstrtab: ShstrtabSec::new(),
            strtab: StrtabSec::new(),
            symtab: SymtabSec::new(),
        }
    }

    pub fn add_section<T>(&mut self, section: &T)
    where
        T: Section + Clone + 'static,
    {
        self.shstrtab.insert(section.name().to_string());
        self.sections.push(Box::new((*section).clone()));
    }

    pub fn bytes(&mut self, cc: &mut CompilerContext) -> IBytes {
        let mut bytes = Vec::new();
        self.set_symbols(cc);
        self.shstrtab.insert(".shstrtab".to_string());
        self.shstrtab.insert(".symtab".to_string());
        self.shstrtab.insert(".strtab".to_string());
        let header = ElfHeader::new(
            (self.sections.len() + 4) as u16,
            (self.sections.len() + 1) as u16,
        );
        bytes.extend(header.to_bytes());
        self.section_header_bytes(&mut bytes);
        bytes.extend(self.sections_bytes());
        bytes
    }

    fn sections_bytes(&self) -> IBytes {
        let mut bytes = Vec::new();
        for section in self.sections.iter() {
            bytes.extend(section.to_bytes());
        }
        bytes.extend(self.shstrtab.to_bytes());
        bytes.extend(self.symtab.to_bytes());
        bytes.extend(self.strtab.to_bytes());
        bytes
    }

    fn section_header_bytes(&self, bytes: &mut IBytes) {
        bytes.extend(SectionHeader::default().to_bytes());
        let mut loc = 64 + ((self.sections.len() + 4) * 64);
        for section in self.sections.iter() {
            bytes.extend(
                section
                    .header(self.shstrtab.index(section.name()), loc as u64, None, None)
                    .to_bytes(),
            );
            loc += section.size();
        }
        bytes.extend(
            self.shstrtab
                .header(self.shstrtab.index(".shstrtab"), loc as u64, None, None)
                .to_bytes(),
        );
        loc += self.shstrtab.size();
        bytes.extend(
            self.symtab
                .header(self.shstrtab.index(".symtab"), loc as u64, Some((self.sections.len() + 3) as u32), None)
                .to_bytes(),
        );
        loc += self.symtab.size();
        bytes.extend(
            self.strtab
                .header(self.shstrtab.index(".strtab"), loc as u64, None, None)
                .to_bytes(),
        );
    }

    fn set_symbols(&mut self, cc: &mut CompilerContext) {
        self.strtab.insert(&cc.program_file);
        self.symtab.insert(SymItem {
            st_name: self.strtab.index(&cc.program_file),
            st_info: st_info!(STB_LOCAL, STT_FILE),
            st_other: st_visibility!(STV_DEFAULT),
            st_shndx: 0xfff1,
            st_size: 0,
            st_value: 0,
        });

        self.symtab.insert(SymItem {
            st_name: 0,
            st_info: st_info!(STB_LOCAL, STT_SECTION),
            st_other: st_visibility!(STV_DEFAULT),
            st_shndx: 1,
            st_size: 0,
            st_value: 0,
        });

        for (label, sym) in cc.codegen.rel_map.iter() {
            // push symbol name to list
            self.strtab.insert(label);
            // push symbol info to sym_list
            let info = match label == "__start" {
                true => st_info!(STB_GLOBAL, STT_NOTYPE),
                false => st_info!(STB_LOCAL, STT_NOTYPE),
            };
            let shndx = match sym.1 {
                super::SymbolType::TextSec => {
                    self.sections.iter().position(|x| x.name() == ".text").unwrap() + 1
                },
                super::SymbolType::DataSec => {
                    self.sections.iter().position(|x| x.name() == ".data").unwrap() + 1
                },
                _ => 0
            };
            
            self.symtab.insert(SymItem {
                st_name: self.strtab.index(label),
                st_info: info,
                st_other: st_visibility!(STV_DEFAULT),
                st_shndx: shndx as u16,
                st_size: 0,
                st_value: sym.0 as u64,
            });
        }
    }
}
