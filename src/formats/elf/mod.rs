#![allow(dead_code)]

use std::{
    fs::File,
    io::{BufWriter, Write},
    path::Path,
};

use crate::{
    compiler::CompilerContext, formats::elf::sections::SectionHeader,
    st_info, st_visibility, utils::IBytes,
};

pub mod flags;
pub mod header;
pub mod program;
pub mod sections;

use self::{
    flags::{STB_GLOBAL, STB_LOCAL, STT_FILE, STT_NOTYPE, STT_SECTION, STV_DEFAULT},
    header::ElfHeader,
    sections::{NOBITSSec, PROGBITSSec, RELASec, STRTABSec, SYMTABSec, Section, SymItem},
};

#[allow(unused)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SymbolType {
    Global,
    Ffi,
    DataSec,
    BssSec,
    TextSec,
    Other,
}

pub fn generate_bin(out_path: &Path, cc: &mut CompilerContext) {
    let file_content = cc.codegen.text_section_bytes();
    let stream = File::create(out_path.with_extension("bin")).unwrap();
    let mut file = BufWriter::new(stream);
    file.write_all(&file_content).unwrap();
    file.flush().unwrap();
}

pub fn generate_elf(out_path: &Path, cc: &mut CompilerContext) {
    let mut elf_object = ElfObject::new();
    elf_object.add_section(&PROGBITSSec::new(
        ".text",
        0x6,
        16,
        cc.codegen.text_section_bytes(),
    ));
    if !cc.codegen.data_buf.is_empty() {
        elf_object.add_section(&PROGBITSSec::new(
            ".data",
            0x3,
            4,
            PROGBITSSec::dmap_to_data(&cc.codegen.data_buf),
        ));
    }
    if !cc.codegen.bss_buf.is_empty() {
        elf_object.add_section(&NOBITSSec::new(
            ".bss",
            cc.codegen.bss_buf.iter().map(|x| x.size).sum(),
        ));
    }
    let file_content = elf_object.bytes(cc);
    let stream = File::create(out_path.with_extension("o")).unwrap();
    let mut file = BufWriter::new(stream);
    file.write_all(&file_content).unwrap();
    file.flush().unwrap();
}

pub struct ElfObject {
    sections: Vec<Box<dyn Section>>,
    shstrtab: STRTABSec,
    pub strtab: STRTABSec,
    pub symtab: SYMTABSec,
    rela_text: RELASec,
}

impl ElfObject {
    pub fn new() -> Self {
        Self {
            sections: Vec::new(),
            shstrtab: STRTABSec::new(".shstrtab"),
            strtab: STRTABSec::new(".strtab"),
            symtab: SYMTABSec::new(".symtab"),
            rela_text: RELASec::new(".rela.text".into()),
        }
    }

    pub fn section_sizes(&self) -> usize {
        let mut size = 0;
        for sec in self.sections.iter() {
            size += sec.size();
        }
        size += self.shstrtab.size();
        size += self.strtab.size();
        size += self.symtab.size();
        // size += self.rela_text.size();
        size
    }

    pub fn add_section<T>(&mut self, section: &T)
    where
        T: Section + Clone + 'static,
    {
        self.shstrtab.insert(&section.name().to_string());
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
        self.set_symbols(cc);
        for item in cc.codegen.rela_map.iter_mut() {
            if item.sym_type == SymbolType::Ffi {
                let indx = self.strtab.index(&item.sym_name).unwrap();
                item.r_section = self.symtab.find(indx) as u32;
            } else {
                item.r_section = self.get_sec_index(&item.sym_name) + 1;
            }
            self.rela_text.push(item.to_owned());
        }
        self.shstrtab.insert(".shstrtab");
        self.shstrtab.insert(".symtab");
        self.shstrtab.insert(".strtab");
        if !cc.codegen.rela_map.is_empty() {
            self.shstrtab.insert(".rela.text");
        }
        bytes.extend(self.get_header().to_bytes());
        self.section_header_bytes(&mut bytes);
        bytes.extend(self.sections_bytes());
        bytes
    }

    pub fn get_header(&self) -> ElfHeader {
        assert!(
            self.get_sec_index(".shstrtab") != 0,
            "Header is not ready yet!"
        );
        ElfHeader::new(
            self.sections_count() as u16,
            self.get_sec_index(".shstrtab") as u16,
        )
    }

    pub fn sections_bytes(&self) -> IBytes {
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

    pub fn get_sec_index(&self, tag: &str) -> u32 {
        match tag {
            "" => 0,
            ".shstrtab" => (self.sections.len() + 1) as u32,
            ".symtab" => (self.sections.len() + 2) as u32,
            ".strtab" => (self.sections.len() + 3) as u32,
            ".rela.text" => (self.sections.len() + 4) as u32,
            _ => (self.sections.iter().position(|x| x.name() == tag).unwrap() + 1) as u32,
        }
    }

    pub fn section_headers(&self) -> Vec<SectionHeader> {
        let mut secs = vec![SectionHeader::default()];
        let mut loc = 64 + (self.sections_count() * 64);
        for section in self.sections.iter() {
            let (link_tag, info_tag) = section.link_and_info();
            let link = self.get_sec_index(link_tag.unwrap_or(""));
            let info = self.get_sec_index(info_tag.unwrap_or(""));
            secs.push(section.header(
                self.shstrtab.index(&section.name()).unwrap(),
                loc as u64,
                link,
                info,
            ));
            loc += section.size();
        }
        secs.push(self.shstrtab.header(
            self.shstrtab.index(".shstrtab").unwrap(),
            loc as u64,
            0,
            0,
        ));
        loc += self.shstrtab.size();
        secs.push(self.symtab.header(
            self.shstrtab.index(".symtab").unwrap(),
            loc as u64,
            self.get_sec_index(".strtab"),
            0,
        ));
        loc += self.symtab.size();
        secs.push(
            self.strtab
                .header(self.shstrtab.index(".strtab").unwrap(), loc as u64, 0, 0),
        );
        loc += self.strtab.size();
        if !self.rela_text.is_empty() {
            secs.push(self.rela_text.header(
                self.shstrtab.index(".rela.text").unwrap(),
                loc as u64,
                self.get_sec_index(".symtab"),
                self.get_sec_index(".text"),
            ));
        }
        secs
    }

    fn section_header_bytes(&self, bytes: &mut IBytes) {
        for sec in self.section_headers().iter() {
            bytes.extend(sec.to_bytes());
        }
    }

    fn set_symbols(&mut self, cc: &mut CompilerContext) {
        self.strtab.insert(&cc.program_file);
        self.symtab.insert(SymItem {
            st_name: self.strtab.index(&cc.program_file).unwrap(),
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

            if label == "_start" || sym.1 == SymbolType::Ffi {
                continue;
            }
            // push symbol info to sym_list
            let info = match sym.1 == SymbolType::Ffi {
                true => st_info!(STB_GLOBAL, STT_NOTYPE),
                false => st_info!(STB_LOCAL, STT_NOTYPE),
            };
            let shndx = match sym.1 {
                SymbolType::TextSec => self.get_sec_index(".text"),
                SymbolType::DataSec => self.get_sec_index(".data"),
                SymbolType::BssSec => self.get_sec_index(".bss"),
                _ => 0,
            };

            self.symtab.insert(SymItem {
                st_name: self.strtab.index(label).unwrap(),
                st_info: info,
                st_other: st_visibility!(STV_DEFAULT),
                st_shndx: shndx as u16,
                st_size: 0,
                st_value: sym.0 as u64,
            });
        }
        // Global items
        self.symtab.set_global_start();
        for item in cc.codegen.ffi_map.values() {
            self.symtab.insert(SymItem {
                st_name: self.strtab.index(item).unwrap(),
                st_info: st_info!(STB_GLOBAL, STT_NOTYPE),
                st_other: st_visibility!(STV_DEFAULT),
                st_shndx: 0,
                st_size: 0,
                st_value: 0,
            });
        }
        self.symtab.insert(SymItem {
            st_name: self.strtab.index("_start").unwrap(),
            st_info: st_info!(STB_GLOBAL, STT_NOTYPE),
            st_other: st_visibility!(STV_DEFAULT),
            st_shndx: self.get_sec_index(".text") as u16,
            st_size: 0,
            st_value: 0,
        });
    }
}
