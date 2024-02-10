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
use std::{fmt::Display, process::exit};

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
    eprintln!("ERROR: {} {loc}", msg.to_string());
    exit(-1);
}
