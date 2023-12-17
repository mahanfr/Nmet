use std::fmt::Display;

use crate::parser::types::VariableType;

#[allow(dead_code)]
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum Reg {
    RAX = 0x80,
    RCX = 0x81,
    RDX = 0x82,
    RBX = 0x83,
    RSP = 0x84,
    RBP = 0x85,
    RSI = 0x86,
    RDI = 0x87,
    R8 = 0x88,
    R9 = 0x89,
    EAX = 0x40,
    ECX = 0x41,
    EDX = 0x42,
    EBX = 0x43,
    ESP = 0x44,
    EBP = 0x45,
    ESI = 0x46,
    EDI = 0x47,
    R8D = 0x48,
    R9D = 0x49,
    AX = 0x20,
    CX = 0x21,
    DX = 0x22,
    BX = 0x23,
    SP = 0x24,
    BP = 0x25,
    SI = 0x26,
    DI = 0x27,
    R8W = 0x28,
    R9W = 0x29,
    AH = 0x00,
    AL = 0x10,
    CH = 0x01,
    CL = 0x11,
    DH = 0x02,
    DL = 0x12,
    BH = 0x03,
    BL = 0x13,
    SPL = 0x14,
    BPL = 0x15,
    SIL = 0x16,
    DIL = 0x17,
    R8B = 0x18,
    R9B = 0x19,
}

#[allow(dead_code)]
#[allow(non_snake_case)]
impl Reg {
    pub fn AX_sized(vtype: &VariableType) -> Self {
        let size = vtype.item_size();
        match size {
            1 => Self::AL,
            2 => Self::AX,
            4 => Self::EAX,
            8 => Self::RAX,
            _ => unreachable!(),
        }
    }

    pub fn BX_sized(vtype: &VariableType) -> Self {
        let size = vtype.item_size();
        match size {
            1 => Self::BL,
            2 => Self::BX,
            4 => Self::EBX,
            8 => Self::RBX,
            _ => unreachable!(),
        }
    }

    pub fn CX_sized(vtype: &VariableType) -> Self {
        let size = vtype.item_size();
        match size {
            1 => Self::CL,
            2 => Self::CX,
            4 => Self::ECX,
            8 => Self::RCX,
            _ => unreachable!(),
        }
    }

    pub fn DX_sized(vtype: &VariableType) -> Self {
        let size = vtype.item_size();
        match size {
            1 => Self::DL,
            2 => Self::DX,
            4 => Self::EDX,
            8 => Self::RDX,
            _ => unreachable!(),
        }
    }

    pub fn DI_sized(vtype: &VariableType) -> Self {
        let size = vtype.item_size();
        match size {
            1 => Self::DIL,
            2 => Self::DI,
            4 => Self::EDI,
            8 => Self::RDI,
            _ => unreachable!(),
        }
    }

    pub fn Si_sized(vtype: &VariableType) -> Self {
        let size = vtype.item_size();
        match size {
            1 => Self::SIL,
            2 => Self::SI,
            4 => Self::ESI,
            8 => Self::RSI,
            _ => unreachable!(),
        }
    }

    pub fn R8_sized(vtype: &VariableType) -> Self {
        let size = vtype.item_size();
        match size {
            1 => Self::R8B,
            2 => Self::R8W,
            4 => Self::R8D,
            8 => Self::R8,
            _ => unreachable!(),
        }
    }

    pub fn R9_sized(vtype: &VariableType) -> Self {
        let size = vtype.item_size();
        match size {
            1 => Self::R9B,
            2 => Self::R9W,
            4 => Self::R9D,
            8 => Self::R9,
            _ => unreachable!(),
        }
    }
}

impl Display for Reg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RAX => write!(f, "rax"),
            Self::RCX => write!(f, "rcx"),
            Self::RDX => write!(f, "rdx"),
            Self::RBX => write!(f, "rbx"),
            Self::RSP => write!(f, "rsp"),
            Self::RBP => write!(f, "rbp"),
            Self::RSI => write!(f, "rsi"),
            Self::RDI => write!(f, "rdi"),
            Self::R8 => write!(f, "r8"),
            Self::R9 => write!(f, "r9"),
            Self::EAX => write!(f, "eax"),
            Self::ECX => write!(f, "ecx"),
            Self::EDX => write!(f, "edx"),
            Self::EBX => write!(f, "ebx"),
            Self::ESP => write!(f, "esp"),
            Self::EBP => write!(f, "ebp"),
            Self::ESI => write!(f, "esi"),
            Self::EDI => write!(f, "edi"),
            Self::R8D => write!(f, "r8d"),
            Self::R9D => write!(f, "r9d"),
            Self::AX => write!(f, "ax"),
            Self::CX => write!(f, "cx"),
            Self::DX => write!(f, "dx"),
            Self::BX => write!(f, "bx"),
            Self::SP => write!(f, "sp"),
            Self::BP => write!(f, "bp"),
            Self::SI => write!(f, "si"),
            Self::DI => write!(f, "di"),
            Self::R8W => write!(f, "r8w"),
            Self::R9W => write!(f, "r9w"),
            Self::AH => write!(f, "ah"),
            Self::AL => write!(f, "al"),
            Self::CH => write!(f, "ch"),
            Self::CL => write!(f, "cl"),
            Self::DH => write!(f, "dh"),
            Self::DL => write!(f, "dl"),
            Self::BH => write!(f, "bh"),
            Self::BL => write!(f, "bl"),
            Self::SPL => write!(f, "spl"),
            Self::BPL => write!(f, "bpl"),
            Self::SIL => write!(f, "sil"),
            Self::DIL => write!(f, "dil"),
            Self::R8B => write!(f, "r8b"),
            Self::R9B => write!(f, "r9b"),
        }
    }
}

#[allow(dead_code)]
pub enum Mnemonic {
    Lea,
    Mov,
    Cmove,
    Cmovne,
    Cmovg,
    Cmovl,
    Cmovge,
    Cmovle,
    Push,
    Pop,
    Add,
    Sub,
    Imul,
    Idiv,
    Mul,
    Or,
    And,
    Sal,
    Sar,
    Shr,
    Cmp,
    Test,
    Cqo,
    Neg,
    Not,
    Call,
    Jmp,
    Jz,
    Jnz,
    Jne,
    Syscall,
    Leave,
    Ret,
}

impl Display for Mnemonic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Lea => write!(f, "lea"),
            Self::Mov => write!(f, "mov"),
            Self::Cmove => write!(f, "cmove"),
            Self::Cmovne => write!(f, "cmovne"),
            Self::Cmovg => write!(f, "cmovg"),
            Self::Cmovl => write!(f, "cmovl"),
            Self::Cmovge => write!(f, "cmovge"),
            Self::Cmovle => write!(f, "cmovle"),
            Self::Push => write!(f, "push"),
            Self::Pop => write!(f, "pop"),
            Self::Add => write!(f, "add"),
            Self::Sub => write!(f, "sub"),
            Self::Imul => write!(f, "imul"),
            Self::Idiv => write!(f, "idiv"),
            Self::Mul => write!(f, "mul"),
            Self::Or => write!(f, "or"),
            Self::And => write!(f, "and"),
            Self::Sal => write!(f, "sal"),
            Self::Sar => write!(f, "sar"),
            Self::Shr => write!(f, "shr"),
            Self::Cmp => write!(f, "cmp"),
            Self::Test => write!(f, "test"),
            Self::Cqo => write!(f, "cqo"),
            Self::Neg => write!(f, "neg"),
            Self::Not => write!(f, "not"),
            Self::Jmp => write!(f, "jmp"),
            Self::Jz => write!(f, "jz"),
            Self::Jnz => write!(f, "jnz"),
            Self::Jne => write!(f, "jne"),
            Self::Syscall => write!(f, "syscall"),
            Self::Call => write!(f, "call"),
            Self::Leave => write!(f, "leave"),
            Self::Ret => write!(f, "ret"),
        }
    }
}

#[allow(dead_code)]
#[derive(Clone)]
pub struct Codegen {
    pub instruct_buf: Vec<String>,
    pub data_buf: Vec<String>,
    pub bss_buf: Vec<String>,
}

#[allow(dead_code)]
impl Codegen {
    pub fn new() -> Self {
        Self {
            instruct_buf: Vec::new(),
            bss_buf: Vec::new(),
            data_buf: Vec::new(),
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
        self.instruct_buf.push(String::new());
        self.instruct_buf.len() - 1
    }

    pub fn insert_raw(&mut self, instr: String) {
        self.instruct_buf.push(instr);
    }

    pub fn replace(&mut self, index: usize, instr: String) -> Result<(), String> {
        if index < self.instruct_buf.len() - 1 {
            self.instruct_buf[index] = instr;
            Ok(())
        } else {
            Err("index out of bounds!".into())
        }
    }

    pub fn insert_into_raw(&mut self, index: usize, instr: String) -> Result<(), String> {
        if index < self.instruct_buf.len() - 1 {
            self.instruct_buf[index] = instr;
            Ok(())
        } else {
            Err("index out of bounds!".into())
        }
    }

    pub fn instr0(&mut self, mnem: Mnemonic) {
        self.instruct_buf.push(mnem.to_string());
    }

    pub fn instr1(&mut self, mnem: Mnemonic, op1: impl Display) {
        self.instruct_buf.push(format!("{mnem} {op1}"));
    }

    pub fn instr2(&mut self, mnem: Mnemonic, op1: impl Display, op2: impl Display) {
        self.instruct_buf.push(format!("{mnem} {op1}, {op2}"));
    }

    pub fn set_lable(&mut self, lable: impl Display) {
        self.instruct_buf.push(format!("{lable}:"))
    }
}

// pub fn build_instr0(mnem: Mnemonic) -> String {
//     mnem.to_string()
// }

pub fn build_instr1(mnem: Mnemonic, op1: impl Display) -> String {
    format!("{mnem} {op1}")
}

pub fn build_instr2(mnem: Mnemonic, op1: impl Display, op2: impl Display) -> String {
    format!("{mnem} {op1}, {op2}")
}
