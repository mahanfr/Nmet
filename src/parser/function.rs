use crate::{
    error_handeling::error,
    lexer::{Lexer, TokenType},
    parser::{block::Block, types::type_def},
};

use super::{block::block, types::VariableType};

/// Function Definition Arguments
/// * ident - name of argument in function name space
/// * typedef - type of acceptable argument
#[derive(Debug, Clone)]
pub struct FunctionArg {
    pub ident: String,
    pub typedef: VariableType,
}

/// Function Definition
/// * ident - name of the function
/// * args - list of all function arguments
/// * block - function block
/// * typedef - type of acceptable
#[derive(Debug, Clone)]
pub struct Function {
    pub ident: String,
    pub args: Vec<FunctionArg>,
    pub block: Block,
    pub ret_type: VariableType,
}

/// Parsing Function definition
pub fn function_def(lexer: &mut Lexer) -> Function {
    let loc = lexer.get_current_loc();
    lexer.match_token(TokenType::Func);
    let function_ident_token = lexer.get_token();
    let mut ret_type = VariableType::Void;
    if function_ident_token.is_empty() {
        error("Function defenition without identifier", loc);
    }
    lexer.match_token(TokenType::Identifier);
    let args = function_def_args(lexer);
    if lexer.get_token_type() == TokenType::ATSign {
        ret_type = type_def(lexer);
    }
    let block = block(lexer);
    Function {
        ident: function_ident_token.literal,
        ret_type,
        args,
        block,
    }
}

/// Parsing Function definition
/// returns list of function definition arguments
pub fn function_def_args(lexer: &mut Lexer) -> Vec<FunctionArg> {
    let loc = lexer.get_current_loc();
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
                error(
                    format!("Expected Identifier found ({})", lexer.get_token_type()),
                    loc,
                );
            }
        }
    }
    args
}
