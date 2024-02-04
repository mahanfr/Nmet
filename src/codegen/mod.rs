pub mod asm_parser;
pub mod assemble;
pub mod elf;
pub mod instructions;
pub mod memory;
pub mod mnemonic;
pub mod opcodes;
pub mod register;
pub mod text;
pub mod data_bss;
use std::{collections::{HashMap, BTreeMap}, fmt::Display};

use crate::{utils::IBytes, parser::types::VariableType};

use self::{
    assemble::assemble_instr,
    instructions::{Instr, Opr, Oprs},
    mnemonic::Mnemonic, data_bss::DataItem,
};

pub fn placeholder(instr: Instr) -> Instr {
    match instr.oprs {
        Oprs::One(Opr::Fs(_)) => Instr::new1(instr.mnem, Opr::Imm32(0)),
        Oprs::Two(x, Opr::Fs(_)) => Instr::new2(instr.mnem, x, Opr::Imm32(0)),
        Oprs::Two(Opr::Fs(_), x) => Instr::new2(instr.mnem, Opr::Imm32(0), x),
        _ => unreachable!(),
    }
}

#[derive(Clone, PartialEq)]
enum Relocatable {
    None,
    Ref,
    Loc,
}

#[derive(Debug, Clone)]
pub struct RelaItem {
    r_offset: u64,
    r_info: u64,
    r_addend: i64,
    sec_name: String,
}
impl RelaItem {
    pub fn new(sec_name:impl ToString, r_offset: u64, r_addend: i64) -> Self {
        Self {
            r_offset,
            r_info: 0x020000000b,
            r_addend,
            sec_name: sec_name.to_string(),
        }
    }

    pub fn set_info(&mut self, r_type: u64, r_sym: u64) {
        self.r_info = (r_type << 32) | (r_sym & u32::MAX as u64);
    }

    pub fn to_bytes(&self) -> IBytes {
        let mut bytes = vec![];
        bytes.extend(self.r_offset.to_le_bytes());
        bytes.extend(self.r_info.to_le_bytes());
        bytes.extend(self.r_addend.to_le_bytes());
        bytes
    }
}

#[derive(Clone)]
struct InstrData {
    instr: Instr,
    // span
    relocatable: Relocatable,
    bytes: IBytes,
}

impl InstrData {
    pub fn new(instr: Instr, rel: Relocatable) -> Self {
        let bytes = match rel {
            Relocatable::Loc => assemble_instr(&Instr::new1(instr.mnem, Opr::Imm32(0))),
            Relocatable::Ref => assemble_instr(&placeholder(instr.clone())),
            Relocatable::None => assemble_instr(&instr),
        };
        Self {
            instr,
            bytes,
            relocatable: rel,
        }
    }

    pub fn new_lable(display: String) -> Self {
        Self {
            instr: Instr::new1(Mnemonic::Lable, Opr::Rel(display)),
            relocatable: Relocatable::None,
            bytes: Vec::new(),
        }
    }
}

#[allow(unused)]
#[derive(Debug, Clone, Copy)]
pub enum SymbolType {
    Global,
    DataSec,
    TextSec,
}

#[allow(dead_code)]
#[derive(Clone)]
pub struct Codegen {
    instructs: Vec<InstrData>,
    pub data_buf: Vec<DataItem>,
    pub bss_buf: Vec<String>,
    pub symbols_map: BTreeMap<String, (usize, SymbolType)>,
    pub rela_map: Vec<RelaItem>,
}

impl Codegen {
    pub fn new() -> Self {
        Self {
            instructs: Vec::new(),
            bss_buf: Vec::new(),
            data_buf: Vec::new(),
            symbols_map: BTreeMap::new(),
            rela_map: Vec::new(),
        }
    }

    pub fn get_id(&self) -> usize {
        self.instructs.len()
    }

    pub fn relocate(&mut self) {
        let mut bytes_sum = 0;
        for item in self.instructs.iter_mut() {
            if item.relocatable == Relocatable::Ref {
                let Oprs::One(Opr::Fs(key)) = item.instr.oprs.clone() else {
                    unreachable!();
                };
                let rela_offset = item.bytes.windows(4).position(|x| x == [0,0,0,0]).unwrap()
                    + bytes_sum;
                let addend = self.data_buf.iter().find(|x| x.name == key).unwrap().index;
                self.rela_map.push(RelaItem::new(".data",rela_offset as u64, addend as i64));
                bytes_sum += item.bytes.len();
            } else if item.relocatable == Relocatable::Loc {
                let Oprs::One(Opr::Rel(key)) = item.instr.oprs.clone() else {
                    unreachable!();
                };
                let Some(target) = self.symbols_map.get(&key) else {
                    panic!("Unknown Target!");
                };
                let loc: i32 = target.0 as i32 - bytes_sum as i32 - item.bytes.len() as i32;
                let new_bytes =
                    assemble_instr(&Instr::new1(item.instr.mnem, Opr::Imm32(loc as i64)));
                bytes_sum += new_bytes.len();
                item.bytes = new_bytes;
            } else {
                bytes_sum += item.bytes.len();
            }
        }
    }

    pub fn text_section_bytes(&mut self) -> IBytes {
        self.relocate();
        let mut bytes = Vec::new();
        let mut _bc = 0;
        for item in self.instructs.iter() {
            // println!(
            //     "\x1b[92m{_bc:3X}\x1b[0m: {:02X?} \x1b[93m{}\x1b[0m",
            //     item.bytes, item.instr
            // );
            _bc += item.bytes.len();
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

    pub fn add_data(&mut self, data: Vec<u8>, dtype: VariableType) -> String {
        let name = format!("data{}", self.data_buf.len());
        let index = match self.data_buf.last() {
            Some(dt) => {
                dt.index + dt.data.len()
            }
            None => 0,
        };
        self.symbols_map.insert(name.clone(), 
                    (self.data_buf.iter().map(|x| x.data.len()).sum(), SymbolType::DataSec));
        self.data_buf.push(DataItem::new(name.clone(), index, data, dtype));
        name
    }

    pub fn add_bss_seg(&mut self, size: usize) -> String {
        let bss_tag = format!("arr{}", self.bss_buf.len());
        self.bss_buf.push(format!("{}: resb {}", bss_tag, size));
        bss_tag
    }

    pub fn instr2(&mut self, mnemonic: Mnemonic, opr1: impl Into<Opr>, opr2: impl Into<Opr>) {
        let (opr1, rel1) = self.relocate_lable(opr1);
        let (opr2, rel2) = self.relocate_lable(opr2);
        let rel = match (rel1, rel2) {
            (Relocatable::Ref, _) | (_, Relocatable::Ref) => Relocatable::Ref,
            (Relocatable::Loc, _) | (_, Relocatable::Loc) => Relocatable::Loc,
            _ => Relocatable::None,
        };
        self.instructs
            .push(InstrData::new(Instr::new2(mnemonic, opr1, opr2), rel));
    }

    pub fn instr1(&mut self, mnemonic: Mnemonic, opr1: impl Into<Opr>) {
        let (opr1, rel1) = self.relocate_lable(opr1);
        self.instructs
            .push(InstrData::new(Instr::new1(mnemonic, opr1), rel1));
    }

    pub fn instr0(&mut self, mnemonic: Mnemonic) {
        self.instructs
            .push(InstrData::new(Instr::new0(mnemonic), Relocatable::None));
    }

    pub fn new_instr(&mut self, instr: Instr) {
        self.instructs
            .push(InstrData::new(instr, Relocatable::None));
    }

    pub fn set_lable(&mut self, lable: impl Display) {
        let lable = lable.to_string();
        self.instructs.push(InstrData::new_lable(lable.clone()));
        let real_loc: usize = self.instructs.iter().map(|x| x.bytes.len()).sum();
        self.symbols_map.insert(lable, (real_loc, SymbolType::TextSec));
    }

    fn relocate_lable(&mut self, opr1: impl Into<Opr>) -> (Opr, Relocatable) {
        let opr = opr1.into();
        match opr.to_owned() {
            Opr::Rel(_) => (opr, Relocatable::Loc),
            Opr::Fs(_) => (opr, Relocatable::Ref),
            _ => (opr, Relocatable::None),
        }
    }
}
