use std::fmt::Display;

#[allow(dead_code)]
#[derive(Debug,Clone, Copy)]
pub enum R {
    RAX,
    RCX,
    RDX,
    RBX,
    RSP,
    RBP,
    RSI,
    RDI,
    EAX,
    ECX,
    EDX,
    EBX,
    ESP,
    EBP,
    ESI,
    EDI,
    AX,
    CX,
    DX,
    BX,
    SP,
    BP,
    SI,
    DI,
    AH,
    AL,
    CH,
    CL,
    DH,
    DL,
    BH,
    BL,
    SPL,
    BPL,
    SIL,
    DIL,
}
impl Display for R {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RAX => write!(f,"rax"),
            Self::RCX => write!(f,"rcx"),
            Self::RDX => write!(f,"rdx"),
            Self::RBX => write!(f,"rbx"),
            Self::RSP => write!(f,"rsp"),
            Self::RBP => write!(f,"rbp"),
            Self::RSI => write!(f,"rsi"),
            Self::RDI => write!(f,"rdi"),
            Self::EAX => write!(f,"eax"),
            Self::ECX => write!(f,"ecx"),
            Self::EDX => write!(f,"edx"),
            Self::EBX => write!(f,"ebx"),
            Self::ESP => write!(f,"esp"),
            Self::EBP => write!(f,"ebp"),
            Self::ESI => write!(f,"esi"),
            Self::EDI => write!(f,"edi"),
            Self::AX  => write!(f,"ax"),
            Self::CX  => write!(f,"cx"),
            Self::DX  => write!(f,"dx"),
            Self::BX  => write!(f,"bx"),
            Self::SP  => write!(f,"sp"),
            Self::BP  => write!(f,"bp"),
            Self::SI  => write!(f,"si"),
            Self::DI  => write!(f,"di"),
            Self::AH  => write!(f,"ah"),
            Self::AL  => write!(f,"al"),
            Self::CH  => write!(f,"ch"),
            Self::CL  => write!(f,"cl"),
            Self::DH  => write!(f,"dh"),
            Self::DL  => write!(f,"dl"),
            Self::BH  => write!(f,"bh"),
            Self::BL  => write!(f,"bl"),
            Self::SPL => write!(f,"spl"),
            Self::BPL => write!(f,"bpl"),
            Self::SIL => write!(f,"sil"),
            Self::DIL => write!(f,"dil"),
        }
    }
}

#[allow(dead_code)]
#[derive(Clone)]
pub struct Codegen {
    pub instruct_buf: Vec<String>,
    pub data_buf: Vec<String>,
    pub bss_buf: Vec<String>,
}

static SPACING: &str = "    ";

#[allow(dead_code)]
impl Codegen {
    pub fn new() -> Self {
        Self {
            instruct_buf: Vec::new(),
            bss_buf: Vec::new(),
            data_buf: Vec::new(),
        }
    }

    pub fn get_id(&mut self) -> usize {
        self.instruct_buf.len()
    }

    pub fn add_data_seg(&mut self, data: impl ToString, _size: usize) -> u64 {
        let id = self.data_buf.len();
        self.data_buf
            .push(format!("data{id} db {}", data.to_string()));
        self.data_buf.push(format!("len{id} equ $ - data{id}"));
        id as u64
    }

    pub fn add_bss_seg(&mut self, size: usize) -> String {
        let bss_tag = format!("arr{}", self.bss_buf.len());
        self.bss_buf.push(format!("{}: resb {}", bss_tag, size));
        bss_tag
    }

    pub fn place_holder(&mut self) -> usize {
        self.instruct_buf.push(String::new());
        self.instruct_buf.len() - 1
    }

    pub fn insert_raw(&mut self, instr: String) {
        self.instruct_buf.push(instr);
    }

    pub fn insert_into_raw(&mut self, index: usize, instr: String) -> Result<(), String> {
        if index < self.instruct_buf.len() - 1 {
            self.instruct_buf[index] = instr;
            Ok(())
        } else {
            Err("index out of bounds!".into())
        }
    }

    pub fn mov(&mut self, d1: impl ToString, d2: impl ToString) {
        self.instruct_buf.push(format!(
            "{SPACING}mov {}, {}",
            d1.to_string(),
            d2.to_string()
        ));
    }
    pub fn cmove(&mut self, d1: impl ToString, d2: impl ToString) {
        self.instruct_buf.push(format!(
            "{SPACING}cmove {}, {}",
            d1.to_string(),
            d2.to_string()
        ));
    }
    pub fn cmovne(&mut self, d1: impl ToString, d2: impl ToString) {
        self.instruct_buf.push(format!(
            "{SPACING}cmovne {}, {}",
            d1.to_string(),
            d2.to_string()
        ));
    }
    pub fn cmovg(&mut self, d1: impl ToString, d2: impl ToString) {
        self.instruct_buf.push(format!(
            "{SPACING}cmovg {}, {}",
            d1.to_string(),
            d2.to_string()
        ));
    }
    pub fn cmovl(&mut self, d1: impl ToString, d2: impl ToString) {
        self.instruct_buf.push(format!(
            "{SPACING}cmovl {}, {}",
            d1.to_string(),
            d2.to_string()
        ));
    }
    pub fn cmovge(&mut self, d1: impl ToString, d2: impl ToString) {
        self.instruct_buf.push(format!(
            "{SPACING}cmovge {}, {}",
            d1.to_string(),
            d2.to_string()
        ));
    }
    pub fn cmovle(&mut self, d1: impl ToString, d2: impl ToString) {
        self.instruct_buf.push(format!(
            "{SPACING}cmovle {}, {}",
            d1.to_string(),
            d2.to_string()
        ));
    }
    pub fn push(&mut self, d1: impl ToString) {
        self.instruct_buf
            .push(format!("{SPACING}push {}", d1.to_string()));
    }
    pub fn pop(&mut self, d1: impl ToString) {
        self.instruct_buf
            .push(format!("{SPACING}pop {}", d1.to_string()));
    }
    pub fn add(&mut self, d1: impl ToString, d2: impl ToString) {
        self.instruct_buf.push(format!(
            "{SPACING}add {}, {}",
            d1.to_string(),
            d2.to_string()
        ));
    }
    pub fn sub(&mut self, d1: impl ToString, d2: impl ToString) {
        self.instruct_buf.push(format!(
            "{SPACING}sub {}, {}",
            d1.to_string(),
            d2.to_string()
        ));
    }
    pub fn imul(&mut self, d1: impl ToString, d2: impl ToString) {
        self.instruct_buf.push(format!(
            "{SPACING}imul {}, {}",
            d1.to_string(),
            d2.to_string()
        ));
    }
    pub fn idiv(&mut self, d1: impl ToString) {
        self.instruct_buf
            .push(format!("{SPACING}idiv {}", d1.to_string()));
    }
    pub fn or(&mut self, d1: impl ToString, d2: impl ToString) {
        self.instruct_buf.push(format!(
            "{SPACING}or {}, {}",
            d1.to_string(),
            d2.to_string()
        ));
    }
    pub fn and(&mut self, d1: impl ToString, d2: impl ToString) {
        self.instruct_buf.push(format!(
            "{SPACING}and {}, {}",
            d1.to_string(),
            d2.to_string()
        ));
    }
    pub fn sal(&mut self, d1: impl ToString, d2: impl ToString) {
        self.instruct_buf.push(format!(
            "{SPACING}sal {}, {}",
            d1.to_string(),
            d2.to_string()
        ));
    }
    pub fn sar(&mut self, d1: impl ToString, d2: impl ToString) {
        self.instruct_buf.push(format!(
            "{SPACING}sar {}, {}",
            d1.to_string(),
            d2.to_string()
        ));
    }
    pub fn cmp(&mut self, d1: impl ToString, d2: impl ToString) {
        self.instruct_buf.push(format!(
            "{SPACING}cmp {}, {}",
            d1.to_string(),
            d2.to_string()
        ));
    }
    pub fn test(&mut self, d1: impl ToString, d2: impl ToString) {
        self.instruct_buf.push(format!(
            "{SPACING}test {}, {}",
            d1.to_string(),
            d2.to_string()
        ));
    }
    pub fn cqo(&mut self) {
        self.instruct_buf.push(format!("{SPACING}cqo"));
    }
    pub fn neg(&mut self, d1: impl ToString) {
        self.instruct_buf
            .push(format!("{SPACING}neg {}", d1.to_string()));
    }
    pub fn not(&mut self, d1: impl ToString) {
        self.instruct_buf
            .push(format!("{SPACING}not {}", d1.to_string()));
    }
    pub fn call(&mut self, d1: impl ToString) {
        self.instruct_buf
            .push(format!("{SPACING}call {}", d1.to_string()));
    }
    pub fn tag(&mut self, tag: impl ToString) {
        self.instruct_buf.push(format!("{}:", tag.to_string()));
    }
    pub fn jmp(&mut self, tag: impl ToString) {
        self.instruct_buf
            .push(format!("{SPACING}jmp {}", tag.to_string()));
    }
    pub fn jz(&mut self, tag: impl ToString) {
        self.instruct_buf
            .push(format!("{SPACING}jz {}", tag.to_string()));
    }
    pub fn jnz(&mut self, tag: impl ToString) {
        self.instruct_buf
            .push(format!("{SPACING}jnz {}", tag.to_string()));
    }
    pub fn syscall(&mut self) {
        self.instruct_buf.push(format!("{SPACING}syscall"));
    }
    pub fn leave(&mut self) {
        self.instruct_buf.push(format!("{SPACING}leave"));
    }
    pub fn ret(&mut self) {
        self.instruct_buf.push(format!("{SPACING}ret"));
    }
}
