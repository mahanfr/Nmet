pub mod asm_parser;
pub mod assemble;
pub mod instructions;
pub mod memory;
pub mod mnemonic;
pub mod opcodes;
pub mod register;
use std::{collections::HashMap, fmt::Display};

use self::{
    instructions::{Instr, Opr},
    mnemonic::Mnemonic, assemble::assemble_instr,
};

type RelocatableInstr = (String, usize);

#[derive(Clone)]
struct InstrData {
    mnemonic: Mnemonic,
    // span
    is_compelete: bool,
    refrence: String,
    bytes: Vec<u8>,
    display: String,
}

impl InstrData {
    pub fn new(instr: Instr) -> Self {
        Self {
            mnemonic: instr.mnem,
            is_compelete: true,
            refrence: String::new(),
            bytes: assemble_instr(&instr),
            display: instr.to_string(),
        }
    }

    pub fn new_lable(display: String) -> Self {
        Self {
            mnemonic: Mnemonic::Lable,
            is_compelete: true,
            refrence: String::new(),
            bytes: Vec::new(),
            display,
        }
    }

    pub fn new_ref(mnemonic: Mnemonic, display: String, refrence: String) -> Self {
        Self {
            mnemonic,
            is_compelete: false,
            refrence,
            bytes: Vec::new(),
            display,
        }
    }
}

#[allow(dead_code)]
#[derive(Clone)]
pub struct Codegen {
    instructs: Vec<InstrData>,
    pub data_buf: Vec<String>,
    pub bss_buf: Vec<String>,
    pub lables_map: HashMap<String, usize>,
    unknown_ref: HashMap<String, Vec<RelocatableInstr>>,
    last_lable: String,
}

impl Codegen {
    pub fn new() -> Self {
        Self {
            instructs: Vec::new(),
            bss_buf: Vec::new(),
            data_buf: Vec::new(),
            unknown_ref: HashMap::new(),
            lables_map: HashMap::new(),
            last_lable: String::new(),
        }
    }

    pub fn get_id(&self) -> usize {
        self.instructs.len()
    }

    pub fn text_section_asm(&self) -> String {
        let mut asm = String::new();
        for item in self.instructs.iter() {
            if item.mnemonic == Mnemonic::Lable {
                asm.push_str(item.display.as_str());
            } else {
                asm.push_str("    ");
                asm.push_str(item.display.as_str());
            }
            asm.push('\n');
        }
        asm
    }

    pub fn add_data_seg(&mut self, data: impl ToString, _size: usize) -> u64 {
        let id = self.data_buf.len();
        self.data_buf
            .push(format!("data{id} db {}", data.to_string()));
        self.data_buf.push(format!("len{id} equ $ - data{id}"));
        id as u64
    }

    pub fn add_bss_seg(&mut self, size: usize) -> String {
        let bss_tag = format!("arr{}", self.bss_buf.len());
        self.bss_buf.push(format!("{}: resb {}", bss_tag, size));
        bss_tag
    }

    fn __relocatable_instr(&mut self, lable: String, mnmemonic: Mnemonic) {
        let key = match lable.starts_with('.') {
            true => format!("{}{lable}", self.last_lable),
            false => lable.clone(),
        };
        self.instructs.push(InstrData::new_ref(
            mnmemonic,
            format!("{mnmemonic} {lable}"),
            key,
        ));
    }

    pub fn jmp(&mut self, lable: impl Display) {
        self.__relocatable_instr(lable.to_string(), Mnemonic::Jmp);
    }

    pub fn jz(&mut self, lable: impl Display) {
        self.__relocatable_instr(lable.to_string(), Mnemonic::Jz);
    }

    pub fn jne(&mut self, lable: impl Display) {
        self.__relocatable_instr(lable.to_string(), Mnemonic::Jne);
    }

    pub fn call(&mut self, lable: impl Display) {
        self.__relocatable_instr(lable.to_string(), Mnemonic::Call);
    }

    pub fn asm_push(&mut self, lable: impl Display) {
        let lable = lable.to_string();
        self.instructs.push(InstrData::new_ref(
            Mnemonic::Push,
            format!("push {lable}"),
            lable,
        ))
    }

    pub fn asm_mov(&mut self, opr: impl Into<Opr>, lable: impl Display) {
        let lable = lable.to_string();
        self.instructs.push(InstrData::new_ref(
            Mnemonic::Push,
            format!("mov {}, {lable}", opr.into()),
            lable,
        ))
    }

    pub fn push_instr(&mut self, instr: Instr) {
        self.instructs.push(InstrData::new(instr));
    }

    pub fn set_lable(&mut self, lable: impl Display) {
        let lable = lable.to_string();
        self.instructs
            .push(InstrData::new_lable(format!("{lable}:")));
        let key = match lable.starts_with('.') {
            true => format!("{}{lable}", self.last_lable),
            false => lable,
        };
        self.lables_map.insert(key, self.instructs.len());
    }
}
