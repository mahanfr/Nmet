use std::process::exit;

use crate::{error_handeling::Loc, lexer::{Lexer, TokenType}, parser::{block::parse_stmt, expr::ExprType}};

use super::{expr::{expr, CompareOp, Expr, Op}, stmt::Stmt};

pub static PLATFORMS : &'static[&'static str] = &[
    "LINUX",
    "WINDOWS",
    "UNKNOWN"
];

pub fn parse_pre_functions(lexer: &mut Lexer, loc: Loc, master: &String) -> Vec<Stmt> {
    lexer.match_token(TokenType::Hash);
    match lexer.get_token_type() {
        TokenType::If => {
            parse_pre_condition(lexer, loc, master)
        },
        _ => {
            eprintln!("{loc}: Unknown pre-processing function!");
            exit(-1);
        }
    }
} 

fn skip_to_end(lexer: &mut Lexer) {
    loop {
        if lexer.get_token_type() == TokenType::Hash {
            lexer.match_token(TokenType::Hash);
            if lexer.get_token().literal == "end" {
                lexer.match_token(TokenType::Identifier);
                break;
            }
        }
        lexer.next_token();
    }
}

fn skip_to_next_tag(lexer: &mut Lexer) {
    loop {
        if lexer.get_token_type() == TokenType::Hash {
            lexer.match_token(TokenType::Hash);
            break;
        }
        lexer.next_token();
    }
}

fn parse_pre_condition(lexer: &mut Lexer, loc: Loc, master: &String) -> Vec<Stmt> {
    lexer.match_token(TokenType::If);
    let cond_expr = expr(lexer);
    let result = compile_pre_expr(&cond_expr);
    if result {
        let mut stmts = Vec::new();
        loop {
            if lexer.get_token_type() == TokenType::Hash {
                break;
            }
            stmts.append(&mut parse_stmt(lexer, master));
        }
        lexer.match_token(TokenType::Hash);
        if lexer.get_token().literal == "end" {
            lexer.match_token(TokenType::Identifier);
            return stmts;
        } else {
            skip_to_end(lexer);
            return stmts;
        }
    } else {
        skip_to_next_tag(lexer); 
        if lexer.get_token_type() == TokenType::Identifier && 
            lexer.get_token().literal == "end" {
            lexer.match_token(TokenType::Identifier);
            return vec![];
        }
        let mut stmts = Vec::new();
        lexer.match_token(TokenType::Else);
        if lexer.get_token_type() == TokenType::If {
            return parse_pre_condition(lexer, loc, master);
        }
        loop {
            if lexer.get_token_type() == TokenType::Hash {
                break;
            }
            stmts.append(&mut parse_stmt(lexer, master));
        }
        lexer.match_token(TokenType::Hash);
        if lexer.get_token().literal != "end" {
            eprintln!("{loc}: Syntax error: pre-processing function should end in #end");
            exit(-1);
        }
        lexer.match_token(TokenType::Identifier);
        return stmts;
    }
}

fn compile_pre_expr(expr : &Expr) -> bool {
    match &expr.etype {
        ExprType::Bool(b) => !(b == &0u8),
        ExprType::Variable(v) => {
            PLATFORMS.contains(&v.as_str())
        },
        ExprType::Unary(ub) => {
            if ub.op == Op::Not {
                return !compile_pre_expr(&ub.right);
            } else {
                eprintln!("{}: Unsupported operand for this expression", expr.loc);
                exit(-1);
            }
        },
        ExprType::Compare(c) => {
            match c.op {
                CompareOp::Eq => {
                    compile_pre_expr(&c.left) == compile_pre_expr(&c.right)
                },
                CompareOp::NotEq => {
                    compile_pre_expr(&c.left) != compile_pre_expr(&c.right)
                },
                _ => {
                    eprintln!("{}: Unsupported operand for this expression", expr.loc);
                    exit(-1);
                }
            }
        },
        _ => {
            eprintln!("{}: Unsupported expression for the pre-processing function", expr.loc);
            exit(-1);
        }
    }
}
