use crate::{
    error_handeling::{error, Loc},
    lexer::{Lexer, TokenType},
};
use core::fmt::Display;

#[derive(Debug, PartialEq, Clone)]
pub struct Expr {
    pub loc: Loc,
    pub etype: ExprType,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ExprType {
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
impl ExprType {
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
        if ExprType::is_binary_op(t_type) {
            let op = Op::from_token_type(t_type);
            lexer.next_token();
            let right = term(lexer);
            term_expr = Expr {
                etype: ExprType::Binary(BinaryExpr {
                    left: Box::new(term_expr),
                    op,
                    right: Box::new(right),
                }),
                loc: lexer.get_token_loc(),
            };
        } else if ExprType::is_compare_op(t_type) {
            let op = CompareOp::from_token_type(lexer.get_token_type());
            lexer.next_token();
            let right = term(lexer);
            term_expr = Expr {
                etype: ExprType::Compare(CompareExpr {
                    left: Box::new(term_expr),
                    op,
                    right: Box::new(right),
                }),
                loc: lexer.get_token_loc(),
            };
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
        left = Expr {
            etype: ExprType::Binary(BinaryExpr {
                left: Box::new(left),
                op,
                right: Box::new(right),
            }),
            loc: lexer.get_token_loc(),
        };
    }
    left
}

pub fn factor(lexer: &mut Lexer) -> Expr {
    let loc = lexer.get_current_loc();
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
            Expr {
                etype: ExprType::Unary(UnaryExpr {
                    op,
                    right: Box::new(value),
                }),
                loc,
            }
        }
        TokenType::String => {
            let str_token = lexer.get_token();
            lexer.next_token();
            Expr {
                etype: ExprType::String(str_token.literal),
                loc,
            }
        }
        TokenType::Ptr => {
            lexer.match_token(TokenType::Ptr);
            let value = expr(lexer);
            Expr {
                etype: ExprType::Ptr(Box::new(value)),
                loc,
            }
        }
        TokenType::True => {
            lexer.match_token(TokenType::True);
            Expr {
                etype: ExprType::Int(1),
                loc,
            }
        }
        TokenType::False => {
            lexer.match_token(TokenType::False);
            Expr {
                etype: ExprType::Int(0),
                loc,
            }
        }
        TokenType::Char(c) => {
            lexer.next_token();
            Expr {
                etype: ExprType::Char(c as u8),
                loc,
            }
        }
        TokenType::Int(val) => {
            lexer.next_token();
            Expr {
                etype: ExprType::Int(val),
                loc,
            }
        }
        TokenType::Identifier => {
            let ident_name = lexer.get_token().literal;
            if lexer.next_token().is_empty() {
                return Expr {
                    etype: ExprType::Variable(ident_name),
                    loc,
                };
            }
            match lexer.get_token_type() {
                TokenType::OParen => {
                    let args = function_call_args(lexer);
                    Expr {
                        etype: ExprType::FunctionCall(FunctionCall {
                            ident: ident_name,
                            args,
                        }),
                        loc,
                    }
                }
                TokenType::OBracket => {
                    let indexer = array_indexer(lexer);
                    Expr {
                        etype: ExprType::ArrayIndex(ArrayIndex {
                            ident: ident_name,
                            indexer: Box::new(indexer),
                        }),
                        loc,
                    }
                }
                _ => Expr {
                    etype: ExprType::Variable(ident_name),
                    loc,
                },
            }
        }
        _ => {
            error(
                format!(
                    "Unexpected Token ({}) while parsing expr",
                    lexer.get_token_type(),
                ),
                loc,
            );
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
