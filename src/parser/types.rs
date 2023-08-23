use std::fmt::Display;

use crate::{
    error_handeling::error,
    lexer::{Lexer, TokenType},
};

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum VariableType {
    Void,
    Any,
    Int,
    UInt,
    Long,
    ULong,
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
            "ulong" | "u64" => Self::ULong,
            "long" | "i64" => Self::Long,
            "char" | "u8" => Self::Char,
            "void" => Self::Void,
            "bool" => Self::Bool,
            "str" => Self::String,
            "ptr" => Self::Pointer,
            _ => Self::Custom(literal),
        }
    }

    pub fn item_size(&self) -> usize {
        match self {
            Self::Array(a, _) => a.size(),
            Self::String => 8,
            _ => self.size(),
        }
    }

    pub fn size(&self) -> usize {
        match self {
            Self::Int | Self::UInt => 4,
            Self::Long | Self::ULong | Self::Pointer => 8,
            Self::Bool => 1,
            Self::Char => 1,
            Self::String => 16,
            Self::Void => 0,
            Self::Array(v, s) => v.size() * s,
            _ => unreachable!(),
        }
    }

    pub fn is_any(&self) -> bool {
        matches!(self, Self::Any)
    }

    pub fn cast(&self, other: &Self) -> Result<Self, String> {
        let cmp = (self, other);
        if cmp.0 == cmp.1 {
            return Ok(self.clone());
        }
        if cmp.0.is_any() {
            return Ok(other.clone());
        }
        if cmp.1.is_any() {
            return Ok(self.clone());
        }
        if cmp.0 == &Self::Pointer || cmp.1 == &Self::Pointer {
            return Ok(Self::Pointer);
        }
        if cmp.0.is_numeric() && cmp.1.is_numeric() {
            if cmp.0.size() < cmp.1.size() {
                Ok(cmp.1.clone())
            } else {
                Ok(cmp.0.clone())
            }
        } else {
            return Err(format!(
                "Types ({}) and ({}) can not be casted to eachother for this operation",
                cmp.0, cmp.1
            ));
        }
    }

    pub fn is_numeric(&self) -> bool {
        matches!(
            self,
            Self::Int | Self::Char | Self::UInt | Self::Pointer | Self::Long | Self::ULong
        )
    }
}
impl Display for VariableType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VariableType::Any => write!(f, "@?"),
            VariableType::Custom(s) => write!(f, "@{}", s),
            VariableType::Array(t, s) => write!(f, "@[{},{}]", t, s),
            VariableType::String => write!(f, "@str"),
            VariableType::Long => write!(f, "@long"),
            VariableType::ULong => write!(f, "@ulong"),
            VariableType::Int => write!(f, "@int"),
            VariableType::Pointer => write!(f, "@ptr"),
            VariableType::UInt => write!(f, "@uint"),
            VariableType::Bool => write!(f, "@bool"),
            VariableType::Char => write!(f, "@char"),
            VariableType::Void => write!(f, "@void"),
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
