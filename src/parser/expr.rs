use crate::lexer::{Lexer, TokenType};
use core::fmt::Display;
use std::process::exit;

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Unary(UnaryExpr),
    Binary(BinaryExpr),
    Compare(CompareExpr),
    Int(i32),
    Char(u8),
    Ptr(Box<Expr>),
    String(String),
    Variable(String),
    FunctionCall(FunctionCall),
    ArrayIndex(ArrayIndex),
}
impl Expr {
    pub fn is_binary_op(t_token: TokenType) -> bool {
        matches!(
            t_token,
            TokenType::Plus | TokenType::Minus | TokenType::And | TokenType::Or
        )
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
    And,
    Or,
    Lsh,
    Rsh,
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
            TokenType::And => Self::And,
            TokenType::Or => Self::Or,
            TokenType::Lsh => Self::Lsh,
            TokenType::Rsh => Self::Rsh,
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
            Op::And => write!(f, "&"),
            Op::Or => write!(f, "|"),
            Op::Lsh => write!(f, "<<"),
            Op::Rsh => write!(f, ">>"),
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

pub fn expr(lexer: &mut Lexer) -> Expr {
    let mut term_expr = term(lexer);
    loop {
        let t_type = lexer.get_token_type();
        if Expr::is_binary_op(t_type) {
            let op = Op::from_token_type(t_type);
            lexer.next_token();
            let right = term(lexer);
            term_expr = Expr::Binary(BinaryExpr {
                left: Box::new(term_expr),
                op,
                right: Box::new(right),
            });
        } else if Expr::is_compare_op(t_type) {
            let op = CompareOp::from_token_type(lexer.get_token_type());
            lexer.next_token();
            let right = term(lexer);
            term_expr = Expr::Compare(CompareExpr {
                left: Box::new(term_expr),
                op,
                right: Box::new(right),
            });
        } else {
            break;
        }
    }
    term_expr
}

pub fn term(lexer: &mut Lexer) -> Expr {
    let mut left = factor(lexer);
    while lexer.get_token_type() == TokenType::Multi
        || lexer.get_token_type() == TokenType::Devide
        || lexer.get_token_type() == TokenType::Mod
        || lexer.get_token_type() == TokenType::Lsh
        || lexer.get_token_type() == TokenType::Rsh
    {
        let op = Op::from_token_type(lexer.get_token_type());
        lexer.next_token();
        let right = factor(lexer);
        left = Expr::Binary(BinaryExpr {
            left: Box::new(left),
            op,
            right: Box::new(right),
        });
    }
    left
}

pub fn factor(lexer: &mut Lexer) -> Expr {
    match lexer.get_token_type() {
        TokenType::OParen => {
            lexer.match_token(TokenType::OParen);
            let value = expr(lexer);
            lexer.match_token(TokenType::CParen);
            value
        }
        TokenType::Plus | TokenType::Minus | TokenType::Not => {
            let op = Op::from_token_type(lexer.get_token_type());
            lexer.next_token();
            let value = factor(lexer);
            Expr::Unary(UnaryExpr {
                op,
                right: Box::new(value),
            })
        }
        TokenType::String => {
            let str_token = lexer.get_token();
            lexer.next_token();
            Expr::String(str_token.literal)
        }
        TokenType::Ptr => {
            lexer.match_token(TokenType::Ptr);
            let value = expr(lexer);
            Expr::Ptr(Box::new(value))
        }
        TokenType::True => {
            lexer.match_token(TokenType::True);
            Expr::Int(1)
        }
        TokenType::False => {
            lexer.match_token(TokenType::False);
            Expr::Int(0)
        }
        TokenType::Char(c) => {
            lexer.next_token();
            Expr::Char(c as u8)
        }
        TokenType::Int(val) => {
            lexer.next_token();
            Expr::Int(val)
        }
        TokenType::Identifier => {
            let ident_name = lexer.get_token().literal;
            if lexer.next_token().is_empty() {
                return Expr::Variable(ident_name);
            }
            match lexer.get_token_type() {
                TokenType::OParen => {
                    let args = function_call_args(lexer);
                    Expr::FunctionCall(FunctionCall {
                        ident: ident_name,
                        args,
                    })
                }
                TokenType::OBracket => {
                    let indexer = array_indexer(lexer);
                    Expr::ArrayIndex(ArrayIndex {
                        ident: ident_name,
                        indexer: Box::new(indexer),
                    })
                }
                _ => Expr::Variable(ident_name),
            }
        }
        _ => {
            eprintln!(
                "Unexpected Token ({:?}) while parsing expr at {}:{}",
                lexer.get_token_type(),
                lexer.file_path,
                lexer.get_token_loc()
            );
            exit(-1);
        }
    }
}

pub fn array_indexer(lexer: &mut Lexer) -> Expr {
    lexer.match_token(TokenType::OBracket);
    let index = expr(lexer);
    lexer.match_token(TokenType::CBracket);
    index
}

pub fn function_call_args(lexer: &mut Lexer) -> Vec<Expr> {
    let mut args = Vec::<Expr>::new();
    lexer.match_token(TokenType::OParen);
    loop {
        //|| | expr | expr , expr
        match lexer.get_token_type() {
            TokenType::CParen => {
                lexer.match_token(TokenType::CParen);
                break;
            }
            _ => {
                args.push(expr(lexer));
                if lexer.get_token_type() == TokenType::Comma {
                    lexer.match_token(TokenType::Comma);
                }
            }
        }
    }
    args
}
