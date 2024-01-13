use super::instructions::{Instr, Oprs, Opr};

pub fn assemble_instr(instr: &Instr) -> Vec<u8> {
    let mut bytes = vec![];
    bytes.extend(rex(&instr));
    bytes
}

fn rex(instr: &Instr) -> Vec<u8> {
    match instr.oprs {
        Oprs::Two(Opr::R64(r1) | Opr::R32(r1) | Opr::R16(r1) | Opr::R8(r1), 
                  Opr::R64(r2) | Opr::R32(r2) | Opr::R16(r2) | Opr::R8(r2)) => {
            let mut bytes = vec![];
            let mut rex: u8 = 0x40;
            if r1.is_extended() { rex |= 0b0100; }
            if r2.is_extended() { rex |= 0b0001; }
            if r1.size() == 64 { rex |= 0b1000; }
            if r1.size() == 16 { bytes.push(0x66); }
            if rex != 0x40 || r1.is_new_8bit_reg() || r2.is_new_8bit_reg() {
                bytes.push(rex);
            }
            bytes
        },
        Oprs::Two(Opr::R64(r1) | Opr::R32(r1) | Opr::R16(r1) | Opr::R8(r1), Opr::Mem(mem))
            | Oprs::Two(Opr::Mem(mem), Opr::R64(r1) | Opr::R32(r1) | Opr::R16(r1) | Opr::R8(r1)) => { 
            let mut bytes = vec![];
            let mut rex: u8 = 0x40;
            if r1.is_extended() { rex |= 0b0100; }
            if mem.register.is_extended() { rex |= 0b0001; }
            if let Some(s_reg) = mem.s_register {
                if s_reg.is_extended() {
                    rex |= 0b0010;
                }
            };
            if r1.size() == 64 { rex |= 0b1000; }
            if r1.size() == 16 { bytes.push(0x66); }
            if rex != 0x40 {
                bytes.push(rex);
            }
            bytes
        },
        Oprs::Two(Opr::R64(r) | Opr::R32(r)| Opr::R16(r) | Opr::R8(r), _) => {
            let mut bytes = vec![];
            let mut rex: u8 = 0x40;
            if r.is_extended() { rex |= 0b0100; }
            if r.size() == 64 { rex |= 0b1000; }
            if r.size() == 16 { bytes.push(0x66); }
            if rex != 0x40 || r.is_new_8bit_reg() {
                bytes.push(rex);
            }
            bytes
        },
        Oprs::Two(Opr::Mem(mem), _) | 
            Oprs::One(Opr::Mem(mem)) => {
                let mut bytes = vec![];
                let mut rex: u8 = 0x40;
                if mem.register.is_extended() { rex |= 0b0100; }
                if let Some(s_reg) = mem.s_register {
                    if s_reg.is_extended() {
                        rex |= 0b0010;
                    }
                };
                if mem.size * 8 == 64 { rex |= 0b1000; }
                if mem.size * 8 == 16 { bytes.push(0x66); }
                if rex != 0x40 {
                    bytes.push(rex);
                }
                bytes
            },
        Oprs::One(Opr::R64(r) | Opr::R32(r)| Opr::R16(r) | Opr::R8(r)) => {
            let mut rex: u8 = 0x40;
            if r.is_extended() { rex |= 0b0100; }
            if rex != 0x40 {
                vec![rex];
            }
            vec![]
        }
        Oprs::None => vec![],
        _ => vec![],
    }
}
