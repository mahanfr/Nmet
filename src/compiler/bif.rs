use crate::{
    codegen::{instructions::Instr, register::Reg::*, Codegen, memory::MemAddr},
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
        codegen.push_instr(Instr::push(RBP));
        codegen.push_instr(Instr::mov(RBP, RSP));
        codegen.push_instr(Instr::sub(RSP, 64));
        codegen.push_instr(Instr::mov(memq!(RBP, -56), RDI));
        codegen.push_instr(Instr::mov(memq!(RBP, -8), 1));
        codegen.push_instr(Instr::mov(RAX, 32));
        codegen.push_instr(Instr::sub(RAX, memq!(RBP, -8)));
        codegen.push_instr(Instr::mov(memb!(RBP, -48, RAX), 10));
        codegen.set_lable(".L3");
        codegen.push_instr(Instr::mov(RCX, memq!(RBP, -56)));
        codegen.push_instr(Instr::mov(RDX, -3689348814741910323i64));
        codegen.push_instr(Instr::mov(RAX, RCX));
        codegen.push_instr(Instr::mul(RDX));
        codegen.push_instr(Instr::shr(RDX, 3));
        codegen.push_instr(Instr::mov(RAX, RDX));
        codegen.push_instr(Instr::sal(RAX, 2));
        codegen.push_instr(Instr::add(RAX, RDX));
        codegen.push_instr(Instr::add(RAX, RAX));
        codegen.push_instr(Instr::sub(RCX, RAX));
        codegen.push_instr(Instr::mov(RDX, RCX));
        codegen.push_instr(Instr::mov(EAX, EDX));
        codegen.push_instr(Instr::lea(EDX, mem!(RAX, 48)));
        codegen.push_instr(Instr::mov(EAX, 31));
        codegen.push_instr(Instr::sub(RAX, memq!(RBP, -8)));
        codegen.push_instr(Instr::mov(memb!(RBP, -48, RAX), DL));
        codegen.push_instr(Instr::add(memq!(RBP, -8), 1));
        codegen.push_instr(Instr::mov(RAX, memq!(RBP, -56)));
        codegen.push_instr(Instr::mov(RDX, -3689348814741910323i64));
        codegen.push_instr(Instr::mul(RDX));
        codegen.push_instr(Instr::mov(RAX, RDX));
        codegen.push_instr(Instr::shr(RAX, 3));
        codegen.push_instr(Instr::mov(memq!(RBP, -56), RAX));
        codegen.push_instr(Instr::cmp(memq!(RBP, -56), 0));
        codegen.jne(".L3");
        codegen.push_instr(Instr::mov(EAX, 32));
        codegen.push_instr(Instr::sub(RAX, memq!(RBP, -8)));
        codegen.push_instr(Instr::lea(RDX, mem!(RBP, -48)));
        codegen.push_instr(Instr::add(RAX, RDX));
        codegen.push_instr(Instr::mov(RSI, RAX));
        codegen.push_instr(Instr::mov(RBX, memq!(RBP, -8)));
        codegen.push_instr(Instr::mov(RDX, RBX));
        codegen.push_instr(Instr::mov(RDI, 1));
        codegen.push_instr(Instr::mov(RAX, 1));
        codegen.push_instr(Instr::syscall());
        codegen.push_instr(Instr::leave());
        codegen.push_instr(Instr::ret());
    }
}
