use std::str::FromStr;

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
    Call,
    Jmp,
    Jz,
    Jnz,
    Jne,
    Syscall,
    Leave,
    Ret,
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
