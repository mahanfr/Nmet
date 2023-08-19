use std::process::exit;

use crate::{
    lexer::{Lexer, TokenType},
    parser::{block::Block, types::type_def},
};

use super::{block::block, types::VariableType};

#[derive(Debug, Clone)]
pub struct FunctionArg {
    pub ident: String,
    pub typedef: VariableType,
}

#[derive(Debug, Clone)]
pub struct Function {
    pub ident: String,
    pub args: Vec<FunctionArg>,
    pub block: Block,
    pub ret_type: Option<VariableType>,
}

pub fn function_def(lexer: &mut Lexer) -> Function {
    lexer.match_token(TokenType::Func);
    let function_ident_token = lexer.get_token();
    let mut ret_type: Option<VariableType> = None;
    if function_ident_token.is_empty() {
        eprintln!(
            "Function Defenition without Identifier at {}:{}",
            lexer.file_path,
            lexer.get_token_loc()
        );
        exit(-1);
    }
    lexer.match_token(TokenType::Identifier);
    let args = function_def_args(lexer);
    if lexer.get_token_type() == TokenType::ATSign {
        ret_type = Some(type_def(lexer));
    }
    let block = block(lexer);
    Function {
        ident: function_ident_token.literal,
        ret_type,
        args,
        block,
    }
}

pub fn function_def_args(lexer: &mut Lexer) -> Vec<FunctionArg> {
    let mut args = Vec::<FunctionArg>::new();
    lexer.match_token(TokenType::OParen);
    loop {
        match lexer.get_token_type() {
            TokenType::CParen => {
                lexer.match_token(TokenType::CParen);
                break;
            }
            TokenType::Identifier => {
                let ident = lexer.get_token().literal;
                lexer.match_token(TokenType::Identifier);
                let typedef = type_def(lexer);
                if lexer.get_token_type() == TokenType::Comma {
                    lexer.match_token(TokenType::Comma);
                }
                args.push(FunctionArg {
                    ident: ident.to_string(),
                    typedef,
                });
            }
            _ => {
                eprintln!(
                    "Error: Expected Identifier found ({:?}) at {}:{}",
                    lexer.get_token_type(),
                    lexer.file_path,
                    lexer.get_token_loc()
                );
                exit(-1);
            }
        }
    }
    args
}
