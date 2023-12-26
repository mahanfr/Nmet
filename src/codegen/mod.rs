pub mod asm_parser;
pub mod instructions;
pub mod mnmemonic;
pub mod register;

use std::fmt::Display;

use self::{
    instructions::{Instr, Opr},
    mnmemonic::Mnemonic,
};

#[allow(dead_code)]
#[derive(Clone)]
pub struct Codegen {
    pub instruct_buf: Vec<Instr>,
    pub data_buf: Vec<String>,
    pub bss_buf: Vec<String>,
    pub last_lable: String,
}

#[allow(dead_code)]
impl Codegen {
    pub fn new() -> Self {
        Self {
            instruct_buf: Vec::new(),
            bss_buf: Vec::new(),
            data_buf: Vec::new(),
            last_lable: String::new(),
        }
    }

    pub fn get_id(&mut self) -> usize {
        self.instruct_buf.len()
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

    pub fn place_holder(&mut self) -> usize {
        self.instruct_buf.push(Instr::Nop);
        self.instruct_buf.len() - 1
    }

    pub fn insert_raw(&mut self, instr: Instr) {
        self.instruct_buf.push(instr);
    }

    pub fn replace(&mut self, index: usize, instr: Instr) -> Result<(), String> {
        if index < self.instruct_buf.len() - 1 {
            self.instruct_buf[index] = instr;
            Ok(())
        } else {
            Err("index out of bounds!".into())
        }
    }

    pub fn insert_into_raw(&mut self, index: usize, instr: Instr) -> Result<(), String> {
        if index < self.instruct_buf.len() - 1 {
            self.instruct_buf[index] = instr;
            Ok(())
        } else {
            Err("index out of bounds!".into())
        }
    }

    pub fn instr0(&mut self, mnem: Mnemonic) {
        self.instruct_buf.push(Instr::new_instr0(mnem));
    }

    pub fn instr1(&mut self, mnem: Mnemonic, op1: impl Into<Opr>) {
        self.instruct_buf.push(Instr::new_instr1(mnem, op1));
    }

    pub fn instr2(&mut self, mnem: Mnemonic, op1: impl Into<Opr>, op2: impl Into<Opr>) {
        self.instruct_buf.push(Instr::new_instr2(mnem, op1, op2));
    }

    pub fn set_lable(&mut self, lable: impl Display) {
        self.instruct_buf.push(Instr::Lable(lable.to_string()));
    }
}
/*
        let lable_string = lable.to_string();
        if lable_string.starts_with(".") {
            if self.last_lable.is_empty() {
                panic!("Unknown jump location from {lable_string}");
            }
            self.instruct_buf.push(Instr::Lable(format!("{}{}",self.last_lable,lable.to_string())));
        } else {
 * */
