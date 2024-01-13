use crate::codegen::Opr::*;

use super::{mnemonic::Mnemonic::{self, *}, instructions::{Oprs::{self,*}, Instr, ModrmType::{self,*}}, register::Reg};

macro_rules! rm_8 { () => {R8(_) | Mem(_)}; }
macro_rules! rm_16_64 { () => {R64(_) | R32(_) | R16(_) | Mem(_)}; }
macro_rules! r_16_64 { () => {R64(_) | R32(_) | R16(_)}; }
macro_rules! r_16_32 { () => {R32(_) | R16(_)}; }

pub fn opcode(instr: &Instr) -> (u16, ModrmType) {
    match (instr.mnem, instr.oprs) {
        (Mnemonic::Add, Two(rm_16_64!(), r_16_64!())) => (0x01, Modrm),
        (Mov, Two(rm_8!(), R8(_))) => (0x88, Modrm),
        (Mov, Two(rm_16_64!(), r_16_64!())) => (0x89, Modrm),
        (Push, One(Imm8(_))) => (0x6A, ModrmType::None),
        (Pop, One(R64(_))) => (0x58, ModrmType::Add),
        (Sub, Two(rm_16_64!(), r_16_32!())) => (0x81, Ext(5)),
        (Idiv, One(R64(Reg::RDX))) => (0xf7, Ext(7)),
        (Syscall, Oprs::None) => (0x0f05, ModrmType::None),
        (Leave, Oprs::None) => (0xc9,ModrmType::None),
        (Nop, Oprs::None) => (0x90,ModrmType::None),
        (Mul, One(R64(Reg::RDX))) => (0xf7, Ext(4)),
        (Imul, Two(r_16_64!(), rm_16_64!())) => (0x0faf, Modrm),
        (Or, Two(rm_16_64!(), r_16_64!())) => (0x09, Modrm),
        (And, Two(rm_16_64!(), r_16_64!())) => (0x21, Modrm),
        (Sar, Two(rm_16_64!(), R8(Reg::CL))) => (0xd3, Ext(7)),
        (Shr, Two(rm_16_64!(), R8(Reg::CL))) => (0xd3, Ext(5)),
        (Sal, Two(rm_16_64!(), R8(Reg::CL))) => (0xd3, Ext(4)),
        (Lea, Two(r_16_64!(), Mem(_))) => (0x8d, Modrm),
        (Cmp, Two(rm_16_64!(), r_16_64!())) => (0x39, Modrm),
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
        (Call, One(Imm32(_))) => (0xe8, ModrmType::None),
        (Jmp, One(Imm8(_))) => (0xeb, ModrmType::None),
        (Jmp, One(Imm32(_))) => (0xe9, ModrmType::None),
        (Jz, One(Imm32(_))) => (0x0f84, ModrmType::None),
        (Jz, One(Imm8(_))) => (0x74, ModrmType::None),
        (Test, Two(rm_16_64!(), r_16_64!())) => (0x85, Modrm),
        (Cqo, Oprs::None) => (0x4899, ModrmType::None),
        _ => unimplemented!("{instr}"),
    }
}

