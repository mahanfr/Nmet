use crate::{
    error_handeling::error,
    lexer::{Lexer, TokenType},
    parser::function::Function,
};

use super::{
    function::function_def,
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
pub enum ProgramItem {
    /// Struct Defenition
    Struct(StructDef),
    /// Function Definitions
    Func(Function),
    /// Static Variables
    StaticVar(VariableDeclare),
    /// Import Functions
    /// filePath, Import names
    Import(String, Vec<String>),
}

/// Parse Program
/// Returns Programfile wich is the ast root
pub fn program(lexer: &mut Lexer) -> ProgramFile {
    lexer.next_token();
    let mut items = Vec::<ProgramItem>::new();
    loop {
        if lexer.get_token().is_empty() {
            break;
        }
        let loc = lexer.get_token_loc();
        match lexer.get_token_type() {
            TokenType::Struct => {
                items.push(ProgramItem::Struct(struct_def(lexer)));
            }
            TokenType::Func => {
                items.push(ProgramItem::Func(function_def(lexer)));
            }
            TokenType::Var => {
                items.push(ProgramItem::StaticVar(variable_declare(lexer)));
            }
            TokenType::Import => items.push(import_file(lexer)),
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

/// import Program
/// Returns Import Program Item
pub fn import_file(lexer: &mut Lexer) -> ProgramItem {
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
        ProgramItem::Import(file_path, idents_vec)
    } else {
        ProgramItem::Import(file_path, vec![])
    }
}
