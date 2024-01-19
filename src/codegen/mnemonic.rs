use std::{fmt::Display, str::FromStr};

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mnemonic {
    Lea,
    Mov,
    Cmove,
    Cmovne,
    Cmovg,
    Cmovl,
    Cmovge,
    Cmovle,
    Push,
    Pop,
    Add,
    Sub,
    Imul,
    Idiv,
    Mul,
    Or,
    And,
    Sal,
    Sar,
    Shr,
    Cmp,
    Test,
    Cqo,
    Neg,
    Not,
    Nop,
    Call,
    Jmp,
    Jz,
    Jnz,
    Jne,
    Syscall,
    Leave,
    Ret,
    Lable,
}

impl Mnemonic {
    pub fn needs_precision_imm(&self) -> bool {
        matches!(self, Self::Mov)
    }
}

impl Display for Mnemonic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Lea => write!(f, "lea"),
            Self::Mov => write!(f, "mov"),
            Self::Cmove => write!(f, "cmove"),
            Self::Cmovne => write!(f, "cmovne"),
            Self::Cmovg => write!(f, "cmovg"),
            Self::Cmovl => write!(f, "cmovl"),
            Self::Cmovge => write!(f, "cmovge"),
            Self::Cmovle => write!(f, "cmovle"),
            Self::Push => write!(f, "push"),
            Self::Pop => write!(f, "pop"),
            Self::Add => write!(f, "add"),
            Self::Sub => write!(f, "sub"),
            Self::Imul => write!(f, "imul"),
            Self::Idiv => write!(f, "idiv"),
            Self::Mul => write!(f, "mul"),
            Self::Or => write!(f, "or"),
            Self::And => write!(f, "and"),
            Self::Sal => write!(f, "sal"),
            Self::Sar => write!(f, "sar"),
            Self::Shr => write!(f, "shr"),
            Self::Cmp => write!(f, "cmp"),
            Self::Test => write!(f, "test"),
            Self::Cqo => write!(f, "cqo"),
            Self::Neg => write!(f, "neg"),
            Self::Not => write!(f, "not"),
            Self::Call => write!(f, "call"),
            Self::Jmp => write!(f, "jmp"),
            Self::Jz => write!(f, "jz"),
            Self::Jnz => write!(f, "jnz"),
            Self::Jne => write!(f, "jne"),
            Self::Syscall => write!(f, "syscall"),
            Self::Leave => write!(f, "leave"),
            Self::Ret => write!(f, "ret"),
            Self::Nop => write!(f, "nop"),
            Self::Lable => write!(f, ""),
        }
    }
}

impl FromStr for Mnemonic {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let clean_s = s.to_string().to_lowercase();
        match clean_s.trim() {
            "lea" => Ok(Self::Lea),
            "mov" => Ok(Self::Mov),
            "cmove" => Ok(Self::Cmove),
            "cmovne" => Ok(Self::Cmovne),
            "cmovg" => Ok(Self::Cmovg),
            "cmovl" => Ok(Self::Cmovl),
            "cmovge" => Ok(Self::Cmovge),
            "cmovle" => Ok(Self::Cmovle),
            "push" => Ok(Self::Push),
            "pop" => Ok(Self::Pop),
            "add" => Ok(Self::Add),
            "sub" => Ok(Self::Sub),
            "imul" => Ok(Self::Imul),
            "idiv" => Ok(Self::Idiv),
            "mul" => Ok(Self::Mul),
            "or" => Ok(Self::Or),
            "and" => Ok(Self::And),
            "sal" => Ok(Self::Sal),
            "sar" => Ok(Self::Sar),
            "shr" => Ok(Self::Shr),
            "cmp" => Ok(Self::Cmp),
            "test" => Ok(Self::Test),
            "cqo" => Ok(Self::Cqo),
            "neg" => Ok(Self::Neg),
            "not" => Ok(Self::Not),
            "jmp" => Ok(Self::Jmp),
            "jz" => Ok(Self::Jz),
            "jnz" => Ok(Self::Jnz),
            "jne" => Ok(Self::Jne),
            "syscall" => Ok(Self::Syscall),
            "call" => Ok(Self::Call),
            "leave" => Ok(Self::Leave),
            "ret" => Ok(Self::Ret),
            _ => Err(format!("Undifiend Mnemonic {clean_s}!")),
        }
    }
}
