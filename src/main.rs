use std::error::Error;
use std::fs;
use std::env::args;

mod lexer;
mod ast;
mod parser;
use lexer::{Lexer, TokenType};
use parser::{Expr, Op, UnaryExpr, BinaryExpr};

// --- Static Compiler Defenition
static VERSION : &'static str = "v0.0.1-Beta";
static COPYRIGHT : &'static str = "Mahan Farzaneh 2023-2024";
static DEBUG : bool = true;

fn padding_right(str : &str) -> String {
    let mut text = String::with_capacity(20);
    text.push_str(str);
    for _ in 0..(20-str.len()) {
       text.push(' '); 
    }
    text
}

fn help_command() -> Result<(),Box<dyn Error>> {
    println!("Nemet {VERSION}");
    println!("Nemet Programmin Language Copyright: {COPYRIGHT}");
    println!("Project distributed Under MIT License");
    if DEBUG {
        println!("--- DEBUG MODE ---");
    }
    println!("\nnemet [Command] <path> (Options)");
    println!("Commands:");
    println!("\t{} Show help",padding_right("help"));
    println!("Options:");
    println!("\t{} Show help",padding_right("--help"));
    println!("\t{} Show Version",padding_right("--version"));
    Ok(())
}

fn compile_command(path: String) -> Result<(),Box<dyn Error>> {
    let source = fs::read_to_string(path.clone())
        .expect("Can not Read the file");
    let mut lexer = Lexer::new(path.clone(),source);
    let mut token = lexer.next_token();
    while !token.is_none() {
        println!("{:?}",token.unwrap());
        token = lexer.next_token();
    }
    Ok(())
}

pub fn expr(lexer: &mut Lexer) -> Expr {
    let mut term_expr = term(lexer);
    while lexer.get_token_type() == TokenType::Plus || 
        lexer.get_token_type() == TokenType::Devide {
        let op = Op::from_token_type(lexer.get_token_type());
        lexer.next_token();
        let right = factor(lexer);
        term_expr = Expr::Binary(BinaryExpr {left: Box::new(term_expr), op, right: Box::new(right)});
    }
    return term_expr;
} 

pub fn term(lexer: &mut Lexer) -> Expr {
    let mut left = factor(lexer);
    while lexer.get_token_type() == TokenType::Multi || 
        lexer.get_token_type() == TokenType::Devide {
        let op = Op::from_token_type(lexer.get_token_type());
        lexer.next_token();
        let right = factor(lexer);
        left = Expr::Binary(BinaryExpr {left: Box::new(left), op, right: Box::new(right)});
    }
    return left;
}

pub fn factor(lexer: &mut Lexer) -> Expr {
    match lexer.get_token_type() {
        TokenType::OParen => {
            lexer.match_token(TokenType::OParen);
            let value = expr(lexer);
            lexer.match_token(TokenType::CParen);
            return value;
        },
        TokenType::Plus | TokenType::Minus => {
            let op = Op::from_token_type(lexer.get_token_type()); 
            lexer.next_token();
            let value = factor(lexer);
            return Expr::Unary(UnaryExpr{op, right: Box::new(value)});
        },
        TokenType::Int(val) => {
            lexer.next_token();
            return Expr::Int(val);
        },
        TokenType::Identifier => {
            let ident_name = lexer.get_token().unwrap().literal;
            if lexer.next_token().is_none() {
                return Expr::Variable(ident_name);
            }
            match lexer.get_token_type() {
                TokenType::OParen => {
                    let args = function_call_args(lexer);
                    return Expr::FunctionCall(parser::FunctionCall{identifier: ident_name, args});
                },
                TokenType::OBracket => {
                    //let indexer = indexer(lexer);
                    todo!();
                },
                _ => {
                    return Expr::Variable(ident_name);
                }
            }
        }
        _ => {
            todo!("ident, funcall and indexer");
        }
    }
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
            },
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


fn main() -> Result<(),Box<dyn Error>> {
    let mut lexer = Lexer::new(String::new(),"-(a(0,0)*2);".to_string());
    // TODO: Move to top root of parser
    lexer.next_token();
    println!("{:?}",expr(&mut lexer));
    return Ok(());
    let mut arg = args().into_iter();
    arg.next();
    loop {
        let Some(command) = arg.next() else {
            break;
        };
        match command.as_str() {
            "help" => {
                help_command()?;
                return Ok(());
            },
            "--help" => {
                help_command()?;
                return Ok(());
            },
            "--version" => {
                println!("{VERSION}");
                return Ok(());
            },
            _ => {
                compile_command(command.clone())?;
                return Ok(());
            },
        }
    }
    Ok(())
}
