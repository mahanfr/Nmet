#![allow(dead_code)]

#[macro_export]
macro_rules! st_visibility {
    ($val: expr) => {
        $val & 0x3
    };
}

#[macro_export]
macro_rules! st_info {
    ($bind: expr, $type: expr) => {
        ($bind << 4) | ($type & 0xf)
    };
}
// Legal values for ST_BIND subfield of st_info (symbol binding)
/// Local symbol
pub static STB_LOCAL: u8 = 0;
/// Global symbol
pub static STB_GLOBAL: u8 = 1;
/// Weak symbol
pub static STB_WEAK: u8 = 2;
/// Number of defined types.
pub static STB_NUM: u8 = 3;
/// Start of OS-specific
pub static STB_LOOS: u8 = 10;
/// Unique symbol.
pub static STB_GNU_UNIQUE: u8 = 10;
/// End of OS-specific
static STB_HIOS: u8 = 12;
/// Start of processor-specific
pub static STB_LOPROC: u8 = 13;
/// End of processor-specific
pub static STB_HIPROC: u8 = 15;

//Legal values for ST_TYPE subfield of st_info (symbol type)
/// Symbol type is unspecified
pub static STT_NOTYPE: u8 = 0;
/// Symbol is a data object
pub static STT_OBJECT: u8 = 1;
/// Symbol is a code object
pub static STT_FUNC: u8 = 2;
/// Symbol associated with a section
pub static STT_SECTION: u8 = 3;
/// Symbol's name is file name
pub static STT_FILE: u8 = 4;
/// Symbol is a common data object
pub static STT_COMMON: u8 = 5;
/// Symbol is thread-local data object*/
pub static STT_TLS: u8 = 6;
/// Number of defined types.
pub static STT_NUM: u8 = 7;
/// Start of OS-specific
pub static STT_LOOS: u8 = 10;
/// Symbol is indirect code object
pub static STT_GNU_IFUNC: u8 = 10;
/// End of OS-specific
pub static STT_HIOS: u8 = 12;
/// Start of processor-specific
pub static STT_LOPROC: u8 = 13;
/// End of processor-specific
pub static STT_HIPROC: u8 = 15;

// Symbol visibility specification encoded in the st_other field.
/// Default symbol visibility rules
pub static STV_DEFAULT: u8 = 0;
/// Processor specific hidden class
pub static STV_INTERNAL: u8 = 1;
/// Sym unavailable in other modules
pub static STV_HIDDEN: u8 = 2;
/// Not preemptible, not exported
pub static STV_PROTECTED: u8 = 3;

#[repr(u64)]
#[derive(Debug, Clone)]
pub enum SHFlags {
    Write = 0x1,
    Alloc = 0x2,
    Execinstr = 0x4,
    Merge = 0x10,
    Strings = 0x20,
    InfoLink = 0x40,
    LinkOrder = 0x80,
    OsNonconforming = 0x100,
    Group = 0x200,
    Tls = 0x400,
    Maskos = 0x0FF00000,
    Maskproc = 0xF0000000,
    Ordered = 0x4000000,
    Exclude = 0x8000000,
}

impl SHFlags {
    pub fn from_u64(flag: u64) -> Vec<Self> {
        let mut flags = Vec::new();
        if flag & Self::Write as u64 > 0 {
            flags.push(Self::Write)
        }
        if flag & Self::Alloc as u64 > 0 {
            flags.push(Self::Alloc)
        }
        if flag & Self::Execinstr as u64 > 0 {
            flags.push(Self::Execinstr)
        }
        if flag & Self::Merge as u64 > 0 {
            flags.push(Self::Merge)
        }
        if flag & Self::Strings as u64 > 0 {
            flags.push(Self::Strings)
        }
        if flag & Self::InfoLink as u64 > 0 {
            flags.push(Self::InfoLink)
        }
        if flag & Self::LinkOrder as u64 > 0 {
            flags.push(Self::LinkOrder)
        }
        if flag & Self::OsNonconforming as u64 > 0 {
            flags.push(Self::OsNonconforming)
        }
        if flag & Self::Group as u64 > 0 {
            flags.push(Self::Group)
        }
        if flag & Self::Tls as u64 > 0 {
            flags.push(Self::Tls)
        }
        if flag & Self::Maskos as u64 > 0 {
            flags.push(Self::Maskos)
        }
        if flag & Self::Maskproc as u64 > 0 {
            flags.push(Self::Maskproc)
        }
        if flag & Self::Ordered as u64 > 0 {
            flags.push(Self::Ordered)
        }
        if flag & Self::Exclude as u64 > 0 {
            flags.push(Self::Exclude)
        }
        flags
    }
}
