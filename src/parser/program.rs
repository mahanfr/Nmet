/**********************************************************************************************
*
*   parser/program: program level parsing
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
use crate::{
    error_handeling::error,
    lexer::{Lexer, TokenType},
};

use super::{
    function::{parse_function_declaration, parse_function_definition, FunctionDecl, FunctionDef},
    parse_source_file,
    structs::{struct_def, StructDef},
    variable_decl::{variable_declare, VariableDeclare},
};

/// Program file information
/// Ast of the code file
/// * shebang: NOT IMPLEMENTED YET
/// * filepath: path of the parsed file
/// * items: All supported top level Items
#[derive(Debug, Clone)]
pub struct ProgramFile {
    pub shebang: String,
    pub file_path: String,
    // pub attrs: Vec<Attr>
    pub items: Vec<ProgramItem>,
}

/// Top level program items
/// e.g: functions, static variables and imports
#[derive(Debug, Clone)]
#[allow(clippy::upper_case_acronyms)]
pub enum ProgramItem {
    /// Struct Defenition
    Struct(StructDef),
    /// Function Definitions
    Func(FunctionDef),
    /// Static Variables
    StaticVar(VariableDeclare),
    /// Foregin Function interface
    FFI(String, FunctionDecl),
}

/// Parse Program
/// Returns Programfile wich is the ast root
pub fn generate_ast(lexer: &mut Lexer) -> ProgramFile {
    lexer.next_token();
    let mut items = Vec::<ProgramItem>::new();
    loop {
        if lexer.get_token().is_empty() {
            break;
        }
        let loc = lexer.get_token_loc();
        match lexer.get_token_type() {
            TokenType::Struct => {
                let struct_def = struct_def(lexer);
                items.push(ProgramItem::Struct(struct_def));
            }
            TokenType::Ffi => {
                let ffi_function = parse_ffi_function_mapping(lexer);
                items.push(ProgramItem::FFI(ffi_function.0, ffi_function.1));
            }
            TokenType::Func => {
                let function_def = parse_function_definition(lexer);
                items.push(ProgramItem::Func(function_def));
            }
            TokenType::Var => {
                items.push(ProgramItem::StaticVar(variable_declare(lexer)));
            }
            TokenType::Import => {
                let import = parse_mod_import(lexer);
                let mut new_path = import.0;
                new_path.push_str(".nmt");
                items.extend(parse_source_file(new_path).items);
            }
            _ => error(
                format!(
                    "Unexpected Token ({}) for the top level program",
                    lexer.get_token_type()
                ),
                loc,
            ),
        }
    }
    ProgramFile {
        shebang: String::new(),
        file_path: lexer.file_path.clone(),
        items,
    }
}
/// Include FFI
/// Returns FFI Program Item
///
/// Syntax:
/// ffi "fopen" func nmt_fopen(pathname @str, mode @str) @FILE
pub fn parse_ffi_function_mapping(lexer: &mut Lexer) -> (String, FunctionDecl) {
    lexer.match_token(TokenType::Ffi);
    let module_name = lexer.get_token().literal;
    lexer.match_token(TokenType::String);
    let function = parse_function_declaration(lexer);
    (module_name, function)
}

/// import Program
/// Returns Import Program Item
pub fn parse_mod_import(lexer: &mut Lexer) -> (String, Vec<String>) {
    lexer.match_token(TokenType::Import);
    let file_path = lexer.get_token().literal;
    lexer.match_token(TokenType::String);
    if lexer.get_token_type() == TokenType::DoubleColon {
        lexer.match_token(TokenType::DoubleColon);
        let mut idents_vec = Vec::<String>::new();
        loop {
            let ident = lexer.get_token().literal;
            lexer.match_token(TokenType::Identifier);
            idents_vec.push(ident);
            if lexer.get_token_type() == TokenType::Comma {
                lexer.match_token(TokenType::Comma);
            } else {
                break;
            }
        }
        (file_path, idents_vec)
    } else {
        (file_path, vec![])
    }
}
