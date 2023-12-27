use std::{fmt::Display, process::exit};

use super::{
    mnmemonic::Mnemonic::{self, *},
    register::Reg,
};

#[macro_export]
macro_rules! mem {
    ($R1:expr) => {
        MemAddr::new($R1)
    };
    ($R1:expr, $disp:expr) => {
        MemAddr::new_disp($R1, $disp)
    };
    ($R1:expr, $disp:expr, $R2:expr) => {
        MemAddr::new_sib($R1, $disp, $R2, 1)
    };
    ($R1:expr, $disp:expr, $R2:expr, $scale:expr) => {
        MemAddr::new_sib($R1, $disp, $R2, $scale)
    };
}

#[macro_export]
macro_rules! mem_s {
    ($s:expr, $R1:expr) => {
        MemAddr::new_s($s, $R1)
    };
    ($s:expr, $R1:expr, $disp:expr) => {
        MemAddr::new_disp_s($s, $R1, $disp)
    };
    ($s:expr, $R1:expr, $disp:expr, $R2:expr) => {
        MemAddr::new_sib_s($s, $R1, $disp, $R2, 1)
    };
    ($s:expr, $R1:expr, $disp:expr, $R2:expr, $scale:expr) => {
        MemAddr::new_sib_s($s, $R1, $disp, $R2, $scale)
    };
}

#[macro_export]
macro_rules! memq {
    ($R1:expr) => {
        MemAddr::new_s(8, $R1)
    };
    ($R1:expr, $disp:expr) => {
        MemAddr::new_disp_s(8, $R1, $disp)
    };
    ($R1:expr, $disp:expr, $R2:expr) => {
        MemAddr::new_sib_s(8, $R1, $disp, $R2, 1)
    };
    ($R1:expr, $disp:expr, $R2:expr, $scale:expr) => {
        MemAddr::new_sib_s(8, $R1, $disp, $R2, $scale)
    };
}
#[macro_export]
macro_rules! memd {
    ($R1:expr) => {
        MemAddr::new_s(4, $R1)
    };
    ($R1:expr, $disp:expr) => {
        MemAddr::new_disp_s(4, $R1, $disp)
    };
    ($R1:expr, $disp:expr, $R2:expr) => {
        MemAddr::new_sib_s(4, $R1, $disp, $R2, 1)
    };
    ($R1:expr, $disp:expr, $R2:expr, $scale:expr) => {
        MemAddr::new_sib_s(4, $R1, $disp, $R2, $scale)
    };
}
#[macro_export]
macro_rules! memw {
    ($R1:expr) => {
        MemAddr::new_s(2, $R1)
    };
    ($R1:expr, $disp:expr) => {
        MemAddr::new_disp_s(2, $R1, $disp)
    };
    ($R1:expr, $disp:expr, $R2:expr) => {
        MemAddr::new_sib_s(2, $R1, $disp, $R2, 1)
    };
    ($R1:expr, $disp:expr, $R2:expr, $scale:expr) => {
        MemAddr::new_sib_s(2, $R1, $disp, $R2, $scale)
    };
}
#[macro_export]
macro_rules! memb {
    ($R1:expr) => {
        MemAddr::new_s(1, $R1)
    };
    ($R1:expr, $disp:expr) => {
        MemAddr::new_disp_s(1, $R1, $disp)
    };
    ($R1:expr, $disp:expr, $R2:expr) => {
        MemAddr::new_sib_s(1, $R1, $disp, $R2, 1)
    };
    ($R1:expr, $disp:expr, $R2:expr, $scale:expr) => {
        MemAddr::new_sib_s(1, $R1, $disp, $R2, $scale)
    };
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MemAddr {
    size: u8,
    register: Reg,
    disp: Option<i32>,
    s_register: Option<Reg>,
    scale: Option<u8>,
}

impl MemAddr {
    fn validate_size(size: &u8) -> bool {
        matches!(size, 0 | 1 | 2 | 4 | 8)
    }

    fn validate_scale(scale: &u8) -> bool {
        matches!(scale, 1 | 2 | 4 | 8)
    }

    pub fn new(reg: Reg) -> Self {
        Self {
            size: 0,
            register: reg,
            disp: None,
            s_register: None,
            scale: None,
        }
    }

    pub fn new_s(size: u8, reg: Reg) -> Self {
        if !Self::validate_size(&size) {
            panic!("unexpected value for memory size");
        }
        let mut res = Self::new(reg);
        res.size = size;
        res
    }

    pub fn new_disp(reg: Reg, disp: i32) -> Self {
        let mut res = Self::new(reg);
        res.disp = Some(disp);
        res
    }

    pub fn new_disp_s(size: u8, reg: Reg, disp: i32) -> Self {
        if !Self::validate_size(&size) {
            panic!("unexpected value for memory size");
        }
        let mut res = Self::new_disp(reg, disp);
        res.size = size;
        res
    }

    pub fn new_sib(reg: Reg, disp: i32, reg_s: Reg, scale: u8) -> Self {
        if !Self::validate_scale(&scale) {
            panic!("unexpected value for scale to size");
        }
        Self {
            size: 0,
            register: reg,
            disp: Some(disp),
            s_register: Some(reg_s),
            scale: Some(scale),
        }
    }

    pub fn new_sib_s(size: u8, reg: Reg, disp: i32, reg_s: Reg, scale: u8) -> Self {
        if !Self::validate_size(&size) {
            panic!("unexpected value for memory size");
        }
        let mut res = Self::new_sib(reg, disp, reg_s, scale);
        res.size = size;
        res
    }

    fn mem_hint(size: &u8) -> &'static str {
        match size {
            0 => "",
            1 => "byte",
            2 => "word",
            4 => "dword",
            8 => "qword",
            _ => unreachable!(),
        }
    }
}

impl Display for MemAddr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut view = String::new();
        match self.size {
            1 | 2 | 4 | 8 => {
                view.push_str(Self::mem_hint(&self.size));
                view.push(' ');
            }
            0 => (),
            _ => unreachable!(),
        }
        view.push('[');
        view.push_str(self.register.to_string().as_str());
        if let Some(disp) = self.disp {
            if disp < 0 {
                view.push_str(" - ");
            } else {
                view.push_str(" + ");
            }
            view.push_str(disp.abs().to_string().as_str());
        }
        if let Some(reg) = self.s_register {
            view.push_str(" + ");
            view.push_str(reg.to_string().as_str());
            view.push_str(" * ");
            view.push_str(self.scale.unwrap().to_string().as_str());
        }
        view.push(']');
        write!(f, "{view}")
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Opr {
    R64(Reg),
    R32(Reg),
    R16(Reg),
    R8(Reg),
    R4(Reg),
    Mem(MemAddr),
    Imm64(i64),
    Imm32(i32),
    Lable(String),
}

impl From<MemAddr> for Opr {
    fn from(val: MemAddr) -> Opr {
        Opr::Mem(val)
    }
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
impl From<&Opr> for Opr {
    fn from(val: &Self) -> Opr {
        val.clone()
    }
}

impl From<usize> for Opr {
    fn from(val: usize) -> Opr {
        Self::Imm32(val as i32)
    }
}
impl From<i32> for Opr {
    fn from(val: i32) -> Opr {
        Self::Imm32(val)
    }
}
impl From<i64> for Opr {
    fn from(val: i64) -> Opr {
        Self::Imm64(val)
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
            Self::Mem(ma) => ma.fmt(f),
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
fn modrm_ex(modr: u8, ex: u8, reg: Reg) -> u8 {
    (((modr & 0x3) << 3) | (ex & 0x07) << 3) | (reg.upcode32() & 0x07)
}

fn modrm_r(reg1: Reg, reg2: Reg) -> u8 {
    ((0b11 << 3) | (reg1.upcode32() & 0x07) << 3) | (reg2.upcode32() & 0x07)
}

fn modrm(modr: u8, r1: Reg, r2: Reg) -> u8 {
    (((modr & 0x3) << 3) | (r1.upcode32() & 0x07) << 3) | (r2.upcode32() & 0x07)
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
                (Opr::R64(r1), Opr::R64(r2)) => {
                    vec![0x48, 0x89, modrm_r(*r2, *r1)]
                }
                (Opr::Mem(mem_addr), Opr::Imm32(val)) => {
                    if let Some(disp) = mem_addr.disp {
                        if disp.abs() < i8::MAX as i32 {
                            let mut bytes = vec![
                                0x48,
                                0x89,
                                modrm_ex(0b01, 0, mem_addr.register),
                                (disp.to_le_bytes()[0]),
                            ];
                            bytes.extend(val.to_le_bytes());
                            bytes
                        } else {
                            let mut bytes = vec![0x48, 0x89, modrm_ex(0b01, 0, mem_addr.register)];
                            bytes.extend(disp.to_le_bytes());
                            bytes.extend(val.to_le_bytes());
                            bytes
                        }
                    } else {
                        unimplemented!()
                    }
                }
                (Opr::Mem(mem_addr), Opr::R64(r)) => {
                    if let Some(disp) = mem_addr.disp {
                        if disp.abs() < i8::MAX as i32 {
                            vec![
                                0x48,
                                0xc7,
                                modrm(0b01, *r, mem_addr.register),
                                (disp.to_le_bytes()[0]),
                            ]
                        } else {
                            let mut bytes = vec![0x48, 0x89, modrm(0b01, *r, mem_addr.register)];
                            bytes.extend(disp.to_le_bytes());
                            bytes
                        }
                    } else {
                        unimplemented!()
                    }
                }
                _ => todo!("{self}"),
            },
            Self::Push(op1) => match op1 {
                Opr::Imm32(val) => {
                    // TODO: Might be 0x6A
                    let mut bytes = vec![0x68];
                    bytes.extend(val.to_le_bytes());
                    bytes
                }
                Opr::R64(r) => {
                    vec![(0x50 + r.upcode32())]
                }
                _ => todo!("{op1}"),
            },
            Self::Pop(op1) => {
                let Opr::R64(r) = op1 else {
                    eprintln!("Unsupported Operator ({op1}) for instruction {self}");
                    exit(1);
                };
                vec![(58 + r.upcode32())]
            }
            Self::Cqo => vec![0x48, 0x99],
            Self::Idiv(op1) => match op1 {
                Opr::R64(r) => vec![0x48, 0xf7, modrm_ex(0b11, 7, *r)],
                Opr::R32(r) => vec![0xf7, modrm_ex(0b11, 7, *r)],
                _ => todo!(),
            },
            Self::Add(op1, op2) => match (op1, op2) {
                (Opr::R64(r1), Opr::R64(r2)) => {
                    vec![0x48, 0x01, modrm_r(*r2, *r1)]
                }
                _ => unimplemented!(),
            },
            Self::Sub(op1, op2) => match (op1, op2) {
                (Opr::R64(r1), Opr::R64(r2)) => {
                    vec![0x48, 0x29, modrm_r(*r2, *r1)]
                }
                (Opr::R64(r1), Opr::Imm32(val)) => {
                    if *val < u8::MAX as i32 {
                        vec![0x48, 0x83, modrm_ex(0b11, 5, *r1)]
                    } else {
                        unimplemented!();
                    }
                }
                (Opr::R64(r), Opr::Mem(mem_addr)) => {
                    if let Some(disp) = mem_addr.disp {
                        if disp.abs() < i8::MAX as i32 {
                            vec![
                                0x48,
                                0x2b,
                                modrm(0b01, *r, mem_addr.register),
                                (disp.to_le_bytes()[0]),
                            ]
                        } else {
                            let mut bytes = vec![0x48, 0x2b, modrm(0b01, *r, mem_addr.register)];
                            bytes.extend(disp.to_le_bytes());
                            bytes
                        }
                    } else {
                        unimplemented!()
                    }
                }
                _ => unimplemented!("{self}"),
            },
            Self::Imul(op1, op2) => match (op1, op2) {
                (Opr::R64(r1), Opr::R64(r2)) => {
                    vec![0x48, 0x0f, 0xaf, modrm_r(*r2, *r1)]
                }
                _ => unimplemented!(),
            },
            Self::Or(op1, op2) => match (op1, op2) {
                (Opr::R64(r1), Opr::R64(r2)) => {
                    vec![0x48, 0x09, modrm_r(*r2, *r1)]
                }
                _ => unimplemented!(),
            },
            Self::And(op1, op2) => match (op1, op2) {
                (Opr::R64(r1), Opr::R64(r2)) => {
                    vec![0x48, 0x21, modrm_r(*r2, *r1)]
                }
                _ => unimplemented!(),
            },
            Self::Sar(op1, op2) => match (op1, op2) {
                (Opr::R64(r1), Opr::R8(Reg::CL)) => {
                    vec![0x48, 0xd3, modrm_ex(0b11, 7, *r1)]
                }
                _ => unimplemented!(),
            },
            Self::Sal(op1, op2) => match (op1, op2) {
                (Opr::R64(r1), Opr::R8(Reg::CL)) => {
                    vec![0x48, 0xd3, modrm_ex(0b11, 6, *r1)]
                }
                _ => unimplemented!(),
            },
            Self::Shr(op1, op2) => match (op1, op2) {
                (Opr::R64(r1), Opr::R8(Reg::CL)) => {
                    vec![0x48, 0xd3, modrm_ex(0b11, 5, *r1)]
                }
                _ => unimplemented!(),
            },
            Self::Call(_) => unreachable!("It should be handeled on higher level"),
            Self::Lable(_) => unreachable!("It should be handeled on higher level"),
            Self::Syscall => {
                vec![0x0f, 0x05]
            }
            _ => todo!("{self:?}"),
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
            _ => panic!("Wrong operand count for {mnem:?} {}", op1.into()),
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
