use std::fmt::Display;

use crate::{
    error_handeling::error,
    lexer::{Lexer, TokenType},
};

#[derive(Debug, Clone, PartialEq)]
pub enum VariableType {
    Any,
    Int,
    UInt,
    Bool,
    Char,
    String,
    Pointer,
    Array(Box<VariableType>, usize),
    Custom(String),
}
impl VariableType {
    pub fn from_string(literal: String) -> Self {
        match literal.as_str() {
            "?" => Self::Any,
            "int" | "i32" => Self::Int,
            "uint" | "u32" => Self::UInt,
            "char" | "u8" => Self::Char,
            "bool" => Self::Bool,
            "str" => Self::String,
            "ptr" => Self::Pointer,
            _ => Self::Custom(literal),
        }
    }
    
    pub fn as_size(&self) -> usize {
        match self {
            Self::Int => 4,
            Self::UInt => 4,
            Self::Bool => 1,
            Self::Char => 1,
            Self::Pointer => 8,
            Self::Array(v, s) => {
                v.as_size() * s
            },
            _ => unreachable!()
        }
    }

    pub fn binary_cast(&self, other: &Self) -> Result<Self,String> {
        let cmp = (self,other);
        match cmp {
            (Self::Int, Self::UInt) | (Self::UInt, Self::Int) => Ok(Self::Int),
            (Self::Pointer, Self::Int) | (Self::Int, Self::Pointer) => Ok(Self::Pointer),
            (Self::Int | Self::UInt, Self::Char) | 
                (Self::Char, Self::UInt| Self::Int) => Ok(Self::Pointer),
            _ => Err(
                format!("Types ({}) and ({}) can not be casted to eachother for this operation", cmp.0 ,cmp.1))
        }
    }

    pub fn is_numeric(&self) -> bool {
        matches!(self,VariableType::Int | VariableType::Char | VariableType::UInt| VariableType::Pointer)
    }
}
impl Display for VariableType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VariableType::Any => write!(f,"@?"),
            VariableType::Custom(s) => write!(f,"@{}",s),
            VariableType::Array(t, s) => write!(f,"@[{},{}]",t,s),
            VariableType::String => write!(f,"@str"),
            VariableType::Int => write!(f,"@int"),
            VariableType::Pointer => write!(f,"@ptr"),
            VariableType::UInt => write!(f,"@uint"),
            VariableType::Bool => write!(f,"@bool"),
            VariableType::Char => write!(f,"@char"),
        }
    }
}


pub fn type_def(lexer: &mut Lexer) -> VariableType {
    let loc = lexer.get_current_loc();
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
                error("Expected an Identifier found EOF", loc);
            }
            if token.t_type == TokenType::Identifier {
                var_type = VariableType::from_string(lexer.get_token().literal);
                lexer.match_token(TokenType::Identifier);
            } else if token.t_type == TokenType::ATSign {
                var_type = self::type_def(lexer);
            } else {
                error(
                    format!(
                        "Error: Expected Identifier found ({})",
                        lexer.get_token_type()
                    ),
                    loc,
                );
            }
            lexer.match_token(TokenType::Comma);
            let token = lexer.get_token();
            if token.is_empty() {
                error("Error: Expected a Number found EOF", loc);
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
                    error(
                        format!(
                            "Error: Expected Integer Number found ({})",
                            lexer.get_token_type()
                        ),
                        loc,
                    );
                }
            }
            lexer.match_token(TokenType::CBracket);
            VariableType::Array(Box::new(var_type), size)
        }
        _ => {
            error(
                format!("Syntax Error: Unknown Token ({})", lexer.get_token_type()),
                loc,
            );
        }
    }
}
