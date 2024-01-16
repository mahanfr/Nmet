use std::fmt::Display;

#[allow(unused_imports)]
use super::{
    memory::MemAddr,
    mnemonic::Mnemonic::{self, *},
    register::Reg,
};

#[allow(dead_code)]
pub enum ModrmType {
    Ext(u8),
    Add,
    Modrm,
    None,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum Opr {
    R64(Reg),
    R32(Reg),
    R16(Reg),
    R8(Reg),
    Mem(MemAddr),
    Imm8(i64),
    Imm32(i64),
    Imm64(i64),
    Rel(String),
    Fs(String),
}

impl Opr {
    pub fn rel(rel: impl ToString) -> Self {
        Self::Rel(rel.to_string())
    }
}

impl From<MemAddr> for Opr {
    fn from(val: MemAddr) -> Opr {
        Opr::Mem(val)
    }
}

impl From<Reg> for Opr {
    fn from(val: Reg) -> Opr {
        match val.size() {
            64 => Self::R64(val),
            32 => Self::R32(val),
            16 => Self::R16(val),
            8 => Self::R8(val),
            _ => unreachable!(),
        }
    }
}

impl From<usize> for Opr {
    fn from(val: usize) -> Opr {
        if val as u64 <= u8::MAX as u64 {
            Self::Imm8(val as i64)
        } else if val as u64 <= u32::MAX as u64 {
            Self::Imm32(val as i64)
        } else {
            Self::Imm64(val as i64)
        }
    }
}
impl From<i32> for Opr {
    fn from(val: i32) -> Opr {
        if val <= u8::MAX as i32 && val >= i8::MIN as i32 {
            Self::Imm8(val as i64)
        } else {
            Self::Imm32(val as i64)
        }
    }
}
impl From<i64> for Opr {
    fn from(val: i64) -> Opr {
        Self::Imm64(val)
    }
}

impl Display for Opr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::R64(r) | Self::R32(r) | Self::R16(r) | Self::R8(r) => r.fmt(f),
            Self::Mem(m) => m.fmt(f),
            Self::Imm8(val) | Self::Imm32(val) | Self::Imm64(val) => val.fmt(f),
            Self::Rel(refer) => refer.fmt(f),
            Self::Fs(refer) => refer.fmt(f),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Oprs {
    None,
    One(Opr),
    Two(Opr, Opr),
}

impl Display for Oprs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, ""),
            Self::One(opr) => write!(f, "{opr}"),
            Self::Two(opr1, opr2) => write!(f, "{opr1}, {opr2}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Instr {
    pub mnem: Mnemonic,
    pub oprs: Oprs,
}

impl Display for Instr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if Oprs::None == self.oprs {
            write!(f, "{}", self.mnem)
        } else {
            write!(f, "{} {}", self.mnem, self.oprs)
        }
    }
}

impl Instr {
    pub fn replace_oprs(&mut self,oprs: Oprs) {
        self.oprs = oprs;
    }

    pub fn new(mnem: Mnemonic, oprs: Oprs) -> Self {
        Self {
            mnem,
            oprs,
        }
    }

    pub fn new0(mnem: Mnemonic) -> Self {
        Self {
            mnem,
            oprs: Oprs::None,
        }
    }

    pub fn new1(mnem: Mnemonic, opr: impl Into<Opr>) -> Self {
        let opr = opr.into();
        Self {
            mnem,
            oprs: Oprs::One(opr),
        }
    }

    pub fn new2(mnem: Mnemonic, opr1: impl Into<Opr>, opr2: impl Into<Opr>) -> Self {
        let opr1 = opr1.into();
        let opr2 = opr2.into();
        Self {
            mnem,
            oprs: Oprs::Two(opr1, opr2),
        }
    }

}
// #[import_instructions("./x86/instrs.txt")]
// pub enum Instr {}

#[test]
fn test_enum_varients() {
    assert_eq!("syscall", Instr::new0(Syscall).to_string());
    assert_eq!(
        "mov rax, rbx",
        Instr::new2(Mov, Reg::RAX, Reg::RBX).to_string()
    );
}

#[test]
fn test_new_instr() {
    assert_eq!(
        Instr::new2(Mov, Opr::R64(Reg::RAX), Opr::R64(Reg::RBX)),
        Instr::new(Mov, Oprs::Two(Opr::R64(Reg::RAX), Opr::R64(Reg::RBX)))
    );
}
