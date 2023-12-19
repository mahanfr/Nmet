use std::fmt::Display;

use super::{mnmemonic::Mnemonic, register::Reg, memory::Mem};

#[derive(Debug, Clone, PartialEq)]
pub enum Opr {
    R(Reg),
    Mem(Mem),
    Imm(i64),
    Lable(String),
}

impl From<Reg> for Opr {
    fn from(val: Reg) -> Opr {
        Self::R(val)
    }
}
impl From<Mem> for Opr {
    fn from(val: Mem) -> Opr {
        Self::Mem(val)
    }
}
impl From<usize> for Opr {
    fn from(val: usize) -> Opr {
        Self::Imm(val as i64)
    }
}
impl From<i32> for Opr {
    fn from(val: i32) -> Opr {
        Self::Imm(val as i64)
    }
}
impl From<i64> for Opr {
    fn from(val: i64) -> Opr {
        Self::Imm(val)
    }
}
impl From<String> for Opr {
    fn from(val: String) -> Opr {
        Self::Lable(val)
    }
}

impl Display for Opr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::R(x) => x.fmt(f),
            Self::Mem(x) => x.fmt(f),
            Self::Imm(x) => x.fmt(f),
            Self::Lable(x) => x.fmt(f),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Instr {
    A0(Mnemonic),
    A1(Mnemonic, Opr),
    A2(Mnemonic, Opr, Opr),
}

impl Display for Instr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::A0(m) => m.fmt(f),
            Self::A1(m, opr1) => write!(f,"{m} {opr1}"),
            Self::A2(m, opr1, opr2) => write!(f,"{m} {opr1}, {opr2}"),
        }
    }
}

impl Instr {
    pub fn new_instr0(mnem: Mnemonic) -> Self {
        Self::A0(mnem)
    }

    pub fn new_instr1(mnem: Mnemonic, op1: impl Into<Opr>) -> Self {
        Self::A1(mnem, op1.into())
    }

    pub fn new_instr2(mnem: Mnemonic, op1: impl Into<Opr>, op2: impl Into<Opr>) -> Self {
        Self::A2(mnem, op1.into(), op2.into())
    }
}
