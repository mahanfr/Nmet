use std::collections::BTreeMap;

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
    structs::struct_def,
    types::StructType,
    variable_decl::{variable_declare, VariableDeclare},
};

/// Program file information
/// Ast of the code file
/// * shebang: NOT IMPLEMENTED YET
/// * filepath: path of the parsed file
/// * items: All supported top level Items
#[derive(Debug, Clone)]
pub struct ProgramFile {
    // pub attrs: Vec<Attr>
    pub items: Vec<ProgramItem>,
}

/// Top level program items
/// e.g: functions, static variables and imports
#[derive(Debug, Clone)]
#[allow(clippy::upper_case_acronyms)]
pub enum ProgramItem {
    /// Struct Defenition
    Struct(StructType),
    /// Function Definitions
    Func(FunctionDef),
    /// Static Variables
    StaticVar(VariableDeclare),
    /// Foregin Function interface
    FFI(String, FunctionDecl),
}

impl ProgramItem {
    pub fn get_key(&self) -> String {
        match self {
            Self::Struct(st) => st.ident.clone(),
            Self::FFI(_, fun) => fun.ident.clone(),
            Self::Func(func) => func.decl.ident.clone(),
            Self::StaticVar(st) => st.ident.clone(),
        }
    }
}

/// Parse Program
/// Returns Programfile wich is the ast root
pub fn generate_ast(lexer: &mut Lexer) -> ProgramFile {
    lexer.next_token();
    let mut items = BTreeMap::<String, ProgramItem>::new();
    loop {
        if lexer.get_token().is_empty() {
            break;
        }
        let loc = lexer.get_token_loc();
        match lexer.get_token_type() {
            TokenType::Struct => {
                let struct_def = struct_def(lexer);
                let ident = struct_def.ident.clone();
                let prv_value = items.insert(ident.clone(), ProgramItem::Struct(struct_def));
                if prv_value.is_some() {
                    error(
                        format!("Struct with the name {} already exists", ident.clone()),
                        loc,
                    );
                }
            }
            TokenType::Ffi => {
                let ffi_func = parse_ffi_function_mapping(lexer);
                let ident = ffi_func.1.ident.clone();
                let prv_value =
                    items.insert(ident.clone(), ProgramItem::FFI(ffi_func.0, ffi_func.1));
                if prv_value.is_some() {
                    error(
                        format!("Function with the name {} already exists", ident),
                        loc,
                    );
                }
            }
            TokenType::Func | TokenType::Extern => {
                let function_def = parse_function_definition(lexer);
                let ident = function_def.decl.ident.clone();
                let prv_value = items.insert(ident.clone(), ProgramItem::Func(function_def));
                if prv_value.is_some() {
                    error(
                        format!("Function with the name {} already exists", ident),
                        loc,
                    );
                }
            }
            TokenType::Static => {
                lexer.match_token(TokenType::Static);
                let var_decl = variable_declare(lexer);
                lexer.match_token(TokenType::SemiColon);
                let ident = var_decl.ident.clone();
                let prv_value = items.insert(ident.clone(), ProgramItem::StaticVar(var_decl));
                if prv_value.is_some() {
                    error(
                        format!("Variable with the name {} already exists", ident),
                        loc,
                    );
                }
            }
            TokenType::Import => {
                let import = parse_mod_import(lexer);
                let mut new_path = import.0;
                new_path.push_str(".nmt");
                let new_file = parse_source_file(new_path);
                for item_name in import.1.iter() {
                    if items.contains_key(item_name) {
                        error(
                            format!("Import failed beacuse namespace with the name ({}) already exists in this program",
                            item_name), loc);
                    }
                }
                for item in new_file.items {
                    if import.1.is_empty() {
                        items.insert(item.get_key(), item);
                    } else {
                        let key = item.get_key();
                        if import.1.contains(&key) {
                            items.insert(item.get_key(), item);
                        }
                    }
                }
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
        items: items.values().cloned().collect::<Vec<ProgramItem>>(),
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
