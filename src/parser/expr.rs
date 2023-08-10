use crate::lexer::TokenType;
use core::fmt::Display;

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Unary(UnaryExpr),
    Binary(BinaryExpr),
    Compare(CompareExpr),
    Int(i32),
    String(String),
    Variable(String),
    FunctionCall(FunctionCall),
    ArrayIndex(ArrayIndex),
}
impl Expr {
    pub fn is_binary_op(t_token: TokenType) -> bool {
        matches!(t_token, TokenType::Plus | TokenType::Minus)
    }

    pub fn is_compare_op(t_token: TokenType) -> bool {
        matches!(
            t_token,
            TokenType::DoubleEq
                | TokenType::NotEq
                | TokenType::Bigger
                | TokenType::Smaller
                | TokenType::BiggerEq
                | TokenType::SmallerEq
        )
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct UnaryExpr {
    pub op: Op,
    pub right: Box<Expr>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct BinaryExpr {
    pub left: Box<Expr>,
    pub op: Op,
    pub right: Box<Expr>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Op {
    Plus,
    Sub,
    Multi,
    Devide,
    Not,
    Mod,
}
impl Op {
    pub fn from_token_type(token_type: TokenType) -> Self {
        match token_type {
            TokenType::Plus => Self::Plus,
            TokenType::Minus => Self::Sub,
            TokenType::Multi => Self::Multi,
            TokenType::Devide => Self::Devide,
            TokenType::Not => Self::Not,
            TokenType::Mod => Self::Mod,
            _ => {
                todo!("return Error");
            }
        }
    }
}
impl Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Op::Plus => write!(f, "+"),
            Op::Sub => write!(f, "-"),
            Op::Multi => write!(f, "*"),
            Op::Devide => write!(f, "/"),
            Op::Not => write!(f, "!"),
            Op::Mod => write!(f, "%"),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionCall {
    pub ident: String,
    pub args: Vec<Expr>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ArrayIndex {
    pub ident: String,
    pub indexer: Box<Expr>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CompareExpr {
    pub left: Box<Expr>,
    pub op: CompareOp,
    pub right: Box<Expr>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum CompareOp {
    NotEq,
    Eq,
    Bigger,
    Smaller,
    BiggerEq,
    SmallerEq,
}
impl CompareOp {
    pub fn from_token_type(t_type: TokenType) -> Self {
        match t_type {
            TokenType::DoubleEq => Self::Eq,
            TokenType::NotEq => Self::NotEq,
            TokenType::Bigger => Self::Bigger,
            TokenType::Smaller => Self::Smaller,
            TokenType::BiggerEq => Self::BiggerEq,
            TokenType::SmallerEq => Self::SmallerEq,
            _ => {
                unreachable!();
            }
        }
    }
}
