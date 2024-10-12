use crate::utils::IBytes;

use super::{
    instructions::{Instr, ModrmType, Opr, Oprs},
    memory::{MemAddr, MemAddrType},
    mnemonic::Mnemonic,
    opcodes::opcode,
};

macro_rules! Register {
    ($a:ident) => {
        Opr::R64($a) | Opr::R32($a) | Opr::R16($a) | Opr::R8($a)
    };
}

pub fn assemble_instr(instr: &Instr) -> IBytes {
    let mut bytes = vec![];
    validate_opr_sizes(instr);
    let instr = align_imm_oprs_to_reg(instr);
    bytes.extend(rex(&instr));
    let (mut opcode, modrmtype) = opcode(&instr);
    let mut modrm_val = Vec::<u8>::new();
    match modrmtype {
        ModrmType::Add => match instr.oprs {
            Oprs::One(Register!(r)) | Oprs::Two(Register!(r), _) => {
                opcode += r.opcode() as u16;
            }
            _ => unreachable!("{instr}: Unecxpected behavior!"),
        },
        ModrmType::Modrm => {
            modrm_val = modrm(&instr);
        }
        ModrmType::Ext(ex) => {
            modrm_val = modrm_ex(ex, &instr.oprs);
        }
        ModrmType::None => (),
    }
    if opcode <= 0xff {
        bytes.push(opcode.to_le_bytes()[0]);
    } else {
        bytes.extend(opcode.to_be_bytes());
    }
    bytes.extend(modrm_val);
    include_imm_values(&mut bytes, &instr);
    bytes
}

fn include_imm_values(bytes: &mut IBytes, instr: &Instr) {
    if instr.mnem.needs_precision_imm() {
        match &instr.oprs {
            Oprs::Two(Opr::Mem(m), Opr::Imm8(val) | Opr::Imm32(val)) => match m.size {
                1 => {
                    bytes.push(val.to_le_bytes()[0]);
                }
                2 | 4 | 8 => {
                    bytes.extend(val.to_le_bytes().iter().take(4));
                }
                _ => unreachable!(),
            },
            Oprs::Two(
                Opr::R64(r) | Opr::R32(r) | Opr::R16(r) | Opr::R8(r),
                Opr::Imm8(val) | Opr::Imm32(val) | Opr::Imm64(val),
            ) => {
                bytes.extend(val.to_le_bytes().iter().take((r.size() / 8) as usize));
            }
            _ => (),
        }
    } else {
        match instr.oprs {
            Oprs::Two(_, Opr::Imm8(val)) | Oprs::One(Opr::Imm8(val)) => {
                bytes.extend(val.to_le_bytes().iter().take(1));
            }
            Oprs::Two(_, Opr::Imm32(val)) | Oprs::One(Opr::Imm32(val)) => {
                bytes.extend(val.to_le_bytes().iter().take(4));
            }
            Oprs::Two(_, Opr::Imm64(val)) | Oprs::One(Opr::Imm64(val)) => {
                bytes.extend(val.to_le_bytes().iter().take(8));
            }
            _ => (),
        }
    }
}

fn rex(instr: &Instr) -> IBytes {
    match &instr.oprs {
        Oprs::Two(Register!(r1), Register!(r2)) => {
            let mut bytes = vec![];
            let mut rex: u8 = 0x40;
            if r1.is_extended() {
                rex |= 0b0100;
            }
            if r2.is_extended() {
                rex |= 0b0001;
            }
            if r1.size() == 64 {
                rex |= 0b1000;
            }
            if r1.size() == 16 {
                bytes.push(0x66);
            }
            if rex != 0x40 || r1.is_new_8bit_reg() || r2.is_new_8bit_reg() {
                bytes.push(rex);
            }
            bytes
        }
        Oprs::Two(Register!(r1), Opr::Mem(mem)) | Oprs::Two(Opr::Mem(mem), Register!(r1)) => {
            let mut bytes = vec![];
            let mut rex: u8 = 0x40;
            if r1.is_extended() {
                rex |= 0b0100;
            }
            if mem.get_register().is_extended() {
                rex |= 0b0001;
            }
            if let Some(s_reg) = mem.get_s_register() {
                if s_reg.is_extended() {
                    rex |= 0b0010;
                }
            };
            if r1.size() == 64 {
                rex |= 0b1000;
            }
            if r1.size() == 16 {
                bytes.push(0x66);
            }
            if rex != 0x40 {
                bytes.push(rex);
            }
            bytes
        }
        Oprs::Two(Register!(r), _) => {
            let mut bytes = vec![];
            let mut rex: u8 = 0x40;
            if r.is_extended() {
                rex |= 0b0100;
            }
            if r.size() == 64 {
                rex |= 0b1000;
            }
            if r.size() == 16 {
                bytes.push(0x66);
            }
            if rex != 0x40 || r.is_new_8bit_reg() {
                bytes.push(rex);
            }
            bytes
        }
        Oprs::Two(Opr::Mem(mem), _) | Oprs::One(Opr::Mem(mem)) => {
            let mut bytes = vec![];
            let mut rex: u8 = 0x40;
            if mem.get_register().is_extended() {
                rex |= 0b0100;
            }
            if let Some(s_reg) = mem.get_s_register() {
                if s_reg.is_extended() {
                    rex |= 0b0010;
                }
            };
            if mem.size * 8 == 64 {
                rex |= 0b1000;
            }
            if mem.size * 8 == 16 {
                bytes.push(0x66);
            }
            if rex != 0x40 {
                bytes.push(rex);
            }
            bytes
        }
        Oprs::One(Register!(r)) => {
            let mut rex: u8 = 0x40;
            if r.is_extended() {
                rex |= 0b0100;
            }
            if instr.mnem != Mnemonic::Push && instr.mnem != Mnemonic::Pop && r.size() == 64 {
                rex |= 0b1000;
            }
            if rex != 0x40 {
                vec![rex]
            } else {
                vec![]
            }
        }
        Oprs::None => vec![],
        _ => vec![],
    }
}

fn validate_opr_sizes(instr: &Instr) -> usize {
    if let Oprs::Two(op1, op2) = &instr.oprs {
        let mut lhs_size;
        let rhs_size;
        match op1 {
            Opr::R64(r) | Opr::R32(r) | Opr::R16(r) | Opr::R8(r) => {
                lhs_size = r.size();
            }
            Opr::Mem(mem) => {
                lhs_size = mem.size * 8;
            }
            Opr::Imm8(_) | Opr::Imm32(_) | Opr::Imm64(_) => {
                panic!("Error: First opr for instr ({instr}) can not be an Immidiate value!");
            }
            Opr::Rela(_) | Opr::Loc(_) => unreachable!(),
        }
        match op2 {
            Opr::R64(r) | Opr::R32(r) | Opr::R16(r) | Opr::R8(r) => {
                rhs_size = r.size();
                if lhs_size == 0 {
                    lhs_size = rhs_size;
                }
            }
            Opr::Mem(mem) => {
                if mem.size != 0 {
                    rhs_size = mem.size * 8;
                } else {
                    rhs_size = lhs_size;
                }
            }
            Opr::Imm8(_) | Opr::Imm32(_) | Opr::Imm64(_) => {
                if lhs_size != 0 {
                    rhs_size = lhs_size;
                } else {
                    panic!("Error: oprand size is unknown for instr ({instr})!");
                }
            }
            Opr::Rela(_) | Opr::Loc(_) => unreachable!(),
        }
        if rhs_size == 0 || lhs_size == 0 {
            panic!("Error: oprand size is unknown for instr ({instr})!");
        }
        lhs_size as usize
    } else {
        0
    }
}

fn align_imm_oprs_to_reg(instr: &Instr) -> Instr {
    match (&instr.mnem, &instr.oprs) {
        (Mnemonic::Mov, Oprs::Two(Opr::R64(r), Opr::Imm32(val) | Opr::Imm8(val))) => {
            Instr::new2(Mnemonic::Mov, Opr::R32(r.convert(4)), Opr::Imm32(*val))
        }
        _ => instr.clone(),
    }
}

fn modrm(instr: &Instr) -> IBytes {
    match &instr.oprs {
        Oprs::Two(Register!(r1), Register!(r2)) => {
            if !instr.mnem.reverse_modrm() {
                vec![_modrm(0b11, r1.opcode(), r2.opcode())]
            } else {
                vec![_modrm(0b11, r2.opcode(), r1.opcode())]
            }
        }
        Oprs::Two(Register!(r), Opr::Mem(mem)) | Oprs::Two(Opr::Mem(mem), Register!(r)) => {
            _mem_modrm(r.opcode(), mem)
        }
        Oprs::Two(Opr::Mem(mem), _) | Oprs::One(Opr::Mem(mem)) => _mem_modrm(0, mem),
        _ => unreachable!(),
    }
}

fn modrm_ex(ex: u8, oprs: &Oprs) -> IBytes {
    match oprs {
        Oprs::Two(Register!(r1), _) | Oprs::One(Register!(r1)) => {
            vec![_modrm(0b11, r1.opcode(), ex)]
        }
        Oprs::Two(Opr::Mem(mem), _) | Oprs::One(Opr::Mem(mem)) => _mem_modrm(ex, mem),
        _ => unreachable!(),
    }
}

fn _mem_modrm(r: u8, mem: &MemAddr) -> IBytes {
    match mem.addr_type {
        MemAddrType::Addr(reg) => {
            if reg.opcode() != 0x04 && reg.opcode() != 0x05 {
                vec![_modrm(0b00,reg.opcode(), r)]
            } else {
                unreachable!();
            }
        }
        MemAddrType::Disp(reg, disp) => {
            if disp >= i8::MIN as i32 && disp <= i8::MAX as i32 {
                if reg.opcode() != 0x4 {
                    let disp_byte = disp.to_le_bytes()[0];
                    vec![_modrm(0b01, reg.opcode(), r), disp_byte]
                } else {
                    unreachable!();
                }
            } else if reg.opcode() != 0x4 {
                let mut bytes = vec![_modrm(0b10, reg.opcode(), r)];
                bytes.extend(disp.to_le_bytes());
                bytes
            } else {
                unreachable!();
            }
        }
        MemAddrType::Sib(reg1, disp, reg2, scale) => {
            let mut bytes = vec![];
            if disp == 0 {
                bytes.push(_modrm(0b00, 0x04, r));
                bytes
            } else if disp >= i8::MIN as i32 && disp <= i8::MAX as i32 {
                bytes.push(_modrm(0b01, 0x04, r));
                bytes.push(sib(mem));
                bytes.push(disp.to_le_bytes()[0]);
                bytes
            } else {
                bytes.push(_modrm(0b10, 0x04, r));
                bytes.push(sib(mem));
                bytes.extend(disp.to_le_bytes());
                bytes
            }
        }
        MemAddrType::AddrRela(_) => todo!(),
    }
}

fn _modrm(modr: u8, r1: u8, r2: u8) -> u8 {
    // ((((modr & 0b11) << 3) | (r1.upcode32() & 0x07)) << 3) | (r2.upcode32() & 0x07)
    let mut res = modr & 0b11;
    res <<= 3;
    res |= r2 & 0b111;
    res <<= 3;
    res |= r1 & 0b111;
    res
}

fn sib(mem: &MemAddr) -> u8 {
    let MemAddrType::Sib(register, _, s_register, scale) = mem.clone().addr_type else {
        unreachable!("expected reg in sib founnd none!");
    };
    let mut res = (scale.trailing_zeros() & 0b11) as u8;
    res <<= 3;
    res |= s_register.opcode() & 0b111;
    res <<= 3;
    res |= register.opcode() & 0b111;
    res
}
