pub mod asm_parser;
pub mod assemble;
pub mod data_bss;
pub mod instructions;
pub mod memory;
pub mod mnemonic;
pub mod opcodes;
pub mod register;
pub mod text;
pub mod utils;
use std::{collections::BTreeMap, fmt::Display};

use crate::{
    formats::elf::{sections::RelaItem, SymbolType},
    parser::types::VariableType,
    utils::IBytes,
};

use self::{
    assemble::assemble_instr,
    data_bss::{BssItem, DataItem},
    instructions::{Instr, Opr, Oprs},
    mnemonic::Mnemonic,
};

pub fn placeholder(instr: Instr) -> Instr {
    match instr.oprs {
        Oprs::One(Opr::Rela(_) | Opr::Loc(_)) => Instr::new1(instr.mnem, Opr::Imm32(0)),
        Oprs::Two(x, Opr::Rela(_) | Opr::Loc(_)) => Instr::new2(instr.mnem, x, Opr::Imm32(0)),
        Oprs::Two(Opr::Rela(_) | Opr::Loc(_), x) => Instr::new2(instr.mnem, Opr::Imm32(0), x),
        _ => unreachable!(),
    }
}
#[derive(Clone)]
struct InstrData {
    instr: Instr,
    // span
    bytes: IBytes,
}

impl InstrData {
    pub fn new(instr: Instr) -> Self {
        let bytes =
            match (instr.needs_rela_map() || instr.needs_location()) && !instr.uses_rela_memory() {
                true => assemble_instr(&placeholder(instr.clone())),
                false => assemble_instr(&instr),
            };
        Self { instr, bytes }
    }

    pub fn new_lable(display: String) -> Self {
        Self {
            instr: Instr::new1(Mnemonic::Lable, Opr::Loc(display)),
            bytes: Vec::new(),
        }
    }
}
#[allow(dead_code)]
#[derive(Clone)]
pub struct Codegen {
    instructs: Vec<InstrData>,
    pub data_buf: BTreeMap<String, DataItem>,
    pub bss_buf: Vec<BssItem>,
    pub symbols_map: BTreeMap<String, (usize, SymbolType)>,
    //pub ffi_map: BTreeMap<String, String>,
    pub rela_map: Vec<RelaItem>,
}

impl Codegen {
    pub fn new() -> Self {
        Self {
            instructs: Vec::new(),
            bss_buf: Vec::new(),
            data_buf: BTreeMap::new(),
            symbols_map: BTreeMap::new(),
            rela_map: Vec::new(),
        }
    }

    #[allow(dead_code)]
    pub fn get_raw_instructs(&self) -> Vec<Instr> {
        self.instructs.iter().map(|x| x.instr.clone()).collect()
    }

    pub fn relocate(&mut self) {
        let mut bytes_sum = 0;
        for item in self.instructs.iter_mut() {
            if item.instr.needs_rela_map() {
                let Some(key) = item.instr.get_rela_key() else {
                    unreachable!();
                };
                let rela_offset = item
                    .bytes
                    .windows(4)
                    .position(|x| x == [0, 0, 0, 0])
                    .unwrap()
                    + bytes_sum;
                match self.symbols_map.get(&key).unwrap() {
                    (_, SymbolType::BssSec) => {
                        let addend = self.bss_buf.iter().find(|x| x.name == key).unwrap().index;
                        self.rela_map.push(RelaItem::new(
                            ".bss",
                            SymbolType::BssSec,
                            0xb,
                            rela_offset as u64,
                            addend as i64,
                        ));
                    }
                    (_, SymbolType::DataSec) => {
                        let addend = self
                            .data_buf
                            .values()
                            .find(|x| x.name == key)
                            .unwrap()
                            .index;
                        self.rela_map.push(RelaItem::new(
                            ".data",
                            SymbolType::DataSec,
                            0xb,
                            rela_offset as u64,
                            addend as i64,
                        ));
                    }
                    (_, SymbolType::Ffi) => {
                        self.rela_map.push(RelaItem::new(
                            key,
                            SymbolType::Ffi,
                            0x2,
                            rela_offset as u64,
                            -4,
                        ));
                    }
                    _ => unreachable!("{:?}", item.instr),
                }
                bytes_sum += item.bytes.len();
            } else if item.instr.needs_location() {
                let Oprs::One(Opr::Loc(key)) = item.instr.oprs.clone() else {
                    unreachable!("{:?}", item.instr);
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
                let Oprs::One(Opr::Loc(tag)) = item.instr.oprs.clone() else {
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
        let index = match self.data_buf.values().last() {
            Some(dt) => dt.index + dt.data.len(),
            None => 0,
        };
        self.symbols_map.insert(
            name.clone(),
            (
                self.data_buf.values().map(|x| x.data.len()).sum(),
                SymbolType::DataSec,
            ),
        );
        self.data_buf.insert(
            name.clone(),
            DataItem::new(name.clone(), index, data, dtype),
        );
        name
    }

    pub fn add_bss_seg(&mut self, size: usize) -> String {
        let bss_tag = format!("arr{}", self.bss_buf.len());
        let index = match self.bss_buf.last() {
            Some(dt) => dt.index + dt.size,
            None => 0,
        };
        self.symbols_map.insert(
            bss_tag.clone(),
            (
                self.bss_buf.iter().map(|x| x.size).sum(),
                SymbolType::BssSec,
            ),
        );
        self.bss_buf
            .push(BssItem::new(bss_tag.clone(), index, size));
        bss_tag
    }

    pub fn instr2(&mut self, mnemonic: Mnemonic, opr1: impl Into<Opr>, opr2: impl Into<Opr>) {
        self.instructs
            .push(InstrData::new(Instr::new2(mnemonic, opr1, opr2)));
    }

    pub fn instr1(&mut self, mnemonic: Mnemonic, opr1: impl Into<Opr>) {
        let opr1 = opr1.into();
        if let (Mnemonic::Call, Opr::Rela(r)) = (&mnemonic, &opr1) {
            self.symbols_map.insert(r.clone(), (0, SymbolType::Ffi));
        }
        self.instructs
            .push(InstrData::new(Instr::new1(mnemonic, opr1)));
    }

    pub fn instr0(&mut self, mnemonic: Mnemonic) {
        self.instructs.push(InstrData::new(Instr::new0(mnemonic)));
    }

    pub fn new_instr(&mut self, instr: Instr) {
        self.instructs.push(InstrData::new(instr));
    }

    pub fn set_lable(&mut self, lable: impl Display) {
        let lable = lable.to_string();
        self.instructs.push(InstrData::new_lable(lable.clone()));
        let real_loc: usize = self.instructs.iter().map(|x| x.bytes.len()).sum();
        self.symbols_map
            .insert(lable, (real_loc, SymbolType::TextSec));
    }
}
