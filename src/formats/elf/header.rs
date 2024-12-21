use super::sections::slice_to_u64;

#[allow(unused)]
#[derive(Debug, Clone, Copy)]
#[repr(u16)]
pub enum EType {
    None = 0,
    Rel = 1,
    Exec = 2,
    Dyn = 3,
    Core = 4,
    Loos = 0xFE00,
    Hios = 0xFEFF,
    Loproc = 0xFF00,
    Hiproc = 0xFFFF,
}

impl EType {
    pub fn from(value: u64) -> Self {
        match value {
            0 => Self::None,
            1 => Self::Rel,
            2 => Self::Exec,
            3 => Self::Dyn,
            4 => Self::Core,
            0xFE00 => Self::Loos,
            0xFEFF => Self::Hios,
            0xFF00 => Self::Loproc,
            0xFFFF => Self::Hiproc,
            _ => unreachable!("ETYPE: {value:X}"),
        }
    }
}

#[allow(unused)]
#[repr(u16)]
#[derive(Debug, Clone, Copy)]
pub enum EMachine {
    X86_64 = 0x3E,
    Arm = 0x28,
}

impl EMachine {
    pub fn from(value: u64) -> Self {
        match value {
            0x3E => Self::X86_64,
            0x28 => Self::Arm,
            _ => unreachable!("EMachine: {value:X}"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ElfHeader {
    pub e_ident: [u8; 16],
    pub e_type: EType,
    pub e_machine: EMachine,
    pub e_version: u32,
    pub e_entry: u64,
    pub e_phoff: u64,
    pub e_shoff: u64,
    pub e_flags: u32,
    pub e_ehsize: u16,
    pub e_phensize: u16,
    pub e_phnum: u16,
    pub e_shentsize: u16,
    pub e_shnum: u16,
    pub e_shstrndx: u16,
}

impl ElfHeader {
    pub fn new(e_shnum: u16, e_shstrndx: u16) -> Self {
        Self {
            e_ident: *b"\x7fELF\x02\x01\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00",
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

    pub fn from_bytes(bytes: &[u8]) -> Self {
        if bytes[0..4] != [0x7F, 0x45, 0x4C, 0x46] {
            panic!("File is not an valid elf file!");
        }
        Self {
            e_ident: bytes[0..16].try_into().expect("Cannot parse elf e_ident"),
            e_type: EType::from(slice_to_u64(&bytes[16..18])),
            e_machine: EMachine::from(slice_to_u64(&bytes[18..20])),
            e_version: slice_to_u64(&bytes[20..24]) as u32,
            e_entry: slice_to_u64(&bytes[24..32]),
            e_phoff: slice_to_u64(&bytes[32..40]),
            e_shoff: slice_to_u64(&bytes[40..48]),
            e_flags: slice_to_u64(&bytes[48..52]) as u32,
            e_ehsize: slice_to_u64(&bytes[52..54]) as u16,
            e_phensize: slice_to_u64(&bytes[54..56]) as u16,
            e_phnum: slice_to_u64(&bytes[56..58]) as u16,
            e_shentsize: slice_to_u64(&bytes[58..60]) as u16,
            e_shnum: slice_to_u64(&bytes[60..62]) as u16,
            e_shstrndx: slice_to_u64(&bytes[62..64]) as u16,
        }
    }

    pub fn to_bytes(self) -> Vec<u8> {
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
