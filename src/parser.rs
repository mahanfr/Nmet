
use std::fmt::Display;

use crate::lexer::TokenType;
// -4 -> 4 neg
// 4 + 2 -> 4 2 +
// 4 * 3 + 6 -> 4 3 * 6 +
// 4 + (3 + 6) -> 3 6 + 4 +
// -(4 * cos(0) + 2 - 6) -> 4 cos(0) * 2 + 6 - neg

#[derive(Debug,PartialEq,Clone)]
pub enum Op {
    Plus,
    Sub,
    Multi,
    Devide,
    Oparen,
    Neg,
    Pos
}
impl Op {
    pub fn from_token_type(token_type: TokenType) -> Self {
        match token_type {
            TokenType::Plus => return Self::Plus,
            TokenType::Minus => return Self::Sub,
            TokenType::Multi => return Self::Multi,
            TokenType::Devide => return Self::Devide,
            _ => {
                todo!("return Error");
            }
        }
    }

    pub fn get_op_precedence(op: &Op) -> u8 {
        match op {
            Op::Plus | Op::Sub => 1,
            Op::Multi | Op::Devide => 2,
            Op::Oparen => 0,
            Op::Neg => u8::MAX,
            Op::Pos => u8::MAX,
        }
    }
}
impl Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       match self {
           Op::Plus => write!(f,"+"),
           Op::Sub => write!(f,"-"),
           Op::Multi => write!(f,"*"),
           Op::Devide => write!(f,"/"),
           Op::Neg => write!(f,"-"),
           Op::Pos => write!(f,"+"),
           _ => write!(f,""),

       } 
    }
}


#[derive(Debug,PartialEq,Clone)]
pub struct UnaryExpr {
    pub op: Op,
    pub right: Box<Expr>
}

#[derive(Debug,PartialEq,Clone)]
pub struct BinaryExpr {
    pub left: Box<Expr>,
    pub op: Op,
    pub right: Box<Expr>
}

#[derive(Debug,PartialEq,Clone)]
pub struct FunctionCall {
    pub identifier: String,
    pub args: Vec<Expr>,
}

#[derive(Debug,PartialEq,Clone)]
pub struct ArrayIndex {
    pub identifier: String,
    pub indexer: Box<Expr>,
}

#[derive(Debug,PartialEq,Clone)]
pub enum Expr {
    Unary(UnaryExpr),
    Binary(BinaryExpr),
    Int(i32),
    Variable(String),
    FunctionCall(FunctionCall),
    ArrayIndex(ArrayIndex),
}
