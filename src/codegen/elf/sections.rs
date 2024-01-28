use std::collections::HashMap;

use crate::utils::IBytes;

/// Generic Section for refrencing and storing
/// different code sections including .text, .data, or .bss
/// and internal sections like symtab or strtab
pub trait Section {
    fn to_bytes(&self) -> IBytes;
    fn header(&self, sh_name: u32, sh_offset: u64) -> SectionHeader;
    fn name(&self) -> &'static str;
    fn size(&self) -> usize;
}
// Section header (Shdr)
//   A file's section header table lets one locate all the file's
//   sections.  The section header table is an array of Elf32_Shdr or
//   Elf64_Shdr structures.  The ELF header's e_shoff member gives the
//   byte offset from the beginning of the file to the section header
//   table.  e_shnum holds the number of entries the section header
//   table contains.  e_shentsize holds the size in bytes of each
//   entry.
//   A section header table index is a subscript into this array.
//   Some section header table indices are reserved: the initial entry
//   and the indices between SHN_LORESERVE and SHN_HIRESERVE.  The
//   initial entry is used in ELF extensions for e_phnum, e_shnum, and
//   e_shstrndx; in other cases, each field in the initial entry is
//   set to zero.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub struct SectionHeader {
    sh_name: u32,
    sh_type: u32,
    sh_flags: u64,
    sh_addr: u64,
    sh_offset: u64,
    sh_size: u64,
    sh_link: u32,
    sh_info: u32,
    sh_addralign: u64,
    sh_entsize: u64,
}

impl Default for SectionHeader {
    fn default() -> Self {
        Self {
            sh_name: 0,
            sh_type: 0,
            sh_flags: 0,
            sh_addr: 0,
            sh_offset: 0,
            sh_size: 0,
            sh_link: 0,
            sh_info: 0,
            sh_addralign: 0,
            sh_entsize: 0,
        }
    }
}

impl SectionHeader {
    pub fn to_bytes(&self) -> IBytes {
        let mut bytes = Vec::new();
        bytes.extend(self.sh_name.to_le_bytes());
        bytes.extend(self.sh_type.to_le_bytes());
        bytes.extend(self.sh_flags.to_le_bytes());
        bytes.extend(self.sh_addr.to_le_bytes());
        bytes.extend(self.sh_offset.to_le_bytes());
        bytes.extend(self.sh_size.to_le_bytes());
        bytes.extend(self.sh_link.to_le_bytes());
        bytes.extend(self.sh_info.to_le_bytes());
        bytes.extend(self.sh_addralign.to_le_bytes());
        bytes.extend(self.sh_entsize.to_le_bytes());
        bytes
    }
}
// section .text
//   This section holds the "text", or executable instructions,
//   of a program.  This section is of type SHT_PROGBITS.  The
//   attributes used are SHF_ALLOC and SHF_EXECINSTR.
//   [Nr] Name              Type             Address           Offset
//        Size              EntSize          Flags  Link  Info  Align
//   [ 1] .text             PROGBITS         0000000000000000  00000180
//        0000000000000195  0000000000000000  AX       0     0     16
#[derive(Debug, Clone)]
pub struct TextSec {
    data: IBytes,
}
impl TextSec {
    pub fn new(data: IBytes) -> Self {
        Self { data }
    }
}
impl Section for TextSec {

    fn to_bytes(&self) -> IBytes {
        let mut bytes = vec![];
        bytes.extend(self.data.clone());
        while bytes.len() % 16 != 0 {
           bytes.push(0);
        }
        bytes
    }

    fn size(&self) -> usize {
        self.data.len()
    }

    fn name(&self) -> &'static str {
        ".text"
    }

    fn header(&self, sh_name: u32, sh_offset: u64) -> SectionHeader {
        SectionHeader {
            sh_name,
            sh_type: 1,
            sh_flags: 6,
            sh_addr: 0,
            sh_offset,
            sh_size: self.data.len() as u64,
            sh_link: 0,
            sh_info: 0,
            sh_addralign: 16,
            sh_entsize: 0,
        }
    }
}
// section .shstrtab
//   This section holds section names.  This section is of type
//   SHT_STRTAB.  No attribute types are used.
//   [Nr] Name              Type             Address           Offset
//        Size              EntSize          Flags  Link  Info  Align
//   [ 2] .shstrtab         STRTAB           0000000000000000  00000320
//        0000000000000021  0000000000000000           0     0     1
#[derive(Debug, Clone)]
pub struct ShstrtabSec {
    map: HashMap<String, usize>,
    data: IBytes,
}
impl ShstrtabSec {
    pub fn new() -> Self {
        Self {
            data: vec![0],
            map: HashMap::new(),
        }
    }

    pub fn insert(&mut self, name: String) {
        self.map.insert(name.clone(), self.data.len());
        self.data.extend(name.bytes());
        self.data.push(0);
    }

    pub fn index(&self, name: &str) -> u32 {
        *self.map.get(name).unwrap() as u32
    }
}
impl Section for ShstrtabSec {
    fn to_bytes(&self) -> IBytes {
        let mut bytes = vec![];
        bytes.extend(self.data.clone());
        while bytes.len() % 16 != 0 {
           bytes.push(0);
        }
        bytes
    }

    fn size(&self) -> usize {
        self.data.len()
    }

    fn name(&self) -> &'static str {
        ".shstrtab"
    }

    fn header(&self, sh_name: u32, sh_offset: u64) -> SectionHeader {
        SectionHeader {
            sh_name,
            sh_type: 3,
            sh_flags: 0,
            sh_addr: 0,
            sh_offset,
            sh_size: self.data.len() as u64,
            sh_link: 0,
            sh_info: 0,
            sh_addralign: 1,
            sh_entsize: 0,
        }
    }
}
// section .symtab
//   This section holds a symbol table.  If the file has a
//   loadable segment that includes the symbol table, the
//   section's attributes will include the SHF_ALLOC bit.
//   Otherwise, the bit will be off.  This section is of type
//   SHT_SYMTAB.
//   [Nr] Name              Type             Address           Offset
//        Size              EntSize          Flags  Link  Info  Align
//   [ 3] .symtab           SYMTAB           0000000000000000  00000350
//        0000000000000090  0000000000000018           4     5     8
#[derive(Debug, Clone)]
pub struct SymtabSec {
    data: Vec<SymItem>,
    pub strtab_index: u32,
}
impl SymtabSec {
    pub fn new() -> Self {
        Self {
            data: vec![SymItem::default()],
            strtab_index: 0,
        }
    }

    pub fn insert(&mut self, item: SymItem) {
        self.data.push(item);
    }
}
impl Section for SymtabSec {
    fn to_bytes(&self) -> IBytes {
        let mut bytes = vec![];
        for item in self.data.iter() {
            bytes.extend(item.to_bytes());
        }
        while bytes.len() % 16 != 0 {
           bytes.push(0);
        }
        bytes
    }
    fn size(&self) -> usize {
        self.data.len() * 24
    }

    fn name(&self) -> &'static str {
        ".symtab"
    }

    fn header(&self, sh_name: u32, sh_offset: u64) -> SectionHeader {
        SectionHeader {
            sh_name,
            sh_type: 2,
            sh_flags: 0,
            sh_addr: 0,
            sh_offset,
            sh_size: (self.data.len() * 24) as u64,
            sh_link: self.strtab_index,
            sh_info: self.data.len() as u32,
            sh_addralign: 8,
            sh_entsize: 24,
        }
    }
}
// SymItem entry of string and symbol tables
//   An object file's symbol table holds information needed to locate
//   and relocate a program's symbolic definitions and references.  A
//   symbol table index is a subscript into this array.
//   [Nr] Name              Type             Address           Offset
//        Size              EntSize          Flags  Link  Info  Align
#[allow(unused)]
#[derive(Debug, Clone)]
pub struct SymItem {
    pub st_name: u32,
    pub st_info: u8,
    pub st_other: u8,
    pub st_shndx: u16,
    pub st_value: u64,
    pub st_size: u64,
}
impl SymItem {
    pub fn to_bytes(&self) -> IBytes {
        let mut bytes = vec![];
        bytes.extend(self.st_name.to_le_bytes());
        bytes.extend(self.st_other.to_le_bytes());
        bytes.extend(self.st_info.to_le_bytes());
        bytes.extend(self.st_shndx.to_le_bytes());
        bytes.extend(self.st_value.to_le_bytes());
        bytes.extend(self.st_size.to_le_bytes());
        bytes
    }
}
impl Default for SymItem {
    fn default() -> Self {
        Self {
            st_name: 0,
            st_info: 0,
            st_other: 0,
            st_shndx: 0,
            st_value: 0,
            st_size: 0,
        }
    }
}
// section .strtab
//   This section holds strings, most commonly the strings that
//   represent the names associated with symbol table entries.
//   If the file has a loadable segment that includes the
//   symbol string table, the section's attributes will include
//   the SHF_ALLOC bit.  Otherwise, the bit will be off.  This
//   section is of type SHT_STRTAB.
//   [Nr] Name              Type             Address           Offset
//        Size              EntSize          Flags  Link  Info  Align
//   [ 4] .strtab           STRTAB           0000000000000000  000003e0
//        0000000000000035  0000000000000000           0     0     1

#[derive(Debug, Clone)]
pub struct StrtabSec {
    map: HashMap<String, usize>,
    data: IBytes,
}
impl StrtabSec {
    pub fn new() -> Self {
        Self {
            data: vec![0],
            map: HashMap::new(),
        }
    }

    pub fn insert(&mut self, name: &String) {
        self.map.insert(name.clone(), self.data.len());
        self.data.extend(name.bytes());
        self.data.push(0);
    }

    pub fn index(&self, name: &String) -> u32 {
        *self.map.get(name).unwrap() as u32
    }
}
impl Section for StrtabSec {
    fn to_bytes(&self) -> IBytes {
        let mut bytes = vec![];
        bytes.extend(self.data.clone());
        while bytes.len() % 16 != 0 {
           bytes.push(0);
        }
        bytes
    }

    fn size(&self) -> usize {
        self.data.len()
    }

    fn name(&self) -> &'static str {
        ".strtab"
    }

    fn header(&self, sh_name: u32, sh_offset: u64) -> SectionHeader {
        SectionHeader {
            sh_name,
            sh_type: 3,
            sh_flags: 0,
            sh_addr: 0,
            sh_offset,
            sh_size: self.data.len() as u64,
            sh_link: 0,
            sh_info: 0,
            sh_addralign: 1,
            sh_entsize: 0,
        }
    }
}
