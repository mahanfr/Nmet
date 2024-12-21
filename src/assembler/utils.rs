use crate::compiler::CompilerContext;

use super::{instructions::Opr, mnemonic::Mnemonic, register::Reg};

pub fn mov_unknown_to_register(cc: &mut CompilerContext, r: Reg, opr: Opr) {
    match &opr {
        Opr::Mem(m) => {
            if m.size == 0 {
                cc.codegen.instr2(Mnemonic::Mov, r.convert(8), opr);
            } else {
                cc.codegen.instr2(Mnemonic::Mov, r.convert(m.size), opr);
            }
        }
        Opr::R64(r2) | Opr::R32(r2) | Opr::R8(r2) => {
            if !(r.opcode() == r2.opcode() && r.is_extended() == r2.is_extended()) {
                cc.codegen.instr2(Mnemonic::Mov, r, *r2);
            }
        }
        _ => {
            cc.codegen.instr2(Mnemonic::Mov, r, opr);
        }
    }
}

pub fn restore_last_temp_value(cc: &mut CompilerContext, to: Reg) {
    cc.codegen.instr1(Mnemonic::Pop, to);
}

pub fn save_temp_value(cc: &mut CompilerContext, opr: Opr) {
    if opr.is_mem() {
        mov_unknown_to_register(cc, Reg::RAX, opr);
        cc.codegen.instr1(Mnemonic::Push, Reg::RAX);
    } else {
        cc.codegen.instr1(Mnemonic::Push, opr);
    }
}
