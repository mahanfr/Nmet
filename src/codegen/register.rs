use core::ops::{Add, Mul, Sub};
use std::{fmt::Display, str::FromStr};

use crate::parser::types::VariableType;

use super::memory::MemOp;

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

impl Add<usize> for Reg {
    type Output = MemOp;

    fn add(self, rhs: usize) -> Self::Output {
        MemOp::Offset(self, rhs)
    }
}

impl Sub<usize> for Reg {
    type Output = MemOp;

    fn sub(self, rhs: usize) -> Self::Output {
        MemOp::Negate(self, rhs)
    }
}

impl Mul<usize> for Reg {
    type Output = MemOp;

    fn mul(self, rhs: usize) -> Self::Output {
        MemOp::Multi(self, rhs)
    }
}

impl FromStr for Reg {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lowercase_s = s.to_string();
        lowercase_s = lowercase_s.to_lowercase();
        match lowercase_s.trim() {
            "rax" => Ok(Self::RAX),
            "rcx" => Ok(Self::RCX),
            "rdx" => Ok(Self::RDX),
            "rbx" => Ok(Self::RBX),
            "rsp" => Ok(Self::RSP),
            "rbp" => Ok(Self::RBP),
            "rsi" => Ok(Self::RSI),
            "rdi" => Ok(Self::RDI),
            "r8" => Ok(Self::R8),
            "r9" => Ok(Self::R9),
            "eax" => Ok(Self::EAX),
            "ecx" => Ok(Self::ECX),
            "edx" => Ok(Self::EDX),
            "ebx" => Ok(Self::EBX),
            "esp" => Ok(Self::ESP),
            "ebp" => Ok(Self::EBP),
            "esi" => Ok(Self::ESI),
            "edi" => Ok(Self::EDI),
            "r8d" => Ok(Self::R8D),
            "r9d" => Ok(Self::R9D),
            "ax" => Ok(Self::AX),
            "cx" => Ok(Self::CX),
            "dx" => Ok(Self::DX),
            "bx" => Ok(Self::BX),
            "sp" => Ok(Self::SP),
            "bp" => Ok(Self::BP),
            "si" => Ok(Self::SI),
            "di" => Ok(Self::DI),
            "r8w" => Ok(Self::R8W),
            "r9w" => Ok(Self::R9W),
            "ah" => Ok(Self::AH),
            "al" => Ok(Self::AL),
            "ch" => Ok(Self::CH),
            "cl" => Ok(Self::CL),
            "dh" => Ok(Self::DH),
            "dl" => Ok(Self::DL),
            "bh" => Ok(Self::BH),
            "bl" => Ok(Self::BL),
            "spl" => Ok(Self::SPL),
            "bpl" => Ok(Self::BPL),
            "sil" => Ok(Self::SIL),
            "dil" => Ok(Self::DIL),
            "r8b" => Ok(Self::R8B),
            "r9b" => Ok(Self::R9B),
            _ => Err(format!("Unsupported Register \"{lowercase_s}\"!")),
        }
    }
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
