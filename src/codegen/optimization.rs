use crate::compiler::CompilerContext;

use super::{register::Reg, instructions::Opr, mnemonic::Mnemonic};


pub fn mov_unknown_to_register(cc: &mut CompilerContext, r: Reg, opr: Opr) {
    match opr {
        Opr::Mem(m) => {
            if m.size == 0 {
                cc.codegen.instr2(Mnemonic::Mov, r.convert(8), opr);
            } else {
                cc.codegen.instr2(Mnemonic::Mov, r.convert(m.size), opr);
            }
        },
        Opr::R64(r2) | Opr::R32(r2) | Opr::R8(r2) => {
            if !(r.opcode() == r2.opcode() && r.is_extended() == r2.is_extended()) {
                cc.codegen.instr2(Mnemonic::Mov, r, r2);
            }
        },
        _ => {
            cc.codegen.instr2(Mnemonic::Mov, r, opr);
        },
    }
}
