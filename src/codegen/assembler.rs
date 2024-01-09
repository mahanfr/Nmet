use std::fmt::Display;

use x86::import_instructions;

use super::{register::Reg, instructions::MemAddr};

#[allow(dead_code)]
enum ModrmType {
    Ext(u8),
    Add,
    Modrm,
    None,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
enum Opr {
    R64(Reg),
    R32(Reg),
    R16(Reg),
    R8(Reg),
    Mem(MemAddr),
    Imm8(i64),
    Imm32(i64),
    Imm64(i64),
}

impl From<Reg> for Opr {
    fn from(val: Reg) -> Opr {
        match val.size_bit() {
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
        if val <= u8::MAX as i32 && val >= i8::MIN as i32{
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
        }
    }
}

#[import_instructions("./x86/instrs.txt")]
pub enum Insns {}

#[test]
fn test() {
    assert_eq!("syscall", Insns::Syscall.to_string());
    assert_eq!("mov rax, rbx", 
               Insns::mov(Reg::RAX, Reg::RBX).to_string());
}
