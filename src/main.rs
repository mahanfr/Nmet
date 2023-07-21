use std::{error::Error, process::exit};
use std::fs;
use std::env::args;

mod lexer;
mod ast;
use lexer::{Lexer, TokenType, Token};

// --- Static Compiler Defenition
static VERSION : &'static str = "v0.0.1-Beta";
static COPYRIGHT : &'static str = "Mahan Farzaneh 2023-2024";
static DEBUG : bool = true;

// -4 -> 4 neg
// 4 + 2 -> 4 2 +
// 4 * 3 + 6 -> 4 3 * 6 +
// 4 + (3 + 6) -> 3 6 + 4 +
// -(4 * cos(0) + 2 - 6) -> 4 cos(0) * 2 + 6 - neg

#[derive(Debug,PartialEq,Clone)]
enum Op {
    Plus,
    Sub,
    Multi,
    Devide,
    Oparen,
    Cparen,
}
impl Op {
    pub fn from_token_type(token: &Token) -> Self {
        match token.t_type {
            TokenType::Plus => return Self::Plus,
            TokenType::Minus => return Self::Sub,
            TokenType::Multi => return Self::Multi,
            TokenType::Devide => return Self::Devide,
            _ => {
                println!("Error: Unexpected Op Token ({}) at {}",token.literal,token.get_loc_string());
                exit(-1);
            }
        }
    }

    pub fn get_op_precedence(op: &Op) -> u8 {
        match op {
            Op::Plus | Op::Sub => 1,
            Op::Multi | Op::Devide => 2,
            Op::Oparen | Op::Cparen => 0,
        }
    }
}


#[derive(Debug,PartialEq,Clone)]
struct UnaryExpr {
    op: Op,
    right: Box<Expr>
}

#[derive(Debug,PartialEq,Clone)]
struct BinaryExpr {
    left: Box<Expr>,
    op: Op,
    right: Box<Expr>
}

#[derive(Debug,PartialEq,Clone)]
struct FunctionCall {
    identifier: String,
    args: Vec<Expr>,
}

#[derive(Debug,PartialEq,Clone)]
struct ArrayIndex {
    identifier: String,
    indexer: Box<Expr>,
}

#[derive(Debug,PartialEq,Clone)]
enum Expr {
    Unary(UnaryExpr),
    Binary(BinaryExpr),
    Int(i32),
    Variable(String),
    FunctionCall(FunctionCall),
    ArrayIndex(ArrayIndex),
}
impl Expr {
    pub fn parse(lexer: &mut Lexer) -> Expr {
        let mut stack = Vec::<Op>::new();
        let mut expr_stack = Vec::<Expr>::new();
        loop {
            println!("stack: {:?}\n\nexpr stack: {:?}",stack,expr_stack);
            let token = lexer.expect_some_token();
            if token.t_type == TokenType::SemiColon {
                break;
            }
            
            match token.t_type {
                TokenType::Int(val) => {
                    expr_stack.push(Expr::Int(val));
                },
                TokenType::OParen => {
                    stack.push(Op::Oparen);
                },
                TokenType::CParen => {
                    while !stack.is_empty() && stack.last().unwrap() != &Op::Oparen {
                        let op = stack.pop().unwrap();
                        Self::parse_expr_stack(&mut expr_stack, &op).unwrap();
                        if !stack.is_empty() && stack.last().unwrap() != &Op::Oparen {
                            println!("Error: Unexpected OParen found {} at {}",token.literal, token.get_loc_string());
                            exit(-1);
                        }
                    }
                },
                TokenType::Plus | TokenType::Minus | TokenType::Multi | TokenType::Devide => {
                    let op = Op::from_token_type(&token);
                    while !stack.is_empty() && Op::get_op_precedence(stack.last().unwrap()) > Op::get_op_precedence(&op)  {
                        Self::parse_expr_stack(&mut expr_stack, &stack.pop().unwrap()).unwrap();
                    }
                    stack.push(op);
                },
                _ => {
                    println!("Error: Unexpected token ({}) at {}",token.literal, token.get_loc_string());
                    exit(-1);
                }
            }
        }
        while !stack.is_empty() {
            let op = stack.pop().unwrap();
            Self::parse_expr_stack(&mut expr_stack, &op).unwrap();
        }
        expr_stack.pop().unwrap()
    }

    pub fn parse_expr_stack(expr_stack: &mut Vec<Expr>, op: &Op) -> Result<(),()> {
        if op == &Op::Oparen {
            return Ok(());
        }
        let Some(right) = expr_stack.pop() else { return Err(()); };
        let Some(left) = expr_stack.pop() else { return Err(()); };
        expr_stack.push(Expr::Binary(BinaryExpr{
            right: Box::new(right), 
            op: op.to_owned(), 
            left: Box::new(left)
        }));
        Ok(())
    }
}

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

fn main() -> Result<(),Box<dyn Error>> {
    let source = "((1 + 2) * (3 + 1));".to_string();
    let mut lexer = Lexer::new(String::new(),source);
    let expr_parser = Expr::parse(&mut lexer);
    println!("{:?}",expr_parser);
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
