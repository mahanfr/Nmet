pub mod asm_parser;
pub mod assemble;
pub mod instructions;
pub mod memory;
pub mod mnemonic;
pub mod opcodes;
pub mod register;
use std::{collections::HashMap, fmt::Display};

use self::{
    assemble::assemble_instr,
    instructions::{Instr, Opr, Oprs},
    mnemonic::Mnemonic,
};

#[derive(Clone, PartialEq)]
enum Relocatable {
    None,
    Ref,
    Loc,
}

#[derive(Clone)]
struct InstrData {
    instr: Instr,
    // span
    relocatable: Relocatable,
    bytes: Vec<u8>,
}

impl InstrData {
    pub fn new(instr: Instr, rel: Relocatable) -> Self {
        let bytes = match rel {
            Relocatable::Loc => assemble_instr(&Instr::new1(instr.mnem, Opr::Imm32(0))),
            Relocatable::Ref => Vec::new(),
            Relocatable::None => assemble_instr(&instr),
        };
        Self { instr, bytes, relocatable: rel}
    }

    pub fn new_lable(display: String) -> Self {
        Self {
            instr: Instr::new1(Mnemonic::Lable, Opr::Rel(display)),
            relocatable: Relocatable::None,
            bytes: Vec::new(),
        }
    }
}

#[allow(dead_code)]
#[derive(Clone)]
pub struct Codegen {
    instructs: Vec<InstrData>,
    pub data_buf: Vec<String>,
    pub bss_buf: Vec<String>,
    pub rel_map: HashMap<String, Option<usize>>,
    last_lable: String,
}

impl Codegen {
    pub fn new() -> Self {
        Self {
            instructs: Vec::new(),
            bss_buf: Vec::new(),
            data_buf: Vec::new(),
            rel_map: HashMap::new(),
            last_lable: String::new(),
        }
    }

    pub fn get_id(&self) -> usize {
        self.instructs.len()
    }

    pub fn relocate(&mut self) {
        let mut bytes_sum = 0;
        for item in self.instructs.iter_mut() {
            if item.instr.mnem == Mnemonic::Lable {
                let Oprs::One(Opr::Rel(key)) = item.instr.oprs.clone() else {
                    unreachable!();
                };
                self.rel_map.entry(key).and_modify(|x| {*x = Some(bytes_sum)});
                continue;
            }
            if item.relocatable == Relocatable::Loc {
                let Oprs::One(Opr::Rel(key)) = item.instr.oprs.clone() else {
                    unreachable!();
                };
                let Some(Some(target)) = self.rel_map.get(&key) else {
                    panic!("Unknown Target!");
                };
                let loc: i32 = *target as i32 - bytes_sum as i32;
                let new_bytes = assemble_instr(&Instr::new1(item.instr.mnem, loc));
                bytes_sum += new_bytes.len();
                item.bytes = new_bytes;
            } else {
                bytes_sum += item.bytes.len();
            }
        }
    }

    pub fn text_section_bytes(&mut self) -> Vec<u8> {
        self.relocate();
        let mut bytes = Vec::new();
        let mut bc = 0;
        for item in self.instructs.iter() {
            print!("\x1b[92m{bc:3X}\x1b[0m: {:02X?} \x1b[93m{}\x1b[0m\n",item.bytes, item.instr);
            bc += item.bytes.len();
            bytes.extend(item.bytes.clone());
        }
        bytes
    }

    pub fn text_section_asm(&self) -> String {
        let mut asm = String::new();
        for item in self.instructs.iter() {
            if item.instr.mnem == Mnemonic::Lable {
                let Oprs::One(Opr::Rel(tag)) = item.instr.oprs.clone() else {
                    panic!("Unknown lable instr {}", item.instr);
                };
                asm.push_str(format!("{tag}:").as_str());
            } else {
                asm.push_str("    ");
                asm.push_str(&item.instr.to_string());
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

    pub fn instr2(&mut self, mnemonic: Mnemonic, opr1: impl Into<Opr> , opr2: impl Into<Opr>) {
        let (opr1, rel1) = self.relocate_lable(opr1);
        let (opr2, rel2) = self.relocate_lable(opr2);
        let rel = match (rel1, rel2) {
            (Relocatable::Ref, _) | (_, Relocatable::Ref) => Relocatable::Ref,
            (Relocatable::Loc, _) | (_, Relocatable::Loc) => Relocatable::Loc,
            _ => Relocatable::None,
        };
        self.instructs.push(InstrData::new(Instr::new2(mnemonic, opr1, opr2),rel));
    }

    pub fn instr1(&mut self, mnemonic: Mnemonic, opr1: impl Into<Opr>) {
        let (opr1, rel1) = self.relocate_lable(opr1);
        self.instructs.push(InstrData::new(Instr::new1(mnemonic, opr1), rel1));
    }

    pub fn instr0(&mut self, mnemonic: Mnemonic) {
        self.instructs.push(InstrData::new(Instr::new0(mnemonic), Relocatable::None));
    }

    pub fn new_instr(&mut self, instr: Instr) {
        self.instructs.push(InstrData::new(instr,Relocatable::None));
    }

    pub fn set_lable(&mut self, lable: impl Display) {
        let lable = lable.to_string();
        self.instructs
            .push(InstrData::new_lable(format!("{lable}")));
        let key = match lable.starts_with('.') {
            true => format!("{}{lable}", self.last_lable),
            false => lable,
        };
        let real_loc : usize = self.instructs.iter().map(|x| x.bytes.len()).sum();
        self.rel_map.entry(key)
            .and_modify(|x| {*x = Some(self.instructs.len() - 1)} )
            .or_insert(Some(real_loc));
    }

    fn relocate_lable(&mut self, opr1: impl Into<Opr>) -> (Opr, Relocatable) {
        let opr = opr1.into();
        match opr.to_owned() {
            Opr::Rel(label) => {
                if label.starts_with('.') {
                    let key = format!("{}{label}", self.last_lable);
                    self.rel_map.entry(key.clone()).or_insert(None);
                    (Opr::Rel(key), Relocatable::Loc)
                } else {
                    self.rel_map.entry(label).or_insert(None);
                    (opr, Relocatable::Loc)
                }
            }
            Opr::Fs(_) => (opr, Relocatable::Ref),
            _ => (opr, Relocatable::None)
        }
    }

}
