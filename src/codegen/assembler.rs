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
    assert!(true);
}
