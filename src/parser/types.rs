use std::process::exit;

use crate::lexer::{Lexer, TokenType};

#[derive(Debug, Clone, PartialEq)]
pub enum VariableType {
    Custom(String),
    Array(Box<VariableType>, usize),
    String,
    Int,
    Pointer,
    UInt,
    Bool,
    Char,
}
impl VariableType {
    pub fn from_string(literal: String) -> Self {
        match literal.as_str() {
            "int" | "i32" => Self::Int,
            "uint" | "u32" => Self::UInt,
            "char" | "u8" => Self::Char,
            "bool" => Self::Bool,
            "str" => Self::String,
            "ptr" => Self::Pointer,
            _ => Self::Custom(literal),
        }
    }
}

pub fn type_def(lexer: &mut Lexer) -> VariableType {
    lexer.match_token(TokenType::ATSign);
    match lexer.get_token_type() {
        TokenType::Ptr => {
            lexer.match_token(TokenType::Ptr);
            VariableType::Pointer
        }
        TokenType::Identifier => {
            let ident = lexer.get_token().literal;
            lexer.match_token(TokenType::Identifier);
            VariableType::from_string(ident)
        }
        TokenType::OBracket => {
            let var_type: VariableType;
            let size: usize;
            lexer.match_token(TokenType::OBracket);
            let token = lexer.get_token();
            if token.is_empty() {
                eprintln!(
                    "Error: Expected an Identifier found EOF at {}:{}",
                    lexer.file_path,
                    lexer.get_token_loc()
                );
                exit(1);
            }
            if token.t_type == TokenType::Identifier {
                var_type = VariableType::from_string(lexer.get_token().literal);
                lexer.match_token(TokenType::Identifier);
            } else if token.t_type == TokenType::ATSign {
                var_type = self::type_def(lexer);
            } else {
                eprintln!(
                    "Error: Expected Identifier found {:?}, at {}:{}",
                    lexer.get_token_type(),
                    lexer.file_path,
                    lexer.get_token_loc()
                );
                exit(1);
            }
            lexer.match_token(TokenType::Comma);
            let token = lexer.get_token();
            if token.is_empty() {
                eprintln!(
                    "Error: Expected a Number found EOF at {}:{}",
                    lexer.file_path,
                    lexer.get_token_loc()
                );
                exit(1);
            }
            match token.t_type {
                TokenType::Int(s) => {
                    size = s as usize;
                    lexer.match_token(TokenType::Int(s));
                }
                TokenType::QMark => {
                    lexer.match_token(TokenType::QMark);
                    return VariableType::Pointer;
                }
                _ => {
                    eprintln!(
                        "Error: Expected Integer Number found {:?}, at {}:{}",
                        lexer.get_token_type(),
                        lexer.file_path,
                        lexer.get_token_loc()
                    );
                    exit(1);
                }
            }
            lexer.match_token(TokenType::CBracket);
            VariableType::Array(Box::new(var_type), size)
        }
        _ => {
            eprintln!("Syntax Error: Unknown Token at {}:{}", lexer.file_path, lexer.get_token_loc());
            exit(1);
        }
    }
}
