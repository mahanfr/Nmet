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
    sections::{Section, ShstrtabSec, StrtabSec, SymItem, SymtabSec, TextSec, DataSec, RelaSec, BssSec},
};

pub fn generate_elf(out_path: &Path, cc: &mut CompilerContext) {
    let mut elf_object = Elf::new();
    elf_object.add_section(&TextSec::new(cc.codegen.text_section_bytes()));
    if !cc.codegen.data_buf.is_empty() {
        elf_object.add_section(&DataSec::new(&cc.codegen.data_buf));
    }
    if !cc.codegen.bss_buf.is_empty() {
        println!("REACHED");
        elf_object.add_section(&BssSec::new(cc.codegen.bss_buf.iter().map(|x| x.size).sum()));
    }
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
    rela_text: RelaSec,
}

impl Elf {
    pub fn new() -> Self {
        Self {
            sections: Vec::new(),
            shstrtab: ShstrtabSec::new(),
            strtab: StrtabSec::new(),
            symtab: SymtabSec::new(),
            rela_text: RelaSec::new(),
        }
    }

    pub fn add_section<T>(&mut self, section: &T)
    where
        T: Section + Clone + 'static,
    {
        self.shstrtab.insert(section.name().to_string());
        self.sections.push(Box::new((*section).clone()));
    }

    fn sections_count(&self) -> usize {
        if self.rela_text.is_empty() {
            self.sections.len() + 4
        } else {
            self.sections.len() + 4 + 1
        }
    }  

    pub fn bytes(&mut self, cc: &mut CompilerContext) -> IBytes {
        let mut bytes = Vec::new();
        for item in cc.codegen.rela_map.iter_mut() {
            item.r_info = ((self.get_sec_index(&item.sec_name) as u64 + 1) << 32) | 0xb;
            self.rela_text.push(item.to_owned());
        }
        self.set_symbols(cc);
        self.shstrtab.insert(".shstrtab".to_string());
        self.shstrtab.insert(".symtab".to_string());
        self.shstrtab.insert(".strtab".to_string());
        if !cc.codegen.rela_map.is_empty() {
            self.shstrtab.insert(".rela.text".to_string());
        }
        let header = ElfHeader::new(
            self.sections_count() as u16,
            self.get_sec_index(".shstrtab") as u16,
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
        if !self.rela_text.is_empty() {
            bytes.extend(self.rela_text.to_bytes());
        }
        bytes
    }

    fn get_sec_index(&self, tag: &str) -> u32 {
        match tag {
            "" => 0,
            ".shstrtab" => (self.sections.len() + 1) as u32,
            ".symtab" => (self.sections.len() + 2) as u32,
            ".strtab" => (self.sections.len() + 3) as u32,
            ".rela.text" => (self.sections.len() + 4) as u32,
            _ => (self.sections.iter().position(|x| x.name() == tag).unwrap() + 1) as u32
        }
    }

    fn section_header_bytes(&self, bytes: &mut IBytes) {
        bytes.extend(SectionHeader::default().to_bytes());
        let mut loc = 64 + (self.sections_count() * 64);
        for section in self.sections.iter() {
            let (link_tag, info_tag) = section.link_and_info();
            let link = self.get_sec_index(link_tag.unwrap_or(""));
            let info = self.get_sec_index(info_tag.unwrap_or(""));
            bytes.extend(
                section
                    .header(self.shstrtab.index(section.name()), loc as u64, link, info)
                    .to_bytes(),
            );
            loc += section.size();
        }
        bytes.extend(
            self.shstrtab
                .header(self.shstrtab.index(".shstrtab"), loc as u64, 0, 0)
                .to_bytes(),
        );
        loc += self.shstrtab.size();
        bytes.extend(
            self.symtab
                .header(self.shstrtab.index(".symtab"), loc as u64, self.get_sec_index(".strtab"), 0)
                .to_bytes(),
        );
        loc += self.symtab.size();
        bytes.extend(
            self.strtab
                .header(self.shstrtab.index(".strtab"), loc as u64, 0, 0)
                .to_bytes(),
        );
        loc += self.strtab.size();
        if !self.rela_text.is_empty() {
            bytes.extend(
                self.rela_text
                .header(self.shstrtab.index(".rela.text"), 
                        loc as u64, 
                        self.get_sec_index(".symtab"),
                        self.get_sec_index(".text"))
                .to_bytes(),
                );
        }
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

        for (indx, _) in self.sections.iter().enumerate() {
            self.symtab.insert(SymItem {
                st_name: 0,
                st_info: st_info!(STB_LOCAL, STT_SECTION),
                st_other: st_visibility!(STV_DEFAULT),
                st_shndx: indx as u16 + 1,
                st_size: 0,
                st_value: 0,
            });
        }
        for (label, sym) in cc.codegen.symbols_map.iter() {
            // push symbol name to list
            self.strtab.insert(label);
            // push symbol info to sym_list
            let info = match label == "_start" {
                true => st_info!(STB_GLOBAL, STT_NOTYPE),
                false => st_info!(STB_LOCAL, STT_NOTYPE),
            };
            let shndx = match sym.1 {
                super::SymbolType::TextSec => {
                    self.get_sec_index(".text")
                },
                super::SymbolType::DataSec => {
                    self.get_sec_index(".data")
                },
                super::SymbolType::BssSec => {
                    self.get_sec_index(".bss")
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
