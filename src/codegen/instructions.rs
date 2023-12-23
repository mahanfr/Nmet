use std::fmt::Display;

use super::{
    memory::Mem,
    mnmemonic::Mnemonic::{self, *},
    register::Reg,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Opr {
    R64(Reg),
    R32(Reg),
    R16(Reg),
    R8(Reg),
    R4(Reg),
    Mem(Mem),
    Imm64(i64),
    Imm32(i32),
    Lable(String),
}

impl From<Reg> for Opr {
    fn from(val: Reg) -> Opr {
        let size = ((val as u8) & 0xf0) >> 4;
        match size {
            8 => Self::R64(val),
            4 => Self::R32(val),
            2 => Self::R16(val),
            1 => Self::R8(val),
            0 => Self::R4(val),
            _ => unreachable!(),
        }
    }
}
impl From<Mem> for Opr {
    fn from(val: Mem) -> Opr {
        Self::Mem(val)
    }
}
impl From<usize> for Opr {
    fn from(val: usize) -> Opr {
        Self::Imm32(val as i32)
    }
}
impl From<i32> for Opr {
    fn from(val: i32) -> Opr {
        Self::Imm32(val as i32)
    }
}
impl From<i64> for Opr {
    fn from(val: i64) -> Opr {
        Self::Imm64(val)
    }
}
impl From<&Mem> for Opr {
    fn from(value: &Mem) -> Self {
        Self::Mem(value.clone())
    }
}

impl From<&str> for Opr {
    fn from(value: &str) -> Self {
        Self::Lable(value.to_string())
    }
}
impl From<String> for Opr {
    fn from(val: String) -> Opr {
        Self::Lable(val)
    }
}

impl Display for Opr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::R64(x) | Self::R32(x) | Self::R16(x) | Self::R8(x) | Self::R4(x) => x.fmt(f),
            Self::Mem(x) => x.fmt(f),
            Self::Imm64(x) => x.fmt(f),
            Self::Imm32(x) => x.fmt(f),
            Self::Lable(x) => x.fmt(f),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Instr {
    Lable(String),
    Lea(Opr, Opr),
    Mov(Opr, Opr),
    Cmove(Opr, Opr),
    Cmovne(Opr, Opr),
    Cmovg(Opr, Opr),
    Cmovl(Opr, Opr),
    Cmovge(Opr, Opr),
    Cmovle(Opr, Opr),
    Push(Opr),
    Pop(Opr),
    Add(Opr, Opr),
    Sub(Opr, Opr),
    Imul(Opr, Opr),
    Idiv(Opr),
    Mul(Opr),
    Or(Opr, Opr),
    And(Opr, Opr),
    Sal(Opr, Opr),
    Sar(Opr, Opr),
    Shr(Opr, Opr),
    Cmp(Opr, Opr),
    Test(Opr, Opr),
    Cqo,
    Neg(Opr),
    Not(Opr),
    Call(Opr),
    Jmp(Opr),
    Jz(Opr),
    Jnz(Opr),
    Jne(Opr),
    Syscall,
    Leave,
    Ret,
    Nop,
}

impl Display for Instr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Lable(l) => write!(f, "{l}:"),
            Self::Lea(op1, op2) => write!(f, "lea {op1}, {op2}"),
            Self::Mov(op1, op2) => write!(f, "mov {op1}, {op2}"),
            Self::Cmove(op1, op2) => write!(f, "cmove {op1}, {op2}"),
            Self::Cmovne(op1, op2) => write!(f, "cmovne {op1}, {op2}"),
            Self::Cmovg(op1, op2) => write!(f, "cmovg {op1}, {op2}"),
            Self::Cmovl(op1, op2) => write!(f, "cmovl {op1}, {op2}"),
            Self::Cmovge(op1, op2) => write!(f, "cmovge {op1}, {op2}"),
            Self::Cmovle(op1, op2) => write!(f, "cmovle {op1}, {op2}"),
            Self::Add(op1, op2) => write!(f, "add {op1}, {op2}"),
            Self::Sub(op1, op2) => write!(f, "sub {op1}, {op2}"),
            Self::Imul(op1, op2) => write!(f, "imul {op1}, {op2}"),
            Self::Sal(op1, op2) => write!(f, "sal {op1}, {op2}"),
            Self::Sar(op1, op2) => write!(f, "sar {op1}, {op2}"),
            Self::Shr(op1, op2) => write!(f, "shr {op1}, {op2}"),
            Self::Cmp(op1, op2) => write!(f, "cmp {op1}, {op2}"),
            Self::Test(op1, op2) => write!(f, "test {op1}, {op2}"),
            Self::Or(op1, op2) => write!(f, "or {op1}, {op2}"),
            Self::And(op1, op2) => write!(f, "and {op1}, {op2}"),
            Self::Idiv(op1) => write!(f, "idiv {op1}"),
            Self::Mul(op1) => write!(f, "mul {op1}"),
            Self::Push(op1) => write!(f, "push {op1}"),
            Self::Pop(op1) => write!(f, "pop {op1}"),
            Self::Neg(op1) => write!(f, "neg {op1}"),
            Self::Not(op1) => write!(f, "not {op1}"),
            Self::Jmp(op1) => write!(f, "jmp {op1}"),
            Self::Jz(op1) => write!(f, "jz {op1}"),
            Self::Jnz(op1) => write!(f, "jnz {op1}"),
            Self::Jne(op1) => write!(f, "jne {op1}"),
            Self::Cqo => write!(f, "cqo"),
            Self::Call(l) => write!(f, "call {l}"),
            Self::Syscall => write!(f, "syscall"),
            Self::Leave => write!(f, "leave"),
            Self::Ret => write!(f, "ret"),
            Self::Nop => write!(f, "nop"),
        }
    }
}

impl Instr {
    pub fn assemble(&self) -> Vec<u8> {
        match self {
            Self::Mov(op1, op2) => match (op1, op2) {
                (Opr::R64(r) | Opr::R32(r), Opr::Imm32(val)) => {
                    let mut bytes = Vec::<u8>::new();
                    bytes.push(0xb8 + r.upcode32());
                    bytes.extend(val.to_le_bytes());
                    bytes
                }
                _ => todo!(),
            },
            Self::Syscall => {
                vec![0x0f, 0x05]
            }
            Self::Lable(_) => vec![],
            _ => todo!(),
        }
    }

    pub fn new_instr0(mnem: Mnemonic) -> Self {
        match mnem {
            Cqo => Self::Cqo,
            Leave => Self::Leave,
            Syscall => Self::Syscall,
            Ret => Self::Ret,
            _ => panic!("Wrong operand count for {mnem:?}"),
        }
    }

    pub fn new_instr1(mnem: Mnemonic, op1: impl Into<Opr>) -> Self {
        match mnem {
            Idiv => Self::Idiv(op1.into()),
            Mul => Self::Mul(op1.into()),
            Push => Self::Push(op1.into()),
            Pop => Self::Pop(op1.into()),
            Neg => Self::Neg(op1.into()),
            Not => Self::Not(op1.into()),
            Jmp => Self::Jmp(op1.into()),
            Jz => Self::Jz(op1.into()),
            Jnz => Self::Jnz(op1.into()),
            Jne => Self::Jne(op1.into()),
            Call => Self::Call(op1.into()),
            _ => panic!("Wrong operand count for {mnem:?}"),
        }
    }

    pub fn new_instr2(mnem: Mnemonic, op1: impl Into<Opr>, op2: impl Into<Opr>) -> Self {
        match mnem {
            Lea => Self::Lea(op1.into(), op2.into()),
            Mov => Self::Mov(op1.into(), op2.into()),
            Cmove => Self::Cmove(op1.into(), op2.into()),
            Cmovne => Self::Cmovne(op1.into(), op2.into()),
            Cmovg => Self::Cmovg(op1.into(), op2.into()),
            Cmovl => Self::Cmovl(op1.into(), op2.into()),
            Cmovge => Self::Cmovge(op1.into(), op2.into()),
            Cmovle => Self::Cmovle(op1.into(), op2.into()),
            Add => Self::Add(op1.into(), op2.into()),
            Sub => Self::Sub(op1.into(), op2.into()),
            Imul => Self::Imul(op1.into(), op2.into()),
            Sal => Self::Sal(op1.into(), op2.into()),
            Sar => Self::Sar(op1.into(), op2.into()),
            Shr => Self::Shr(op1.into(), op2.into()),
            Cmp => Self::Cmp(op1.into(), op2.into()),
            Test => Self::Test(op1.into(), op2.into()),
            Or => Self::Or(op1.into(), op2.into()),
            And => Self::And(op1.into(), op2.into()),
            _ => panic!("Wrong operand count for {mnem:?}"),
        }
    }
}
