use crate::lexer::{Lexer, expect_non_empty_token, TToken};

pub fn get_expr(lexer : &mut Lexer) -> Expr {
    let token = lexer.next_token();
    expect_non_empty_token(&token);
    match token.ttype {
       TToken::OPAREN => {
       },
       TToken::Identifier => {
       },
       TToken::SUB => {
       },
       TToken::PLUS => {
       },
       TToken::Number => {
           let op_token = lexer.next_token();
           if op_token.ttype == TToken::PLUS || op_token.ttype == TToken::SUB {
               let left_val = String::from_utf8(token.literal).unwrap();
               return Expr::Binary {
                   left: Box::new(Expr::Literal { 
                       value: left_val.parse::<u32>().unwrap() 
                   }),
                   right: Box::new(get_expr(lexer)),
                   op: op_token.ttype,
               }
           }else if op_token.ttype == TToken::MULTY || op_token.ttype == TToken::DEVIDE ||
            op_token.ttype == TToken::MOD {
                unimplemented!()
            }
       }
        _ => ()
    }
    todo!()
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    Literal{
        value: u32,
    },
    Term{
        left : Box<Expr>,
        right: Box<Expr>,
        op   : TToken,
    },
    Binary{
        left : Box<Expr>,
        right: Box<Expr>,
        op   : TToken,
    },
    Unary,
    Path {
        ident: String,
    }
}

#[derive(Debug, PartialEq)]
pub struct ExprPath {}

#[derive(Debug, PartialEq)]
pub struct ExprAssign {
    pub left: Expr,
    pub right: Expr,
}

