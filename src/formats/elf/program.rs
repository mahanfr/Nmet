use crate::utils::IBytes;


#[derive(Debug, Clone)]
pub struct ProgramHeader {
    p_type: u32,
    p_flags: u32,
    p_offset: u64,
    p_vaddr: u64,
    p_addr: u64,
    p_filesz: u64,
    p_memsz: u64,
    p_align: u64,
}

impl ProgramHeader {
    pub fn new_default(
        p_type: u32,
        offset: u64,
        flags: u32,
        addr: u64,
        size: u64,
        ) -> Self {
        Self {
            p_type,
            p_flags: flags,
            p_offset: offset,
            p_vaddr: addr,
            p_addr: addr,
            p_filesz: size,
            p_memsz: size,
            p_align: 0x1000,
        }
    }

    pub fn to_bytes(&self) -> IBytes {
        let mut bytes = vec![];
        bytes.extend(self.p_type.to_le_bytes());
        bytes.extend(self.p_flags.to_le_bytes());
        bytes.extend(self.p_offset.to_le_bytes());
        bytes.extend(self.p_vaddr.to_le_bytes());
        bytes.extend(self.p_addr.to_le_bytes());
        bytes.extend(self.p_filesz.to_le_bytes());
        bytes.extend(self.p_memsz.to_le_bytes());
        bytes.extend(self.p_align.to_le_bytes());
        bytes
    }
}
