use crate::exit;
use crate::lexer::{Lexer, TToken};
use crate::parser::{function::Func, variable::VariableDelclear};

#[derive(Debug)]
pub enum Node {
    Func {var: Func},
    VariableDelclear { var: VariableDelclear }
}

#[derive(Debug)]
pub struct Program {
    pub shebang: String,
    pub body: Vec<Node>,
}

impl Program {
    pub fn new(lexer: &mut Lexer) -> Self {
        let mut body = Vec::<Node>::new();
        loop {
            let token = lexer.next_token();
            if token.ttype == TToken::Fun {
                body.push(Func::get_node(lexer));
            }else if token.ttype == TToken::ATSIGN {
                body.push(VariableDelclear::get_node(lexer));
            }else if token.ttype == TToken::EOF {
                break;
            }else {
                println!("Syntax error unexpected token ({:?}) at {}:{}:{}",token.ttype,token.file_path,token.line,token.col);
                exit(1);
            }
        }
        Self {
            shebang: String::new(),
            body,
        }
    }
}
