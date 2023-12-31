use x86::import_instructions;

use super::{register::Reg, instructions::MemAddr};

#[allow(dead_code)]
pub enum Oprator {
    R64(Reg),
    R32(Reg),
    R16(Reg),
    R8(Reg),
    Mem(MemAddr),
    Imm8(i8),
    Imm32(i32),
    Imm64(i64),
}

#[import_instructions("./x86/instrs.txt")]
pub enum Insns {}
