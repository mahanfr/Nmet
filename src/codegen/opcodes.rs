use crate::codegen::Opr::*;

use super::{
    instructions::{
        Instr,
        ModrmType::{self, *},
        Oprs::{self, *},
    },
    mnemonic::Mnemonic::{self, *},
    register::Reg,
};

macro_rules! r_8_64 {
    () => {
        R64(_) | R32(_) | R16(_) | R8(_)
    };
}
macro_rules! rm_8 {
    () => {
        R8(_) | Mem(_)
    };
}
macro_rules! rm_16_64 {
    () => {
        R64(_) | R32(_) | R16(_) | Mem(_)
    };
}
macro_rules! r_16_64 {
    () => {
        R64(_) | R32(_) | R16(_)
    };
}
macro_rules! imm {
    () => {
        Imm8(_) | Imm32(_) | Imm64(_)
    };
}

pub fn opcode(instr: &Instr) -> (u16, ModrmType) {
    match (&instr.mnem, &instr.oprs) {
        (Mnemonic::Add, Two(rm_16_64!(), r_16_64!())) => (0x01, Modrm),
        (Mnemonic::Add, Two(rm_16_64!(), Imm32(_))) => (0x81, Ext(0)),
        (Mnemonic::Add, Two(rm_16_64!(), Imm8(_))) => (0x83, Ext(0)),
        (Mov, Two(rm_8!(), R8(_))) => (0x88, Modrm),
        (Mov, Two(R8(_), Mem(_))) => (0x8A, Modrm),
        (Mov, Two(rm_16_64!(), r_16_64!())) => (0x89, Modrm),
        (Mov, Two(r_16_64!(), Mem(_))) => (0x8B, Modrm),
        (Mov, Two(r_16_64!(), imm!())) => (0xB8, ModrmType::Add),
        (Mov, Two(Mem(m), imm!())) => {
            if m.size == 1 {
                (0xC6, Ext(0))
            } else {
                (0xC7, Ext(0))
            }
        }
        (Push, One(Imm8(_))) => (0x6A, ModrmType::None),
        (Push, One(Imm32(_) | Imm64(_))) => (0x68, ModrmType::None),
        (Push, One(R64(_))) => (0x50, ModrmType::Add),
        (Pop, One(R64(_))) => (0x58, ModrmType::Add),
        (Sub, Two(rm_16_64!(), r_16_64!())) => (0x29, Modrm),
        (Sub, Two(r_16_64!(), Mem(_))) => (0x2B, Modrm),
        (Sub, Two(R64(Reg::RAX), Imm32(_))) => (0x2D, ModrmType::None),
        (Sub, Two(rm_16_64!(), Imm32(_))) => (0x81, Ext(5)),
        (Sub, Two(rm_16_64!(), Imm8(_))) => (0x83, Ext(5)),
        (Idiv, One(R64(_))) => (0xf7, Ext(7)),
        (Syscall, Oprs::None) => (0x0f05, ModrmType::None),
        (Leave, Oprs::None) => (0xc9, ModrmType::None),
        (Nop, Oprs::None) => (0x90, ModrmType::None),
        (Mul, One(R64(Reg::RDX))) => (0xf7, Ext(4)),
        (Imul, Two(r_16_64!(), rm_16_64!())) => (0x0faf, Modrm),
        (Or, Two(rm_16_64!(), r_16_64!())) => (0x09, Modrm),
        (And, Two(rm_16_64!(), r_16_64!())) => (0x21, Modrm),
        (Sar, Two(rm_16_64!(), R8(Reg::CL))) => (0xd3, Ext(7)),
        (Shr, Two(rm_16_64!(), R8(Reg::CL))) => (0xd3, Ext(5)),
        (Shr, Two(rm_16_64!(), Imm8(_))) => (0xc1, Ext(5)),
        (Sal, Two(rm_16_64!(), R8(Reg::CL))) => (0xd3, Ext(4)),
        (Sal, Two(rm_16_64!(), Imm8(_))) => (0xc1, Ext(4)),
        (Lea, Two(r_16_64!(), Mem(_))) => (0x8d, Modrm),
        (Cmp, Two(rm_8!(), R8(_))) => (0x38, Modrm),
        (Cmp, Two(rm_16_64!(), r_16_64!())) => (0x39, Modrm),
        (Cmp, Two(rm_16_64!(), Imm32(_))) => (0x81, Ext(7)),
        (Cmp, Two(rm_16_64!(), Imm8(_))) => (0x83, Ext(7)),
        (Neg, One(rm_16_64!())) => (0xf7, Ext(3)),
        (Not, One(rm_16_64!())) => (0xf7, Ext(2)),
        (Jne, One(Imm8(_))) => (0x75, ModrmType::None),
        (Jne, One(Imm32(_))) => (0x0f85, ModrmType::None),
        (Cmove, Two(r_16_64!(), rm_16_64!())) => (0x0f44, Modrm),
        (Cmovne, Two(r_16_64!(), rm_16_64!())) => (0x0f45, Modrm),
        (Cmovg, Two(r_16_64!(), rm_16_64!())) => (0x0f4f, Modrm),
        (Cmovge, Two(r_16_64!(), rm_16_64!())) => (0x0f4d, Modrm),
        (Cmovl, Two(r_16_64!(), rm_16_64!())) => (0x0f4c, Modrm),
        (Cmovle, Two(r_16_64!(), rm_16_64!())) => (0x0f4e, Modrm),
        (Call, One(imm!())) => (0xe8, ModrmType::None),
        (Jmp, One(Imm8(_))) => (0xeb, ModrmType::None),
        (Jmp, One(Imm32(_))) => (0xe9, ModrmType::None),
        (Jz, One(Imm32(_))) => (0x0f84, ModrmType::None),
        (Jz, One(Imm8(_))) => (0x74, ModrmType::None),
        (Test, Two(rm_16_64!(), r_16_64!())) => (0x85, Modrm),
        (Cqo, Oprs::None) => (0x4899, ModrmType::None),
        (Ret, Oprs::None) => (0xc3, ModrmType::None),
        (Inc, One(rm_16_64!())) => (0xff, Ext(0)),
        _ => unimplemented!("{instr}"),
    }
}
