#[allow(unused)]
#[derive(Debug, Clone, Copy)]
enum EType {
    None = 0,
    Rel = 1,
    Exec = 2,
    Dyn = 3,
    Core = 4
}

#[allow(unused)]
#[repr(u16)]
#[derive(Debug, Clone, Copy)]
enum EMachine {
    X86_64 = 62,
    ARM = 40,
}

pub struct ElfHeader {
    e_type: EType,
    e_machine: EMachine,
    e_version: u32,
    e_entry: u64,
    e_phoff: u64,
    e_shoff: u64,
    e_flags: u32,
    e_ehsize: u64,
    e_phensize: u16,
    e_phnum: u16,
    e_shentsize: u16,
    e_shnum: u16,
    e_shstrndx: u16,
}

impl ElfHeader {
    pub fn new(e_shnum: u16, e_shstrndx: u16) -> Self {
        Self {
            e_type: EType::Rel,
            e_machine: EMachine::X86_64,
            e_version: 1,
            e_entry: 0,
            e_phoff: 0,
            e_shoff: 64,
            e_flags: 0,
            e_ehsize: 64,
            e_phensize: 0,
            e_phnum: 0,
            e_shentsize: 64,
            e_shnum,
            e_shstrndx,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend(b"\x7fELF\x02\x01\x01");
        bytes.resize(16, 0);
        bytes.extend((self.e_type as u16).to_le_bytes());
        bytes.extend((self.e_machine as u16).to_le_bytes());
        bytes.extend(self.e_version.to_le_bytes());
        bytes.extend(self.e_entry.to_le_bytes());
        bytes.extend(self.e_phoff.to_le_bytes());
        bytes.extend(self.e_shoff.to_le_bytes());
        bytes.extend(self.e_flags.to_le_bytes());
        bytes.extend(self.e_ehsize.to_le_bytes());
        bytes.extend(self.e_phensize.to_le_bytes());
        bytes.extend(self.e_phnum.to_le_bytes());
        bytes.extend(self.e_shentsize.to_le_bytes());
        bytes.extend(self.e_shnum.to_le_bytes());
        bytes.extend(self.e_shstrndx.to_le_bytes());
        bytes
    }
}

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

pub trait Section {
    fn header(&self, sh_name: u32, sh_offset: u64) -> SectionHeader;
}

pub struct TextSection {
    data: Vec<u8>,
}
impl Section for TextSection {
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


pub struct ElfSections {
    sections: Vec<Box<dyn Section>>,
}
