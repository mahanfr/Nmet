use std::fmt::Display;

use super::register::Reg;

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

#[derive(Debug, Clone, PartialEq)]
pub struct MemAddr {
    pub addr_type: MemAddrType,
    pub size: u8,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MemAddrType {
    Addr(Reg),
    AddrRela(String),
    Disp(Reg, i32),
    Sib(Reg, i32, Reg, u8),
}

impl MemAddr {
    fn validate_size(size: &u8) -> bool {
        matches!(size, 0 | 1 | 2 | 4 | 8)
    }

    fn validate_scale(scale: &u8) -> bool {
        matches!(scale, 1 | 2 | 4 | 8)
    }

    pub fn is_rela(&self) -> bool {
        matches!(self.addr_type, MemAddrType::AddrRela(_))
    }

    pub fn new(reg: Reg) -> Self {
        Self {
            addr_type: MemAddrType::Addr(reg),
            size: 0,
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

    pub fn new_rela(rela: String) -> Self {
        Self {
            addr_type: MemAddrType::AddrRela(rela),
            size: 0,
        }
    }

    pub fn new_rela_s(size: u8, rela: String) -> Self {
        if !Self::validate_size(&size) {
            panic!("unexpected value for memory size");
        }
        Self {
            addr_type: MemAddrType::AddrRela(rela),
            size,
        }
    }

    pub fn new_disp(reg: Reg, disp: i32) -> Self {
        let mut res = Self::new(reg);
        res.addr_type = MemAddrType::Disp(reg, disp);
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
            addr_type: MemAddrType::Sib(reg, disp, reg_s, scale),
            size: 0,
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

    pub fn get_register(&self) -> Reg {
        match self.addr_type {
            MemAddrType::Addr(r) => r,
            MemAddrType::Disp(r, _) => r,
            MemAddrType::Sib(r, _, _, _) => r,
            _ => unreachable!(),
        }
    }

    pub fn get_s_register(&self) -> Option<Reg> {
        match self.addr_type {
            MemAddrType::Sib(_, _, r, _) => Some(r),
            _ => None,
        }
    }

    fn mem_hint(size: &u8) -> &'static str {
        match size {
            0 => "",
            1 => "byte ",
            2 => "word ",
            4 => "dword ",
            8 => "qword ",
            _ => unreachable!(),
        }
    }

    fn internsic(disp: i32) -> &'static str {
        if disp < 0 {
            " - "
        } else {
            " + "
        }
    }
}

impl Display for MemAddr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let intern = match &self.addr_type {
            MemAddrType::Addr(r) => format!("[{r}]"),
            MemAddrType::Disp(r, disp) => format!(
                "[{r}{}{}]",
                Self::internsic(*disp),
                disp.abs().to_string().as_str()
            ),
            MemAddrType::Sib(r, disp, r2, scale) => format!(
                "[{r}{}{} + {r2} * {scale}]",
                Self::internsic(*disp),
                disp.abs().to_string().as_str()
            ),
            MemAddrType::AddrRela(rel) => format!("[{rel}]"),
        };
        write!(f, "{}{intern}", Self::mem_hint(&self.size))
    }
}
