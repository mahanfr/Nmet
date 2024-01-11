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

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MemAddr {
    addr_type: MemAddrType,
    size: u8,
    register: Reg,
    disp: i32,
    s_register: Option<Reg>,
    scale: u8,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MemAddrType {
    Address,
    Disp,
    Sib,
}

impl MemAddr {

    fn validate_size(size: &u8) -> bool {
        matches!(size, 0 | 1 | 2 | 4 | 8)
    }

    fn validate_scale(scale: &u8) -> bool {
        matches!(scale, 1 | 2 | 4 | 8)
    }

    pub fn new(reg: Reg) -> Self {
        Self {
            addr_type: MemAddrType::Address,
            size: 0,
            register: reg,
            disp: 0,
            s_register: None,
            scale: 1,
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

    pub fn new_disp(reg: Reg, disp: i32) -> Self {
        let mut res = Self::new(reg);
        res.addr_type =  MemAddrType::Disp;
        res.disp = disp;
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
            addr_type: MemAddrType::Sib,
            size: 0,
            register: reg,
            disp,
            s_register: Some(reg_s),
            scale,
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

    fn mem_hint(size: &u8) -> &'static str {
        match size {
            0 => "",
            1 => "byte",
            2 => "word",
            4 => "dword",
            8 => "qword",
            _ => unreachable!(),
        }
    }
}

impl Display for MemAddr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut view = String::new();
        match self.size {
            1 | 2 | 4 | 8 => {
                view.push_str(Self::mem_hint(&self.size));
                view.push(' ');
            }
            0 => (),
            _ => unreachable!(),
        }
        view.push('[');
        view.push_str(self.register.to_string().as_str());
        if self.disp != 0 {
            if self.disp < 0 {
                view.push_str(" - ");
            } else {
                view.push_str(" + ");
            }
            view.push_str(self.disp.abs().to_string().as_str());
        }
        if let Some(reg) = self.s_register {
            view.push_str(" + ");
            view.push_str(reg.to_string().as_str());
            view.push_str(" * ");
            view.push_str(self.scale.to_string().as_str());
        }
        view.push(']');
        write!(f, "{view}")
    }
}

