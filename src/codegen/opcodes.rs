use crate::codegen::Opr;

use super::{mnemonic::Mnemonic::*, instructions::{Oprs, Instr}};


pub fn opcode(instr: &Instr) -> u16 {
    match (instr.mnem, instr.oprs) {
        (Mov, Oprs::Two(Opr::R64(_) | Opr::R32(_), Opr::Imm32(_))) => 10u16,
        _ => 0u16,
    }
}

