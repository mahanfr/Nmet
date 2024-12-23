/**********************************************************************************************
*
*   compiler/bif: built-in functions
*
*   LICENSE: MIT
*
*   Copyright (c) 2023-2024 Mahan Farzaneh (@mahanfr)
*
*   This software is provided "as-is", without any express or implied warranty. In no event
*   will the authors be held liable for any damages arising from the use of this software.
*
*   Permission is granted to anyone to use this software for any purpose, including commercial
*   applications, and to alter it and redistribute it freely, subject to the following restrictions:
*
*     1. The origin of this software must not be misrepresented; you must not claim that you
*     wrote the original software. If you use this software in a product, an acknowledgment
*     in the product documentation would be appreciated but is not required.
*
*     2. Altered source versions must be plainly marked as such, and must not be misrepresented
*     as being the original software.
*
*     3. This notice may not be removed or altered from any source distribution.
*
**********************************************************************************************/
use crate::{
    assembler::{
        instructions::Opr, memory::MemAddr, mnemonic::Mnemonic::*, register::Reg::*, Codegen,
    },
    mem, memb, memq,
};

#[derive(Hash, PartialEq, Eq)]
pub enum Bif {
    Print,
}

impl Bif {
    pub fn implement(&self, codegen: &mut Codegen) {
        match self {
            Self::Print => Self::print_impl(codegen),
        }
    }

    fn print_impl(codegen: &mut Codegen) {
        codegen.set_lable("print");
        codegen.instr1(Push, RBP);
        codegen.instr2(Mov, RBP, RSP);
        codegen.instr2(Sub, RSP, 64);
        codegen.instr2(Mov, memq!(RBP, -56), RDI);
        codegen.instr2(Mov, memq!(RBP, -8), 1);
        codegen.instr2(Mov, RAX, 32);
        codegen.instr2(Sub, RAX, memq!(RBP, -8));
        codegen.instr2(Mov, memb!(RBP, -48, RAX), 10);
        codegen.set_lable("print.L3");
        codegen.instr2(Mov, RCX, memq!(RBP, -56));
        codegen.instr2(Mov, RDX, -3689348814741910323i64);
        codegen.instr2(Mov, RAX, RCX);
        codegen.instr1(Mul, RDX);
        codegen.instr2(Shr, RDX, 3);
        codegen.instr2(Mov, RAX, RDX);
        codegen.instr2(Sal, RAX, 2);
        codegen.instr2(Add, RAX, RDX);
        codegen.instr2(Add, RAX, RAX);
        codegen.instr2(Sub, RCX, RAX);
        codegen.instr2(Mov, RDX, RCX);
        codegen.instr2(Mov, EAX, EDX);
        codegen.instr2(Lea, EDX, mem!(RAX, 48));
        codegen.instr2(Mov, EAX, 31);
        codegen.instr2(Sub, RAX, memq!(RBP, -8));
        codegen.instr2(Mov, memb!(RBP, -48, RAX), DL);
        codegen.instr2(Add, memq!(RBP, -8), 1);
        codegen.instr2(Mov, RAX, memq!(RBP, -56));
        codegen.instr2(Mov, RDX, -3689348814741910323i64);
        codegen.instr1(Mul, RDX);
        codegen.instr2(Mov, RAX, RDX);
        codegen.instr2(Shr, RAX, 3);
        codegen.instr2(Mov, memq!(RBP, -56), RAX);
        codegen.instr2(Cmp, memq!(RBP, -56), 0);
        codegen.instr1(Jne, Opr::rel("print.L3"));
        codegen.instr2(Mov, EAX, 32);
        codegen.instr2(Sub, RAX, memq!(RBP, -8));
        codegen.instr2(Lea, RDX, mem!(RBP, -48));
        codegen.instr2(Add, RAX, RDX);
        codegen.instr2(Mov, RSI, RAX);
        codegen.instr2(Mov, RBX, memq!(RBP, -8));
        codegen.instr2(Mov, RDX, RBX);
        codegen.instr2(Mov, RDI, 1);
        codegen.instr2(Mov, RAX, 1);
        codegen.instr0(Syscall);
        codegen.instr0(Leave);
        codegen.instr0(Ret);
    }
}
