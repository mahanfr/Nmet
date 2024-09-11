use std::time::{SystemTime, UNIX_EPOCH};

pub struct Header {
    machine: u16,
    number_of_sections: u16,
    time_date_stamp: u32,
    pointer_to_symtable: u32,
    number_of_symbols: u32,
    size_of_optional_header: u16,
    characteristics: u16,
}

impl Header {
    pub fn new(num_sections: u16, symtab_pointer: u32, sym_entries: u32) -> Self {
        Self {
            machine: 0x8664,
            number_of_sections: num_sections,
            time_date_stamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as u32,
            pointer_to_symtable: symtab_pointer,
            number_of_symbols: sym_entries,
            size_of_optional_header: 0,
            characteristics: 0x0004,

        }
    }

    pub fn to_bytes(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend(self.machine.to_le_bytes());
        bytes.extend(self.number_of_sections.to_le_bytes());
        bytes.extend(self.time_date_stamp.to_le_bytes());
        bytes.extend(self.pointer_to_symtable.to_le_bytes());
        bytes.extend(self.number_of_symbols.to_le_bytes());
        bytes.extend(self.number_of_symbols.to_le_bytes());
        bytes.extend(self.size_of_optional_header.to_le_bytes());
        bytes.extend(self.characteristics.to_le_bytes());
        bytes
    }
}
