
use std::{fmt::Display, process::exit};

use crate::lexer::{TokenType, Lexer};
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
    Not,
}
impl Op {
    pub fn from_token_type(token_type: TokenType) -> Self {
        match token_type {
            TokenType::Plus => return Self::Plus,
            TokenType::Minus => return Self::Sub,
            TokenType::Multi => return Self::Multi,
            TokenType::Devide => return Self::Devide,
            TokenType::Not => return Self::Not,
            _ => {
                todo!("return Error");
            }
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
           Op::Not => write!(f,"!"),
       } 
    }
}

#[derive(Debug)]
pub struct StaticVariable {
    pub ident: String,
    pub value: Expr,
}

#[derive(Debug)]
pub struct FunctionArg {
    pub ident: String,
}

#[derive(Debug)]
pub struct Function {
    pub ident: String,
    pub args: Vec<FunctionArg>,
    pub block: Block,
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
    pub ident: String,
    pub args: Vec<Expr>,
}

#[derive(Debug,PartialEq,Clone)]
pub struct ArrayIndex {
    pub ident: String,
    pub indexer: Box<Expr>,
}

#[derive(Debug,PartialEq,Clone)]
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

#[derive(Debug,PartialEq,Clone)]
pub struct CompareExpr {
    pub left: Box<Expr>,
    pub op: CompareOp,
    pub right: Box<Expr>,
}

#[derive(Debug,PartialEq,Clone)]
pub enum Expr {
    Unary(UnaryExpr),
    Binary(BinaryExpr),
    Compare(CompareExpr),
    Int(i32),
    Variable(String),
    FunctionCall(FunctionCall),
    ArrayIndex(ArrayIndex),
}
impl Expr {
    pub fn is_binary_op(t_token: TokenType) -> bool {
        match t_token {
            TokenType::Plus | TokenType::Minus => true,
            _ => false,
        }
    }

    pub fn is_compare_op(t_token: TokenType) -> bool {
        match t_token {
            TokenType::DoubleEq | TokenType::NotEq | 
                TokenType::Bigger | TokenType::Smaller |
                TokenType::BiggerEq | TokenType::SmallerEq => true,
            _ => false,
        }
    }
}

#[derive(Debug)]
pub enum AssginOp {
    Eq,
}

#[derive(Debug)]
pub struct Assgin {
    pub left : Expr,
    pub right: Expr,
    pub op: AssginOp,
}

#[derive(Debug)]
pub enum ElseBlock {
    Elif(IFStmt),
    Else(Block),
    None,
}
impl ElseBlock {
    pub fn is_none(&self) -> bool {
        match self {
            Self::None => true,
            _ => false,
        }
    }

    pub fn is_else(&self) -> bool {
        match self {
            Self::Else(_) => true,
            _ => false,
        }
    }

    pub fn is_elif(&self) -> bool {
        match self {
            Self::Elif(_) => true,
            _ => false,
        }
    }

}


#[derive(Debug)]
pub struct IFStmt {
    pub condition: Expr,
    pub then_block: Block,
    pub else_block: Box<ElseBlock>,
}


#[derive(Debug)]
pub enum Stmt {
    // expr
    Expr(Expr),
    VariableDecl(VariableDeclare),
    // expr = expr
    Assgin(Assgin),
    Print(Expr),
    While(WhileStmt),
    If(IFStmt),
    Return(Expr),
    Break,
    Continue,
}

#[derive(Debug)]
pub struct WhileStmt {
    pub condition: Expr,
    pub block: Block,
}

#[derive(Debug)]
pub struct Block {
    pub stmts: Vec<Stmt>,
}

#[derive(Debug)]
pub struct VariableDeclare {
    pub mutable: bool,
    pub ident: String,
    pub init_value: Option<Expr>,
}

#[derive(Debug)]
pub enum ProgramItem {
    Func(Function),
    StaticVar(StaticVariable),
}

#[derive(Debug)]
pub struct ProgramFile {
    pub shebang: String,
    pub file_path: String,
    // pub attrs: Vec<Attr>
    pub items: Vec<ProgramItem>,
}

pub fn expr(lexer: &mut Lexer) -> Expr {
    let mut term_expr = term(lexer);
    loop {
        let t_type = lexer.get_token_type();
        if Expr::is_binary_op(t_type) {
            let op = Op::from_token_type(t_type);
            lexer.next_token();
            let right = term(lexer);
            term_expr = Expr::Binary(BinaryExpr {left: Box::new(term_expr), op, right: Box::new(right)});
        } else if Expr::is_compare_op(t_type) {
            let op = CompareOp::from_token_type(lexer.get_token_type());
            lexer.next_token();
            let right = term(lexer);
            term_expr = Expr::Compare(CompareExpr {left: Box::new(term_expr), op, right: Box::new(right)});
        } 
        else {break;}
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
        TokenType::Plus | TokenType::Minus | TokenType::Not => {
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
                    return Expr::FunctionCall(FunctionCall{ident: ident_name, args});
                },
                TokenType::OBracket => {
                    let indexer = array_indexer(lexer);
                    return Expr::ArrayIndex(
                        ArrayIndex {
                            ident: ident_name, 
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
        ident: function_ident_token.literal.to_string(),
        args,
        block,
    }
}

/*
 * Stmt := {declare | expr { = expr}} ;
 * declare := let Ident = expr;
*/

pub fn if_stmt(lexer: &mut Lexer) -> IFStmt {
    lexer.match_token(TokenType::If);
    let condition = expr(lexer);
    let then_block = block(lexer);
    if lexer.get_token_type() == TokenType::Else {
        lexer.match_token(TokenType::Else);
        if lexer.get_token_type() == TokenType::If {
            let else_block = Box::new(ElseBlock::Elif(if_stmt(lexer)));
            return IFStmt {condition, then_block, else_block};
        } else {
            let else_block = Box::new(ElseBlock::Else(block(lexer)));
            return IFStmt {condition, then_block, else_block};
        }
    } else {
        return IFStmt {condition, then_block, else_block: Box::new(ElseBlock::None)};
    }
}

pub fn while_stmt(lexer: &mut Lexer) -> WhileStmt {
    lexer.match_token(TokenType::While);
    let condition = expr(lexer);
    let block = block(lexer);
    return WhileStmt{condition,block};
}

pub fn block(lexer: &mut Lexer) -> Block {
    lexer.match_token(TokenType::OCurly);
    let mut stmts = Vec::<Stmt>::new();
    loop {
        if lexer.get_token_type() == TokenType::CCurly { break; }
        match lexer.get_token_type() {
            TokenType::Let => {
                stmts.push(variable_declare(lexer));
                lexer.match_token(TokenType::SemiColon);
            },
            TokenType::Print => {
                lexer.match_token(TokenType::Print);
                let expr = expr(lexer);
                stmts.push(Stmt::Print(expr));
                lexer.match_token(TokenType::SemiColon);
            },
            TokenType::Break => {
                lexer.match_token(TokenType::Break);
                stmts.push(Stmt::Break);
                lexer.match_token(TokenType::SemiColon);
            },
            TokenType::Continue => {
                lexer.match_token(TokenType::Continue);
                stmts.push(Stmt::Continue);
                lexer.match_token(TokenType::SemiColon);
            },
            TokenType::If => {
                stmts.push(Stmt::If(if_stmt(lexer)));
            },
            TokenType::While => {
                stmts.push(Stmt::While(while_stmt(lexer)));
            },
            TokenType::Return => {
                lexer.match_token(TokenType::Return);
                stmts.push(Stmt::Return(expr(lexer)));
                lexer.match_token(TokenType::SemiColon);
            },
            _ => {
                let left_expr = expr(lexer);
                match lexer.get_token_type() {
                    TokenType::Eq => {
                        lexer.match_token(TokenType::Eq);
                        let right_expr = expr(lexer);
                        stmts.push(Stmt::Assgin(Assgin {
                            left: left_expr,
                            right: right_expr,
                            op: AssginOp::Eq,
                        }));
                    },
                    TokenType::SemiColon => {
                        stmts.push(Stmt::Expr(left_expr));
                    },
                    _ => {
                        eprintln!("Error: Expected Semicolon at {}",lexer.get_loc_string());
                        exit(-1);
                    }
                }
                lexer.match_token(TokenType::SemiColon);
            }
        }
    }
    lexer.match_token(TokenType::CCurly);
    return Block{stmts}
}

pub fn variable_declare(lexer: &mut Lexer) -> Stmt {
   lexer.match_token(TokenType::Let);
   let ident_token = lexer.get_token().unwrap();
   lexer.match_token(TokenType::Identifier);
   let mut is_mutable: bool = true;
   let mut init_value: Option<Expr> = None;
   match lexer.get_token_type() {
        TokenType::ColonEq => {
            is_mutable = false;
            lexer.match_token(TokenType::ColonEq);
            init_value = Some(expr(lexer));
        },
        TokenType::Eq => {
            is_mutable = true;
            lexer.match_token(TokenType::Eq);
            init_value = Some(expr(lexer));
        },
        TokenType::SemiColon => {},
        _ => {
            eprintln!("Error: Expected \"=\" or \":=\" found ({:?}) at {}",
                lexer.get_token_type(),
                lexer.get_loc_string()
            );
            exit(-1);
        }
   }
   return Stmt::VariableDecl(VariableDeclare {
       mutable: is_mutable,
       ident: ident_token.literal,
       init_value
   });

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
                args.push(FunctionArg{ident: ident.to_string()});
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

pub fn static_variable_def(lexer: &mut Lexer) -> StaticVariable {
    lexer.match_token(TokenType::Let);
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
            TokenType::Let => {
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

