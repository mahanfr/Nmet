pub mod assign;
pub mod block;
pub mod expr;
pub mod function;
pub mod program;
pub mod stmt;
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
pub fn parse_file(path: String) -> ProgramFile {
    let source = fs::read_to_string(path.clone()).expect("Can not Read the file");
    let mut lexer = Lexer::new(path, source);
    program(&mut lexer)
}
