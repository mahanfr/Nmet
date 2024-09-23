/**********************************************************************************************
*
*   parser/mod: parse a single file
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
pub mod assign;
pub mod block;
pub mod expr;
pub mod function;
pub mod preprocessing;
pub mod program;
pub mod stmt;
pub mod structs;
pub mod types;
pub mod variable_decl;
use std::fs;

use crate::lexer::Lexer;
use crate::parser::program::*;

/// Parsing a Single File
///
/// # Arguments
/// * path - path to the code file
///
/// # Returns
/// Programfile containing the ast of the parsed file
///
/// Can panic if file dose not exists
pub fn parse_source_file(path: String) -> ProgramFile {
    let source = fs::read_to_string(path.clone()).unwrap_or_else(|_| {
        eprintln!("Error reading file \"{}\"", path.clone());
        panic!("Can not open file!");
    });
    let mut lexer = Lexer::new(path, source);
    generate_ast(&mut lexer)
}
