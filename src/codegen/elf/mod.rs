#![allow(dead_code)]

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
    sections::{Section, ShstrtabSec, StrtabSec, SymItem, SymtabSec},
};

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

    pub fn add_section(&mut self, section: Box<dyn Section>) {
        self.shstrtab.insert(section.name().to_string());
        self.sections.push(section);
    }

    pub fn bytes(&mut self, cc: &mut CompilerContext) -> IBytes {
        let mut bytes = Vec::new();
        let header: ElfHeader;

        self.set_symbols(cc);
        header = ElfHeader::new(
            (self.sections.len() + 4) as u16,
            (self.sections.len() + 1) as u16,
        );
        bytes.extend(header.to_bytes());
        bytes.extend(self.section_header_bytes());
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

    fn section_header_bytes(&self) -> IBytes {
        let mut bytes = Vec::new();
        bytes.extend(SectionHeader::default().to_bytes());
        for section in self.sections.iter() {
            bytes.extend(
                section
                    .header(self.shstrtab.index(section.name()), bytes.len() as u64)
                    .to_bytes(),
            );
        }
        bytes.extend(
            self.shstrtab
            .header(self.shstrtab.index(".shstrtab"), bytes.len() as u64)
            .to_bytes(),
        );
        bytes.extend(
            self.symtab
            .header(self.shstrtab.index(".symtab"), bytes.len() as u64)
            .to_bytes(),
        );
        bytes.extend(
            self.strtab
            .header(self.shstrtab.index(".strtab"), bytes.len() as u64)
            .to_bytes(),
        );
        bytes
    }

    fn set_symbols(&mut self, cc: &mut CompilerContext) {
        self.symtab.strtab_index = (self.sections.len() + 3) as u32;
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

        for (label, loc) in cc.codegen.rel_map.iter() {
            // push symbol name to list
            self.strtab.insert(&label);
            // push symbol info to sym_list
            let info = match label == "__start" {
                true => st_info!(STB_GLOBAL, STT_NOTYPE),
                false => st_info!(STB_LOCAL, STT_NOTYPE),
            };
            self.symtab.insert(SymItem {
                st_name: self.strtab.index(&label),
                st_info: info,
                st_other: st_visibility!(STV_DEFAULT),
                st_shndx: 1,
                st_size: 0,
                st_value: loc.unwrap() as u64,
            });
        }
    }
}
