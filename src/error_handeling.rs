/**********************************************************************************************
*
*   error_handeling: Structure for string code locations and error handeling
*
*   LICENSE: MIT
*
*   Copyright (c) 2023-2024 Mahan Farzaneh (@mahanfr)
*
*   This software is provided "as-is", without any express or implied warranty. In no event
*   will the authors be held liable for any damages arising from the use of this software.
*
*   Permission is granted to anyone to use this software for any purpose, including commercial
*   applications, and to alter it and redistribute it freely, subject to the following restrictions:
*
*     1. The origin of this software must not be misrepresented; you must not claim that you
*     wrote the original software. If you use this software in a product, an acknowledgment
*     in the product documentation would be appreciated but is not required.
*
*     2. Altered source versions must be plainly marked as such, and must not be misrepresented
*     as being the original software.
*
*     3. This notice may not be removed or altered from any source distribution.
*
**********************************************************************************************/
use std::{error::Error, fmt::Display, process::exit};

use crate::parser::{expr::Op, types::VariableType};

#[derive(Debug)]
pub enum CompilationError {
    UndefinedVariable(String),
    UndefinedNameSpace(String),
    UnknownType(String),
    UnexpectedType(String),
    InvalidTypeCasting(String, String),
    InValidBinaryOperation(Op, String, String),
    FunctionOutOfScope(String),
    InvalidInlineAsm(String),
    ImmutableVariable(String),
    UnmatchingTypes(VariableType, VariableType),
    NotLoopBlock,
    Err(String),
}
impl Display for CompilationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UndefinedVariable(v) => write!(f,"Undifiend Variable ({v})"),
            Self::UndefinedNameSpace(v) => write!(f,"Undifiend Name Space ({v})"),
            Self::UnknownType(v) => write!(f,"Unknown vaiable type ({v})"),
            Self::UnexpectedType(t) => write!(f,"Unexpected type ({t})"),
            Self::InvalidTypeCasting(a, b) => write!(f, "Types ({a}) and ({b}) can not be casted to eachother for this operation"),
            Self::InValidBinaryOperation(op, a, b) => write!(f,"Invalid Operation ({op}) on types ({a}) and ({b})"),
            Self::FunctionOutOfScope(s) => write!(f,"Error: Function {s} is not avaliable in this scope. Make sure you are calling the correct function"),
            Self::InvalidInlineAsm(i) => write!(f,"Invalid Identifier for Inline asm instruct ({i})"),
            Self::ImmutableVariable(v) => write!(f,"Variable ({v}) is not mutable. Did you forgot to define it with '=' insted of ':=' ?" ),
            Self::UnmatchingTypes(a, b) => write!(f, "Expected type ({a}), found type ({b})"),
            Self::NotLoopBlock => write!(f, "Can not break or continue out of non-loop blocks!"),
            Self::Err(e) => write!(f, "{e}"),
        }
    }
}

impl Error for CompilationError {}

/// Code Location
#[derive(Debug, PartialEq, Clone)]
pub struct Loc {
    /// code file path
    pub file_path: String,
    /// code line number
    pub line: usize,
    /// code col number
    pub col: usize,
}

impl Loc {
    pub fn new(file_path: String, line: usize, col: usize) -> Self {
        Self {
            file_path,
            line,
            col,
        }
    }
}

impl Display for Loc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}:{}", self.file_path, self.line, self.col)
    }
}

/// eprint error msg with location and exit the program
pub fn error(msg: impl ToString, loc: Loc) -> ! {
    eprintln!("\x1b[91m[{}]\x1b[0m {}", loc, msg.to_string());
    exit(-1);
}
