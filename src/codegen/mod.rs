pub mod asm_parser;
pub mod instructions;
pub mod mnemonic;
pub mod register;
pub mod memory;
pub mod opcodes;
use std::{fmt::Display, collections::HashMap};

use self::{instructions::{Instr, Opr}, mnemonic::Mnemonic};

type RelocatableInstr = (String, usize);

#[allow(dead_code)]
#[derive(Clone)]
pub struct Codegen {
    instruct_buf: Vec<Instr>,
    instruct_bytes: Vec<u8>,
    pub instruct_asm: String,
    pub data_buf: Vec<String>,
    pub bss_buf: Vec<String>,
    pub lables_map: HashMap<String, usize>,
    unknown_ref: HashMap<String, Vec<RelocatableInstr>>,
    last_lable: String,
}

impl Codegen {
    pub fn new() -> Self {
        Self {
            instruct_buf: Vec::new(),
            instruct_bytes: Vec::new(),
            instruct_asm: String::new(),
            bss_buf: Vec::new(),
            data_buf: Vec::new(),
            unknown_ref: HashMap::new(),
            lables_map: HashMap::new(),
            last_lable: String::new(),
        }
    }

    pub fn get_id(&self) -> usize {
        self.instruct_asm.len()
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
        self.instruct_asm.push_str(format!("    {mnmemonic} {lable}\n").as_str());
        let key = match lable.starts_with('.') {
            true => format!("{}{lable}",self.last_lable),
            false => lable,
        };
       match self.lables_map.get(&key) {
            Some(loc) => {
                let opr: Opr = (*loc).into();
                self.instruct_buf.push(Instr::new1(mnmemonic, opr));
            }
            None => {
                self.instruct_buf.push(Instr::new0(Mnemonic::Nop));
                let this_loc = self.instruct_buf.len() - 1;
                self.unknown_ref
                    .entry(key)
                    .or_insert(Vec::new())
                    .push((mnmemonic.to_string(), this_loc));
            }
       }
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
        self.instruct_asm.push_str(format!("    push {lable}\n").as_str());
    }

    pub fn asm_mov(&mut self, opr: impl Into<Opr>, lable: impl Display) {
        self.instruct_asm.push_str(format!("    mov {}, {lable}\n",opr.into()).as_str());
    }

    pub fn push_instr(&mut self, instr: Instr) {
        self.instruct_asm.push_str(format!("    {instr}\n").as_str());
        self.instruct_buf.push(instr);
    }

    pub fn set_lable(&mut self, lable: impl Display) {
        let lable = lable.to_string();
        self.instruct_asm.push_str(format!("{lable}:\n").as_str());
        // let key = match lable.starts_with('.') {
        //     true => format!("{}{lable}",self.last_lable),
        //     false => lable,
        // };
        // if let Some(rel_items) = self.unknown_ref.remove(&key) {
        //     for item in rel_items.iter() {
        //         self.instruct_buf[item.1] = Instr::new(&item.0, vec![])
        //     }
        // }
        // self.lables_map.insert(key, self.instruct_buf.len());
    }
}
