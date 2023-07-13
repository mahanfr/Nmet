use std::process::exit;

use crate::lexer::{TokenType, Token};
use crate::ast::AsmParsable;

struct Binary {
    left: Token,
    op: Token,
    right: Token
}

impl Binary {
    pub fn new(left: Token, op:Token, right:Token) -> Self {
        Self { left, op, right }
    }
}

impl AsmParsable for Binary {
    fn parse_to_asm(&self) -> String {
        /*
         *  mv eax ,left.val
         *  [add,sub,imul,idiv] eax, right.val
         *  push eax
         * */
        let mut code_buffer = String::new();
        if let TokenType::Int(lval) = self.left.t_type {
            let asm_line = format!("mov eax, {}\n",lval);
            code_buffer.push_str(asm_line.as_str());
        } else {
            println!("Error: Invalid token at {}:{}",self.left.line,self.left.col);
            exit(-1);
        };
        let TokenType::Int(rval) = self.right.t_type else {
            println!("Error: Invalid token at {}:{}",self.right.line,self.right.col);
            exit(-1);
        };
        match self.op.t_type {
            TokenType::Plus => {
                let asm_line = format!("add eax, {}\n",rval);
                code_buffer.push_str(asm_line.as_str());
            },
            TokenType::Minus => {
                let asm_line = format!("sub eax, {}\n",rval);
                code_buffer.push_str(asm_line.as_str());
            },
            TokenType::Multi => {
                let asm_line = format!("imul eax, {}\n",rval);
                code_buffer.push_str(asm_line.as_str());
            },
            TokenType::Devide => {
                let asm_line = format!("idiv eax, {}\n",rval);
                code_buffer.push_str(asm_line.as_str());
            },
            _ => {
                println!("Error: Invalid operand ({}) at {}:{}",self.op.literal,self.op.line,self.op.col);
                exit(-1);
            }
        }
        code_buffer.push_str("push eax\n");
        code_buffer
    }
}

