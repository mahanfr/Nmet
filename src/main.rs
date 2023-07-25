use std::{error::Error, process::exit};
use std::fs;
use std::env::args;

mod lexer;
mod ast;
mod parser;
use lexer::{Lexer, TokenType};
use parser::{Expr, Op, UnaryExpr, BinaryExpr, ArrayIndex};

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
                    let indexer = array_indexer(lexer);
                    return Expr::ArrayIndex(
                        ArrayIndex {
                            identifier: ident_name, 
                            indexer: Box::new(indexer)
                        });
                },
                _ => {
                    return Expr::Variable(ident_name);
                }
            }
        }
        _ => {
            eprintln!("Unexpected Token ({:?}) while parsing expr at {}",lexer.get_token_type(),lexer.get_loc_string());
            exit(-1);
        }
    }
}

pub fn array_indexer(lexer: &mut Lexer) -> Expr {
    lexer.match_token(TokenType::OBracket);
    let index = expr(lexer);
    lexer.match_token(TokenType::CBracket);
    return index;
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

#[derive(Debug)]
pub struct StaticVariable {
    ident: String,
    value: Expr,
}

pub fn variable_def(lexer: &mut Lexer) {

}

#[derive(Debug)]
pub struct FunctionArg {
    identifier: String,
}

#[derive(Debug)]
pub struct Function {
    identifier: String,
    args: Vec<FunctionArg>,
    block: Block,
}

#[derive(Debug)]
enum Stmts {}

#[derive(Debug)]
pub struct Block {
    stmts: Vec<Stmts>,
}

pub fn function_def(lexer: &mut Lexer) -> Function {
    lexer.match_token(TokenType::Fun);
    let Some(function_ident_token) = lexer.get_token() else {
        eprintln!("Function Defenition without Identifier at {}",lexer.get_loc_string());
        exit(-1);
    };
    lexer.match_token(TokenType::Identifier);
    let args = function_def_args(lexer);
    let block = block(lexer);
    return Function {
        identifier: function_ident_token.literal.to_string(),
        args,
        block,
    }
}

pub fn block(lexer: &mut Lexer) -> Block {
    lexer.match_token(TokenType::OCurly);
    let stmts = Vec::<Stmts>::new();
    loop {
        if lexer.get_token_type() == TokenType::CCurly {
            break;
        }
        // TODO:
        lexer.next_token();
    }
    lexer.match_token(TokenType::CCurly);
    return Block{stmts}
}

pub fn function_def_args(lexer: &mut Lexer) -> Vec<FunctionArg> {
    let mut args = Vec::<FunctionArg>::new();
    lexer.match_token(TokenType::OParen);
    loop {
        match lexer.get_token_type() {
            TokenType::CParen => {
                lexer.match_token(TokenType::CParen);
                break;
            },
            TokenType::Identifier => {
                let ident = lexer.get_token().unwrap().literal;
                args.push(FunctionArg{identifier: ident.to_string()});
                lexer.match_token(TokenType::Identifier);
                if lexer.get_token_type() == TokenType::Comma {
                    lexer.match_token(TokenType::Comma);
                }
            },
            _ => {
                eprintln!("Error: Expected Identifier found ({:?}) at {}",lexer.get_token_type(),lexer.get_loc_string());
                exit(-1);
            }
        }
    }
    args
}

#[derive(Debug)]
pub enum ProgramItem {
    Func(Function),
    StaticVar(StaticVariable),
}

#[derive(Debug)]
pub struct ProgramFile {
    shebang: String,
    file_path: String,
    // attrs: Vec<Attr>
    items: Vec<ProgramItem>,
}

pub fn static_variable_def(lexer: &mut Lexer) -> StaticVariable {
    let Some(ident_token) = lexer.get_token() else {
        eprintln!("Error: Expected Identifier found Eof at {}",lexer.get_loc_string());
        exit(-1);
    };
    lexer.match_token(TokenType::Identifier);
    lexer.match_token(TokenType::DoubleColon);
    let expr = expr(lexer);
    lexer.match_token(TokenType::SemiColon);
    return StaticVariable {ident: ident_token.literal, value: expr};
}

pub fn program(lexer: &mut Lexer) -> ProgramFile {
    lexer.next_token();
    let mut items = Vec::<ProgramItem>::new();
    loop {
        if lexer.get_token().is_none() { break; }
        match lexer.get_token_type() {
            TokenType::Fun => {
                items.push(ProgramItem::Func(function_def(lexer)));
            },
            TokenType::Identifier => {
                items.push(ProgramItem::StaticVar(static_variable_def(lexer)));
            },
            _ => {
                eprintln!("Error: Unexpected Token ({:?}) for top level program at {}",
                    lexer.get_token_type(),
                    lexer.get_loc_string()
                );
                exit(-1);
            }
        }
    }
    return ProgramFile{
        shebang: String::new(),
        file_path: lexer.file_path.clone(),
        items,
    }
}

fn main() -> Result<(),Box<dyn Error>> {
    let source = 
        r#"
            a :: 1 + 2;
            fun main(args,kwargs) {}
        "#;
    let mut lexer = Lexer::new(String::new(),source.to_string());
    // TODO: Move to top root of parser
    // lexer.next_token();
    // println!("{:?}",expr(&mut lexer));
    println!("{:#?}",program(&mut lexer));
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
