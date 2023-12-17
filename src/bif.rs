use crate::codegen::{Codegen, Mnemonic::*, R};


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
        codegen.instr1(Push, R::RBP);
        codegen.instr2(Mov, R::RBP, R::RSP);
        codegen.instr2(Sub, R::RSP, 64);
        codegen.instr2(Mov, "qword [rbp-56]", R::RDI);
        codegen.instr2(Mov, "qword [rbp-8]", 1);
        codegen.instr2(Mov, R::RAX, 32);
        codegen.instr2(Sub, R::RAX, "qword [rbp-8]");
        codegen.instr2(Mov, "byte [rbp-48+rax]", 10);
        codegen.set_lable(".L3");
        codegen.instr2(Mov, R::RCX, "qword [rbp-56]");
        codegen.instr2(Mov, R::RDX, -3689348814741910323i64);
        codegen.instr2(Mov, R::RAX, R::RCX);
        codegen.instr1(Mul, R::RDX);
        codegen.instr2(Shr, R::RDX, 3);
        codegen.instr2(Mov, R::RAX, R::RDX);
        codegen.instr2(Sal, R::RAX, 2);
        codegen.instr2(Add, R::RAX, R::RDX);
        codegen.instr2(Add, R::RAX, R::RAX);
        codegen.instr2(Sub, R::RCX, R::RAX);
        codegen.instr2(Mov, R::RDX, R::RCX);
        codegen.instr2(Mov, R::EAX, R::EDX);
        codegen.instr2(Lea, R::EDX, "[rax+48]");
        codegen.instr2(Mov, R::EAX, 31);
        codegen.instr2(Sub, R::RAX, "qword [rbp-8]");
        codegen.instr2(Mov, "byte [rbp-48+rax]", R::DL);
        codegen.instr2(Add, "qword [rbp-8]", 1);
        codegen.instr2(Mov, R::RAX, "qword [rbp-56]");
        codegen.instr2(Mov, R::RDX, -3689348814741910323i64);
        codegen.instr1(Mul, R::RDX);
        codegen.instr2(Mov, R::RAX, R::RDX);
        codegen.instr2(Shr, R::RAX, 3);
        codegen.instr2(Mov, "qword [rbp-56]", R::RAX);
        codegen.instr2(Cmp, "qword [rbp-56]", 0);
        codegen.instr1(Jne, ".L3");
        codegen.instr2(Mov, R::EAX, 32);
        codegen.instr2(Sub, R::RAX, "qword [rbp-8]");
        codegen.instr2(Lea, R::RDX, "[rbp-48]");
        codegen.instr2(Add, R::RAX, R::RDX);
        codegen.instr2(Mov, R::RSI, R::RAX);
        codegen.instr2(Mov, R::RBX, "qword [rbp-8]");
        codegen.instr2(Mov, R::RDX, R::RBX);
        codegen.instr2(Mov, R::RDI, 1);
        codegen.instr2(Mov, R::RAX, 1);
        codegen.instr0(Syscall);
        codegen.instr0(Leave);
        codegen.instr0(Ret);
    }
}
