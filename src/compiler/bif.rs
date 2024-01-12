use crate::{
    codegen::{mnemonic::Mnemonic::*, instructions::Instr, register::Reg::*, Codegen, memory::MemAddr},
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
        codegen.push_instr(Instr::new1(Push,RBP));
        codegen.push_instr(Instr::new2(Mov,RBP, RSP));
        codegen.push_instr(Instr::new2(Sub,RSP, 64));
        codegen.push_instr(Instr::new2(Mov,memq!(RBP, -56), RDI));
        codegen.push_instr(Instr::new2(Mov,memq!(RBP, -8), 1));
        codegen.push_instr(Instr::new2(Mov,RAX, 32));
        codegen.push_instr(Instr::new2(Sub,RAX, memq!(RBP, -8)));
        codegen.push_instr(Instr::new2(Mov,memb!(RBP, -48, RAX), 10));
        codegen.set_lable(".L3");
        codegen.push_instr(Instr::new2(Mov,RCX, memq!(RBP, -56)));
        codegen.push_instr(Instr::new2(Mov,RDX, -3689348814741910323i64));
        codegen.push_instr(Instr::new2(Mov,RAX, RCX));
        codegen.push_instr(Instr::new1(Mul,RDX));
        codegen.push_instr(Instr::new2(Shr,RDX, 3));
        codegen.push_instr(Instr::new2(Mov,RAX, RDX));
        codegen.push_instr(Instr::new2(Sal,RAX, 2));
        codegen.push_instr(Instr::new2(Add,RAX, RDX));
        codegen.push_instr(Instr::new2(Add,RAX, RAX));
        codegen.push_instr(Instr::new2(Sub,RCX, RAX));
        codegen.push_instr(Instr::new2(Mov,RDX, RCX));
        codegen.push_instr(Instr::new2(Mov,EAX, EDX));
        codegen.push_instr(Instr::new2(Lea,EDX, mem!(RAX, 48)));
        codegen.push_instr(Instr::new2(Mov,EAX, 31));
        codegen.push_instr(Instr::new2(Sub,RAX, memq!(RBP, -8)));
        codegen.push_instr(Instr::new2(Mov,memb!(RBP, -48, RAX), DL));
        codegen.push_instr(Instr::new2(Add,memq!(RBP, -8), 1));
        codegen.push_instr(Instr::new2(Mov,RAX, memq!(RBP, -56)));
        codegen.push_instr(Instr::new2(Mov,RDX, -3689348814741910323i64));
        codegen.push_instr(Instr::new1(Mul,RDX));
        codegen.push_instr(Instr::new2(Mov,RAX, RDX));
        codegen.push_instr(Instr::new2(Shr,RAX, 3));
        codegen.push_instr(Instr::new2(Mov,memq!(RBP, -56), RAX));
        codegen.push_instr(Instr::new2(Cmp,memq!(RBP, -56), 0));
        codegen.jne(".L3");
        codegen.push_instr(Instr::new2(Mov,EAX, 32));
        codegen.push_instr(Instr::new2(Sub,RAX, memq!(RBP, -8)));
        codegen.push_instr(Instr::new2(Lea,RDX, mem!(RBP, -48)));
        codegen.push_instr(Instr::new2(Add,RAX, RDX));
        codegen.push_instr(Instr::new2(Mov,RSI, RAX));
        codegen.push_instr(Instr::new2(Mov,RBX, memq!(RBP, -8)));
        codegen.push_instr(Instr::new2(Mov,RDX, RBX));
        codegen.push_instr(Instr::new2(Mov,RDI, 1));
        codegen.push_instr(Instr::new2(Mov,RAX, 1));
        codegen.push_instr(Instr::new0(Syscall));
        codegen.push_instr(Instr::new0(Leave));
        codegen.push_instr(Instr::new0(Ret));
    }
}
