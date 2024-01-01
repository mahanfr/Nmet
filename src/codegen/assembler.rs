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

#[import_instructions("./x86/instrs.txt")]
pub enum Insns {}
