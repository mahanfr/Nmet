use crate::parser::types::VariableType;
use core::ops::{Add, Sub};
use std::fmt::Display;

use super::register::Reg;

pub enum Mem {
    U(MemOp),
    Byte(MemOp),
    Word(MemOp),
    Dword(MemOp),
    Qword(MemOp),
}

impl Mem {
    pub fn dyn_sized(vtype: &VariableType, mem_op: MemOp) -> Self {
        let size = vtype.item_size();
        match size {
            1 => Self::Byte(mem_op),
            2 => Self::Word(mem_op),
            4 => Self::Dword(mem_op),
            8 => Self::Qword(mem_op),
            _ => unreachable!(),
        }
    }
}

impl Display for Mem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::U(mop) => write!(f, "[{mop}]"),
            Self::Byte(mop) => write!(f, "Byte [{mop}]"),
            Self::Word(mop) => write!(f, "Word [{mop}]"),
            Self::Dword(mop) => write!(f, "Dword [{mop}]"),
            Self::Qword(mop) => write!(f, "Qword [{mop}]"),
        }
    }
}

pub enum MemOp {
    Single(Reg),
    Offset(Reg, usize),
    Negate(Reg, usize),
    Multi(Reg, usize),
    Add(Box<MemOp>, Box<MemOp>),
    Sub(Box<MemOp>, Box<MemOp>),
}

impl From<Reg> for MemOp {
    fn from(val: Reg) -> MemOp {
        MemOp::Single(val)
    }
}

impl Add for MemOp {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Add(Box::new(self), Box::new(rhs))
    }
}

impl Sub for MemOp {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Sub(Box::new(self), Box::new(rhs))
    }
}

impl Display for MemOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Single(r) => write!(f, "{r}"),
            Self::Offset(r, i) => write!(f, "{r} + {i}"),
            Self::Negate(r, i) => write!(f, "{r} - {i}"),
            Self::Multi(r, i) => write!(f, "{r} * {i}"),
            Self::Add(r1, r2) => write!(f, "{r1} + {r2}"),
            Self::Sub(r1, r2) => write!(f, "{r1} - {r2}"),
        }
    }
}
