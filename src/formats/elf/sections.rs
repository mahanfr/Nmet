use std::{any::Any, collections::BTreeMap};

use crate::{assembler::data_bss::DataItem, utils::IBytes};

use super::{flags::SHFlags, SymbolType};

#[repr(u32)]
#[derive(Debug, Clone)]
pub enum SHType {
    Null = 0x0,
    Progbits = 0x1,
    Symtab = 0x2,
    Strtab = 0x3,
    Rela = 0x4,
    Hash = 0x5,
    Dynamic = 0x6,
    Note = 0x7,
    Nobits = 0x8,
    Rel = 0x9,
    Shlib = 0x0A,
    Dynsym = 0x0B,
    InitArray = 0x0E,
    FiniArray = 0x0F,
    PreinitArray = 0x10,
    Group = 0x11,
    SymtabShndx = 0x12,
    Num = 0x13,
    Loos = 0x60000000,
    GNUAttributes = 0x6ffffff5,
    GNUHash = 0x6ffffff6,
    GNULiblist = 0x6ffffff7,
    Checksum = 0x6ffffff8,
    Losunw = 0x6ffffffa,
    GNUVerdef = 0x6ffffffd,
    GNUVerneed = 0x6ffffffe,
    Hios = 0x6fffffff,
    LOProc = 0x70000000,
    HIProc = 0x7fffffff,
    LOUser = 0x80000000,
    HIUser = 0x8fffffff,
}
impl SHType {
    pub fn from_u32(value: u32) -> Self {
        match value {
            0x0 => Self::Null,
            0x1 => Self::Progbits,
            0x2 => Self::Symtab,
            0x3 => Self::Strtab,
            0x4 => Self::Rela,
            0x5 => Self::Hash,
            0x6 => Self::Dynamic,
            0x7 => Self::Note,
            0x8 => Self::Nobits,
            0x9 => Self::Rel,
            0x0A => Self::Shlib,
            0x0B => Self::Dynsym,
            0x0E => Self::InitArray,
            0x0F => Self::FiniArray,
            0x10 => Self::PreinitArray,
            0x11 => Self::Group,
            0x12 => Self::SymtabShndx,
            0x13 => Self::Num,
            _ => unimplemented!(),
        }
    }
}

/// Generic Section for refrencing and storing
/// different code sections including .text, .data, or .bss
/// and internal sections like symtab or strtab
pub trait Section: CloneSection + Any {
    fn to_bytes(&self) -> IBytes;
    fn header(&self, sh_name: u32, sh_offset: u64, sh_link: u32, sh_info: u32) -> SectionHeader;
    fn name(&self) -> String;
    fn as_any(&self) -> &dyn Any;
    fn insert(&mut self, bytes: &[u8]) -> usize;
    fn padded_size(&self) -> usize;
    fn size(&self) -> usize;
    fn link_and_info(&self) -> (Option<&'static str>, Option<&'static str>);
}
impl std::fmt::Debug for dyn Section {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}
pub trait CloneSection {
    fn clone_box(&self) -> Box<dyn Section>;
}
impl<T> CloneSection for T
where
    T: 'static + Section + Clone,
{
    fn clone_box(&self) -> Box<dyn Section> {
        Box::new(self.clone())
    }
}
impl Clone for Box<dyn Section> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

pub fn parse_section(sh: &SectionHeader, name: &str, bytes: &[u8]) -> Option<Box<dyn Section>> {
    match SHType::from_u32(sh.sh_type) {
        SHType::Null => None,
        SHType::Nobits => Some(Box::new(NOBITSSec::new(name, sh.sh_size as usize))),
        SHType::Progbits => Some(Box::new(PROGBITSSec::new(
            name,
            sh.sh_flags,
            sh.sh_addralign,
            bytes.to_vec(),
        ))),
        SHType::Symtab => Some(Box::new(SYMTABSec::from_bytes(sh, name, bytes))),
        SHType::Strtab => Some(Box::new(STRTABSec::new_from_bytes(name, bytes))),
        SHType::Rela => Some(Box::new(RELASec::from_bytes(sh, name, bytes))),
        SHType::Note => Some(Box::new(NOTESec::new(name, bytes.to_vec()))),
        _ => todo!("{name} cannot be parsed by the linker!"),
    }
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
#[derive(Debug, Default, Clone, Copy)]
pub struct SectionHeader {
    pub sh_name: u32,
    pub sh_type: u32,
    pub sh_flags: u64,
    pub sh_addr: u64,
    pub sh_offset: u64,
    pub sh_size: u64,
    pub sh_link: u32,
    pub sh_info: u32,
    pub sh_addralign: u64,
    pub sh_entsize: u64,
}

impl SectionHeader {
    pub fn to_bytes(self) -> IBytes {
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

    pub fn from_bytes(bytes: &[u8]) -> Self {
        Self {
            sh_name: slice_to_u64(&bytes[0..4]) as u32,
            sh_type: slice_to_u64(&bytes[4..8]) as u32,
            sh_flags: slice_to_u64(&bytes[8..16]),
            sh_addr: slice_to_u64(&bytes[16..24]),
            sh_offset: slice_to_u64(&bytes[24..32]),
            sh_size: slice_to_u64(&bytes[32..40]),
            sh_link: slice_to_u64(&bytes[40..44]) as u32,
            sh_info: slice_to_u64(&bytes[44..48]) as u32,
            sh_addralign: slice_to_u64(&bytes[48..56]),
            sh_entsize: slice_to_u64(&bytes[56..64]),
        }
    }
}
// section NOBITS
//   This section holds uninitialized data that contributes to
//   the program's memory image.  By definition, the system
//   initializes the data with zeros when the program begins to
//   run.  This section is of type SHT_NOBITS.  The attribute
//   types are SHF_ALLOC and SHF_WRITE.
//   [Nr] Name              Type             Address           Offset
//        Size              EntSize          Flags  Link  Info  Align
//   [ 2] .bss              NOBITS           0000000000000000  00000300
//        0000000000000008  0000000000000000  WA       0     0     4
#[derive(Debug, Clone)]
pub struct NOBITSSec {
    size: usize,
    name: String,
}
impl NOBITSSec {
    pub fn new(name: &str, size: usize) -> Self {
        Self {
            name: name.to_string(),
            size,
        }
    }
}
impl Section for NOBITSSec {
    fn to_bytes(&self) -> IBytes {
        vec![]
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn insert(&mut self, _: &[u8]) -> usize {
        0
    }

    fn link_and_info(&self) -> (Option<&'static str>, Option<&'static str>) {
        (None, None)
    }

    fn size(&self) -> usize {
        0
    }

    fn padded_size(&self) -> usize {
        0
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn header(&self, sh_name: u32, sh_offset: u64, _: u32, _: u32) -> SectionHeader {
        SectionHeader {
            sh_name,
            sh_type: 8,
            sh_flags: 3,
            sh_addr: 0,
            sh_offset,
            sh_size: self.size as u64,
            sh_link: 0,
            sh_info: 0,
            sh_addralign: 4,
            sh_entsize: 0,
        }
    }
}
#[derive(Debug, Clone)]
pub struct NOTESec {
    name: String,
    data: IBytes,
}
impl NOTESec {
    pub fn new(name: &str, data: IBytes) -> Self {
        Self {
            name: name.to_string(),
            data,
        }
    }
}
impl Section for NOTESec {
    fn to_bytes(&self) -> IBytes {
        let mut data = self.data.clone();
        data.resize(self.padded_size(), 0);
        data
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn insert(&mut self, bytes: &[u8]) -> usize {
        let index = self.data.len();
        self.data.extend(bytes);
        index
    }

    fn link_and_info(&self) -> (Option<&'static str>, Option<&'static str>) {
        (None, None)
    }

    fn padded_size(&self) -> usize {
        let data_len = self.data.len();
        data_len + (16 - (data_len % 16))
    }

    fn size(&self) -> usize {
        self.data.len()
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn header(&self, sh_name: u32, sh_offset: u64, _: u32, _: u32) -> SectionHeader {
        SectionHeader {
            sh_name,
            sh_type: 1,
            sh_flags: SHFlags::Alloc as u64,
            sh_addr: 0,
            sh_offset,
            sh_size: self.size() as u64,
            sh_link: 0,
            sh_info: 0,
            sh_addralign: 8,
            sh_entsize: 0,
        }
    }
}
// section PROGBITS
//   This section holds initialized data that contribute to the
//   program's memory image.  This section is of type
//   SHT_PROGBITS.  The attribute types are SHF_ALLOC and
//   SHF_WRITE
//   [Nr] Name              Type             Address           Offset
//        Size              EntSize          Flags  Link  Info  Align
//   [ 1] .data             PROGBITS         0000000000000000  00000200
//        000000000000000c  0000000000000000  WA       0     0     4
#[derive(Debug, Clone)]
pub struct PROGBITSSec {
    name: String,
    align: u64,
    flags: u64,
    data: IBytes,
}
impl PROGBITSSec {
    pub fn new(name: &str, flags: u64, alignment: u64, data: IBytes) -> Self {
        Self {
            name: name.to_string(),
            flags,
            align: alignment,
            data,
        }
    }

    pub fn dmap_to_data(items: &BTreeMap<String, DataItem>) -> IBytes {
        let mut data = Vec::new();
        for item in items.values() {
            data.extend(item.data.clone());
        }
        data
    }
}
impl Section for PROGBITSSec {
    fn to_bytes(&self) -> IBytes {
        let mut bytes = vec![];
        bytes.extend(self.data.clone());
        bytes.resize(self.padded_size(), 0);
        bytes
    }

    fn insert(&mut self, bytes: &[u8]) -> usize {
        let index = self.data.len();
        self.data.extend(bytes);
        index
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn link_and_info(&self) -> (Option<&'static str>, Option<&'static str>) {
        (None, None)
    }

    fn padded_size(&self) -> usize {
        let data_len = self.data.len();
        data_len + (16 - (data_len % 16))
    }

    fn size(&self) -> usize {
        self.data.len()
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn header(&self, sh_name: u32, sh_offset: u64, _: u32, _: u32) -> SectionHeader {
        SectionHeader {
            sh_name,
            sh_type: 1,
            sh_flags: self.flags,
            sh_addr: 0,
            sh_offset,
            sh_size: self.size() as u64,
            sh_link: 0,
            sh_info: 0,
            sh_addralign: self.align,
            sh_entsize: 0,
        }
    }
}
// section STRTAB
//   This section holds section names.  This section is of type
//   SHT_STRTAB.  No attribute types are used.
//   [Nr] Name              Type             Address           Offset
//        Size              EntSize          Flags  Link  Info  Align
//   [ 2] .shstrtab         STRTAB           0000000000000000  00000320
//        0000000000000021  0000000000000000           0     0     1
#[derive(Debug, Clone)]
pub struct STRTABSec {
    name: String,
    data: IBytes,
}
impl STRTABSec {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            data: vec![0],
        }
    }

    pub fn new_from_bytes(name: &str, bytes: &[u8]) -> Self {
        Self {
            name: name.to_string(),
            data: bytes.to_vec(),
        }
    }

    pub fn get(&self, index: usize) -> String {
        let mut i = index;
        let mut buffer = String::new();
        while i < self.data.len() && self.data[i] != 0 {
            buffer.push(self.data[i] as char);
            i += 1;
        }
        buffer
    }

    pub fn insert(&mut self, name: &str) -> u32 {
        match self.index(name) {
            Some(x) => x,
            None => {
                let index = self.data.len();
                self.data.append(&mut name.as_bytes().to_vec());
                // THIS THING WASTED MY TIME FOR 6 HOURS
                self.data.push(0);
                index as u32
            }
        }
    }

    pub fn index(&self, name: &str) -> Option<u32> {
        let substr = name.as_bytes();
        if substr.len() > self.data.len() {
            return None;
        }
        self.data
            .windows(substr.len())
            .position(|win| win == substr)
            .map(|x| x as u32)
    }
}
impl Section for STRTABSec {
    fn to_bytes(&self) -> IBytes {
        let mut bytes = vec![];
        bytes.extend(self.data.clone());
        bytes.resize(self.padded_size(), 0);
        bytes
    }

    fn insert(&mut self, bytes: &[u8]) -> usize {
        self.insert(core::str::from_utf8(bytes).unwrap()) as usize
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn link_and_info(&self) -> (Option<&'static str>, Option<&'static str>) {
        (None, None)
    }

    fn padded_size(&self) -> usize {
        let data_len = self.data.len();
        data_len + (16 - (data_len % 16))
    }

    fn size(&self) -> usize {
        self.data.len()
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn header(&self, sh_name: u32, sh_offset: u64, _: u32, _: u32) -> SectionHeader {
        SectionHeader {
            sh_name,
            sh_type: 3,
            sh_flags: 0,
            sh_addr: 0,
            sh_offset,
            sh_size: self.size() as u64,
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
pub struct SYMTABSec {
    name: String,
    pub data: Vec<SymItem>,
    pub start_of_global: usize,
}
impl SYMTABSec {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            data: vec![SymItem::default()],
            start_of_global: 4,
        }
    }

    pub fn from_bytes(sh: &SectionHeader, name: &str, bytes: &[u8]) -> Self {
        let mut symtab = Self::new(name);
        let entries = sh.sh_size / sh.sh_entsize;
        for i in 0..entries {
            let index = (i * sh.sh_entsize) as usize;
            let item = SymItem::from_bytes(&bytes[index..index + sh.sh_entsize as usize]);
            symtab.insert(item);
        }
        symtab
    }

    pub fn insert(&mut self, item: SymItem) {
        self.data.push(item);
    }

    pub fn find(&self, name_index: u32) -> usize {
        self.data
            .iter()
            .position(|&r| r.st_name == name_index)
            .unwrap()
    }

    pub fn set_global_start(&mut self) {
        self.start_of_global = self.data.len();
    }
}
impl Section for SYMTABSec {
    fn to_bytes(&self) -> IBytes {
        let mut bytes = vec![];
        for item in self.data.iter() {
            bytes.extend(item.to_bytes());
        }
        bytes.resize(self.padded_size(), 0);
        bytes
    }

    fn insert(&mut self, bytes: &[u8]) -> usize {
        let index = self.data.len() * 24;
        self.data.push(SymItem::from_bytes(bytes));
        index
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn link_and_info(&self) -> (Option<&'static str>, Option<&'static str>) {
        (Some(".strtab"), None)
    }

    fn padded_size(&self) -> usize {
        let data_len = (self.data.len() + 1) * 24;
        data_len + (16 - (data_len % 16))
    }

    fn size(&self) -> usize {
        self.data.len() * 24
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn header(&self, sh_name: u32, sh_offset: u64, sh_link: u32, _: u32) -> SectionHeader {
        SectionHeader {
            sh_name,
            sh_type: 2,
            sh_flags: 0,
            sh_addr: 0,
            sh_offset,
            sh_size: self.size() as u64,
            sh_link,
            sh_info: self.start_of_global as u32,
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
#[derive(Debug, Default, Clone, Copy)]
pub struct SymItem {
    pub st_name: u32,
    pub st_info: u8,
    pub st_other: u8,
    pub st_shndx: u16,
    pub st_value: u64,
    pub st_size: u64,
}
impl SymItem {
    pub fn to_bytes(self) -> IBytes {
        let mut bytes = vec![];
        bytes.extend(self.st_name.to_le_bytes());
        bytes.extend(self.st_info.to_le_bytes());
        bytes.extend(self.st_other.to_le_bytes());
        bytes.extend(self.st_shndx.to_le_bytes());
        bytes.extend(self.st_value.to_le_bytes());
        bytes.extend(self.st_size.to_le_bytes());
        bytes
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        Self {
            st_name: slice_to_u64(&bytes[0..4]) as u32,
            st_info: bytes[4],
            st_other: bytes[5],
            st_shndx: slice_to_u64(&bytes[6..8]) as u16,
            st_value: slice_to_u64(&bytes[8..16]),
            st_size: slice_to_u64(&bytes[16..24]),
        }
    }
}
// section RELA
//   This section holds relocation information as described
//   below.  If the file has a loadable segment that includes
//   relocation, the section's attributes will include the
//   SHF_ALLOC bit.  Otherwise, the bit will be off.  By
//   convention, "NAME" is supplied by the section to which the
//   relocations apply.  Thus a relocation section for .text
//   normally would have the name .rela.text.  This section is
//   of type SHT_RELA.
//   [Nr] Name              Type             Address           Offset
//        Size              EntSize          Flags  Link  Info  Align
//   [ 6] .rela.text        RELA             0000000000000000  00000440
//        0000000000000018  0000000000000018           4     1     8
#[allow(unused)]
#[derive(Debug, Clone)]
pub struct RELASec {
    pub name: String,
    pub data: Vec<RelaItem>,
}
impl RELASec {
    pub fn new(name: String) -> Self {
        Self {
            name,
            data: Vec::new(),
        }
    }
    pub fn from_bytes(sh: &SectionHeader, name: &str, bytes: &[u8]) -> Self {
        let mut rela = Self::new(name.to_string());
        let entries = sh.sh_size / sh.sh_entsize;
        for i in 0..entries {
            let index = (i * sh.sh_entsize) as usize;
            let item = RelaItem::from_bytes(name, &bytes[index..index + sh.sh_entsize as usize]);
            rela.data.push(item);
        }
        rela
    }
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    pub fn push(&mut self, item: RelaItem) {
        self.data.push(item);
    }
}
impl Section for RELASec {
    fn to_bytes(&self) -> IBytes {
        let mut bytes = vec![];
        for item in self.data.iter() {
            bytes.extend(item.to_bytes());
        }
        bytes.resize(self.padded_size(), 0);
        bytes
    }

    fn insert(&mut self, bytes: &[u8]) -> usize {
        let index = self.data.len() * 24;
        //TODO: FIX THIS
        self.data.push(RelaItem::from_bytes(&self.name, bytes));
        index
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn link_and_info(&self) -> (Option<&'static str>, Option<&'static str>) {
        (Some(".symtab"), Some(".text"))
    }

    fn padded_size(&self) -> usize {
        let data_len = self.data.len() * 24;
        data_len + (16 - (data_len % 16))
    }

    fn size(&self) -> usize {
        self.data.len() * 24
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn header(&self, sh_name: u32, sh_offset: u64, sh_link: u32, sh_info: u32) -> SectionHeader {
        SectionHeader {
            sh_name,
            // SHT_RELA 4
            sh_type: 4,
            sh_flags: 0,
            sh_addr: 0,
            sh_offset,
            sh_size: self.size() as u64,
            sh_link,
            sh_info,
            sh_addralign: 8,
            sh_entsize: 24,
        }
    }
}

#[derive(Debug, Clone)]
pub struct RelaItem {
    r_offset: u64,
    pub r_section: u32,
    r_platform: u32,
    r_addend: i64,
    pub sym_name: String,
    pub sym_type: SymbolType,
}
impl RelaItem {
    pub fn new(
        sym_name: impl ToString,
        sym_type: SymbolType,
        r_platform: u32,
        r_offset: u64,
        r_addend: i64,
    ) -> Self {
        Self {
            r_offset,
            r_section: 0,
            r_platform,
            r_addend,
            sym_name: sym_name.to_string(),
            sym_type,
        }
    }

    pub fn from_bytes(name: &str, bytes: &[u8]) -> Self {
        Self {
            r_offset: slice_to_u64(&bytes[0..8]),
            r_section: slice_to_u64(&bytes[8..12]) as u32,
            r_platform: slice_to_u64(&bytes[12..16]) as u32,
            r_addend: slice_to_u64(&bytes[16..24]) as i64,
            sym_name: name.to_string(),
            sym_type: SymbolType::Other,
        }
    }

    pub fn to_bytes(&self) -> IBytes {
        let mut bytes = vec![];
        let r_info = ((self.r_section as u64) << 32) | self.r_platform as u64;
        bytes.extend(self.r_offset.to_le_bytes());
        bytes.extend(r_info.to_le_bytes());
        bytes.extend(self.r_addend.to_le_bytes());
        bytes
    }
}

// Vector Slice to u64
pub fn slice_to_u64(bytes: &[u8]) -> u64 {
    match bytes.len() {
        1 => bytes[0] as u64,
        2 => {
            let ins_bytes = <[u8; 2]>::try_from(bytes).unwrap();
            u16::from_le_bytes(ins_bytes) as u64
        }
        4 => {
            let ins_bytes = <[u8; 4]>::try_from(bytes).unwrap();
            u32::from_le_bytes(ins_bytes) as u64
        }
        8 => {
            let ins_bytes = <[u8; 8]>::try_from(bytes).unwrap();
            u64::from_le_bytes(ins_bytes)
        }
        _ => unreachable!("Chunk to number"),
    }
}
