use std::fmt::Display;

use crate::{
    error_handeling::error,
    lexer::{Lexer, TokenType},
};

/// Type of any variable or function
/// Expandable
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum VariableType {
    /// Default type for functions
    Void,
    /// Type of a Variable Before type refering
    /// Will cause unreachable code if used
    Any,
    /// 4 byte integer
    Int,
    /// 64bit float
    Float,
    /// 4 byte unsigned integer
    UInt,
    /// 8 byte long
    Long,
    /// 8 byte unsigned long
    ULong,
    /// 1 byte Boolean
    Bool,
    /// 1 byte character or integer
    Char,
    /// 16 byte String pointer and usize
    String,
    /// 8 byte Adress to a memory
    Pointer,
    /// const sized array
    Array(Box<VariableType>, usize),
    /// user defined types
    Custom(String),
}
impl VariableType {
    /// Convert String literal to Variable Type
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
            "float" | "f" => Self::Float,
            _ => Self::Custom(literal),
        }
    }

    /// Returns size of a single item in the type
    pub fn item_size(&self) -> u8 {
        match self {
            Self::Array(a, _) => a.size() as u8,
            Self::String => 8,
            _ => self.size() as u8,
        }
    }

    /// returns size of the type
    pub fn size(&self) -> usize {
        match self {
            Self::Int | Self::UInt => 4,
            Self::Long | Self::ULong | Self::Pointer => 8,
            Self::Bool => 1,
            Self::Char => 1,
            Self::String => 16,
            Self::Void => 0,
            Self::Array(t, s) => t.item_size() as usize * s,
            Self::Float => 8,
            Self::Custom(_) => 8,
            Self::Any => todo!(),
        }
    }

    /// checks if type is any
    pub fn is_any(&self) -> bool {
        matches!(self, Self::Any)
    }

    /// Cast two types into a single type
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
            if cmp.0 == &Self::Float || cmp.1 == &Self::Float {
                return Ok(Self::Float);
            }
            if cmp.0.size() < cmp.1.size() {
                Ok(cmp.1.clone())
            } else {
                Ok(cmp.0.clone())
            }
        } else {
            Err(format!(
                "Types ({}) and ({}) can not be casted to eachother for this operation",
                cmp.0, cmp.1
            ))
        }
    }

    /// returns true if types can be used mathmaticaly
    pub fn is_numeric(&self) -> bool {
        matches!(
            self,
            Self::Int
                | Self::Char
                | Self::UInt
                | Self::Pointer
                | Self::Long
                | Self::ULong
                | Self::Float
        )
    }

    // pub fn to_llvm_type(&self) -> String {
    //     match self {
    //         VariableType::Any => unreachable!(),
    //         VariableType::Custom(s) => format!("{}", s),
    //         VariableType::Array(t, s) => format!("[{} x {}]", s, t),
    //         VariableType::String => todo!(),
    //         VariableType::Long => "i64".to_string(),
    //         VariableType::ULong => "u64".to_string(),
    //         VariableType::Int => "i32".to_string(),
    //         VariableType::Pointer => "ptr".to_string(),
    //         VariableType::UInt => "u32".to_string(),
    //         VariableType::Bool => todo!(),
    //         VariableType::Char => todo!(),
    //         VariableType::Void => todo!(),
    //     }
    // }
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
            VariableType::Float => write!(f, "@float"),
        }
    }
}

/// Parse type definition
pub fn type_def(lexer: &mut Lexer) -> VariableType {
    let loc = lexer.get_current_loc();
    lexer.match_token(TokenType::ATSign);
    match lexer.get_token_type() {
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
