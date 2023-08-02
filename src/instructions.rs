pub enum IData {
    A,
    B,
    C,
    D,
    SP,
    BP,
    SI,
    DI,
    R8,
    R9,
    R10,
    R11,
    GMEM(usize),
    SMEM(usize),
}
impl IData {
    fn set(self,size: usize) -> String {
        match self {
            Self::A | Self::B | Self::C | Self::D => {
                let ch = match self {
                    Self::A => 'a',
                    Self::B => 'b',
                    Self::C => 'c',
                    Self::D => 'd',
                    _ => {
                        unreachable!("Compiler Error");
                    }
                };
                match size {
                    1 => format!("{ch}l"),
                    2 => format!("{ch}x"),
                    4 => format!("e{ch}x"),
                    8 => format!("r{ch}x"),
                    _ => {
                        unreachable!("Incurrect Size")
                    }
                }
            },
            Self::SP | Self::BP  => {
                let ch = match self {
                    Self::SP => 's',
                    Self::BP => 'b',
                    _ => {
                        unreachable!("Compiler Error");
                    }
                };
                match size {
                    1 => format!("{ch}pl"),
                    2 => format!("{ch}p"),
                    4 => format!("e{ch}p"),
                    8 => format!("r{ch}p"),
                    _ => {
                        unreachable!("Incurrect Size")
                    }
                }
            },
            Self::SI | Self::DI  => {
                let ch = match self {
                    Self::SI => 's',
                    Self::DI => 'd',
                    _ => {
                        unreachable!("Compiler Error");
                    }
                };
                match size {
                    1 => format!("{ch}il"),
                    2 => format!("{ch}i"),
                    4 => format!("e{ch}i"),
                    8 => format!("r{ch}i"),
                    _ => {
                        unreachable!("Incurrect Size")
                    }
                }
            },
            Self::R8 | Self::R9 | Self::R10 | Self::R11 => {
                let ch = match self {
                    Self::R8  => "r8",
                    Self::R9  => "r9",
                    Self::R10 => "r10",
                    Self::R11 => "r11",
                    _ => {
                        unreachable!("Compiler Error");
                    }
                };
                match size {
                    1 => format!("{ch}b"),
                    2 => format!("{ch}w"),
                    4 => format!("{ch}d"),
                    8 => format!("{ch}"),
                    _ => {
                        unreachable!("Incurrect Size")
                    }
                }
            },
            Self::GMEM(_) | Self::SMEM(_) => {
                let addr = match self {
                    Self::SMEM(offset) => format!("[rsp-{offset}]"),
                    Self::GMEM(offset) => format!("[mem-{offset}]"),
                    _ => {
                        unreachable!("Compiler Error");
                    }
                };
                match size {
                    1 => format!("byte {addr}"),
                    2 => format!("word {addr}"),
                    4 => format!("dword {addr}"),
                    8 => format!("qword {addr}"),
                    _ => {
                        unreachable!("Incurrect Size")
                    }
                }
            }
        }
    }
}

pub enum Instruction {
    JMP(String),
    JZ(String),
    JNZ(String),
    PUSH(IData),
    POP(IData),
    MOV(IData, IData),
    ADD(IData, IData),
    SUB(IData, IData),
    IMUL(IData, IData),
    CMP(IData, IData),
    TEST(IData, IData),

    TAG(String),
    CALL(String),
    SYSCALL,
    CMOVE(IData, IData),
    CMOVNE (IData, IData),
    CMOVG(IData, IData),
    CMOVL(IData, IData),
    CMOVGE(IData, IData),
    CMOVLE(IData, IData),
}
impl Instruction {
    fn set(self,size: usize) -> String {
        match self {
            Self::JMP(tag) => {
                asm!("jmp {tag}")
            }
            Self::JZ(tag) => {
                asm!("jz {tag}")
            }
            Self::JNZ(tag) => {
                asm!("jnz {tag}")
            },
            Self::TAG(ident) => {
                asm!("{ident}:")
            },
            Self::CALL(ident) => {
                asm!("call {ident}")
            },
            Self::SYSCALL => {
                asm!("syscall")
            }
            
        }
    }
}

