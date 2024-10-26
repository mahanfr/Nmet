use crate::{codegen::{instructions::{Opr, Oprs}, mnemonic::Mnemonic, register::Reg}, compiler::CompilerContext};

#[allow(dead_code)]
static INTER_MEM_INIT_CAPACITY : usize = 4096;

#[derive(Debug, Clone)]
pub struct InptrRegister {
    a : u64,
    b : u64,
    c : u64,
    d : u64,
}

impl Default for InptrRegister {
    fn default() -> Self {
        Self {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
        }
    }
}

#[allow(dead_code)]
impl InptrRegister {

    pub fn get_register(&self, reg: &Reg) -> u64 {
       match reg {
           Reg::RAX | Reg::EAX | Reg::AX | Reg::AL => self.a,
           Reg::RBX | Reg::EBX | Reg::BX | Reg::BL => self.b,
           Reg::RCX | Reg::ECX | Reg::CX | Reg::CL => self.c,
           Reg::RDX | Reg::EDX | Reg::DX | Reg::DL => self.d,
           _ => unimplemented!()
       }
    }

    pub fn set_register(&mut self, reg: &Reg, value: u64) {
       match reg {
           Reg::RAX | Reg::EAX | Reg::AX | Reg::AL => self.a = value,
           Reg::RBX | Reg::EBX | Reg::BX | Reg::BL => self.b = value,
           Reg::RCX | Reg::ECX | Reg::CX | Reg::CL => self.c = value,
           Reg::RDX | Reg::EDX | Reg::DX | Reg::DL => self.d = value,
           _ => unimplemented!()
       }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct IntrMemory {
    arena: Vec<u8>,
}


#[allow(dead_code)]
impl IntrMemory {
    pub fn new() -> Self {
        Self {
            arena: Vec::with_capacity(INTER_MEM_INIT_CAPACITY),
        }
    }
}

#[derive(Debug, Clone)]
pub struct IntrState {
    memory: IntrMemory,
    registers: InptrRegister,
}

impl IntrState {
    pub fn new() -> Self {
        Self {
            memory: IntrMemory::new(),
            registers: InptrRegister::default()
        }
    }

}

pub fn simulate_mov(state: &mut IntrState, oprs: &Oprs) {
}

pub fn simulate_program(cc: &CompilerContext) {
    let mut state = IntrState::new();
    let instrs = cc.codegen.get_raw_instructs();
    for instr in instrs {
        match &instr.mnem {
            Mnemonic::Mov => {
                simulate_mov(&mut state, &instr.oprs);
            }
            _ => todo!("{} {}", instr.mnem, instr.oprs),
        }
    }
}

