use std::fmt::{Display, write};
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
    Neg,
    Pos
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
        let mut last_token : Option<TokenType> = None;
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
                        // if !stack.is_empty() {
                        //     if stack.last().unwrap() != &Op::Oparen || stack.last().unwrap() != &Op::Neg {
                        //         println!("Error: Unexpected OParen found {} at {}",token.literal, token.get_loc_string());
                        //         exit(-1);
                        //     }
                        // }
                    }
                },
                TokenType::Plus | TokenType::Minus | TokenType::Multi | TokenType::Devide => {
                    let op;
                    if last_token.is_none(){
                        op = Op::Neg;
                    } else {
                        match last_token.unwrap() {
                            TokenType::Plus | TokenType::Minus | TokenType::Multi | TokenType::Devide| TokenType::OParen => {
                                op = Op::Neg;
                            },
                            _ => {
                                op = Op::from_token_type(&token);
                            }
                        } 
                    }
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
            last_token = Some(token.t_type);
        }
        while !stack.is_empty() {
            let op = stack.pop().unwrap();
            Self::parse_expr_stack(&mut expr_stack, &op).unwrap();
        }
        expr_stack.pop().unwrap()
    }

    pub fn debug(expr: &Expr) -> String {
        match expr {
            Self::Unary(u) => format!("({}{})",u.op,Self::debug(u.right.as_ref())),
            Self::Binary(b) => format!("({} {} {})",Self::debug(b.left.as_ref()),b.op,Self::debug(b.right.as_ref())),
            Self::Int(val) => format!("{}",val),
            _ => {todo!()} 
        }
    }

    fn parse_expr_stack(expr_stack: &mut Vec<Expr>, op: &Op) -> Result<(),String> {
        if op == &Op::Oparen {
            return Ok(());
        }
        if op == &Op::Neg {
            let Some(right) = expr_stack.pop() else { return Err("Wrong Unary Definition".to_string()); };
            expr_stack.push(Expr::Unary(UnaryExpr{
                right: Box::new(right),
                op: op.to_owned(), 
            }));
        } else {
            let Some(right) = expr_stack.pop() else { return Err("Missing Left Side".to_string()); };
            let Some(left) = expr_stack.pop() else { return Err("Missing Right Side".to_string()) };
            expr_stack.push(Expr::Binary(BinaryExpr{
                right: Box::new(right),
                op: op.to_owned(), 
                left: Box::new(left)
            }));
        }
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
#[cfg(test)]
mod expr_parsing_tests {
    use crate::{lexer::Lexer, Expr};

    #[test]
    fn single_expr() {
        let mut lexer = Lexer::new(String::new(),"1;".to_string());
        let expr_parser = Expr::parse(&mut lexer);
        assert_eq!(Expr::debug(&expr_parser), "1".to_string());

        let mut lexer = Lexer::new(String::new(),"-1;".to_string());
        let expr_parser = Expr::parse(&mut lexer);
        assert_eq!(Expr::debug(&expr_parser), "(-1)".to_string());

        let mut lexer = Lexer::new(String::new(),"1+2;".to_string());
        let expr_parser = Expr::parse(&mut lexer);
        assert_eq!(Expr::debug(&expr_parser), "(1 + 2)".to_string());
    }

    #[test]
    fn multi_expr() {
        let mut lexer = Lexer::new(String::new(),"-1 + 2 + 3;".to_string());
        let expr_parser = Expr::parse(&mut lexer);
        assert_eq!(Expr::debug(&expr_parser), "((-1) + (2 + 3))".to_string());

        let mut lexer = Lexer::new(String::new(),"1 + 2 * 3;".to_string());
        let expr_parser = Expr::parse(&mut lexer);
        assert_eq!(Expr::debug(&expr_parser), "(1 + (2 * 3))".to_string());

        let mut lexer = Lexer::new(String::new(),"2 / 3 * (-3 + -11);".to_string());
        let expr_parser = Expr::parse(&mut lexer);
        assert_eq!(Expr::debug(&expr_parser), "(2 / (3 * ((-3) + (-11))))".to_string());
    }
}

fn main() -> Result<(),Box<dyn Error>> {
    let source = "2 / 3 * (-3 + -11);".to_string();
    let mut lexer = Lexer::new(String::new(),source);
    let expr_parser = Expr::parse(&mut lexer);
    println!("{:?}",Expr::debug(&expr_parser));
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
