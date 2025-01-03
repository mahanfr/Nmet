/**********************************************************************************************
*
*   parser/funtion: parse function syntax
*
*   LICENSE: MIT
*
*   Copyright (c) 2023-2024 Mahan Farzaneh (@mahanfr)
*
*   This software is provided "as-is", without any express or implied warranty. In no event
*   will the authors be held liable for any damages arising from the use of this software.
*
*   Permission is granted to anyone to use this software for any purpose, including commercial
*   applications, and to alter it and redistribute it freely, subject to the following restrictions:
*
*     1. The origin of this software must not be misrepresented; you must not claim that you
*     wrote the original software. If you use this software in a product, an acknowledgment
*     in the product documentation would be appreciated but is not required.
*
*     2. Altered source versions must be plainly marked as such, and must not be misrepresented
*     as being the original software.
*
*     3. This notice may not be removed or altered from any source distribution.
*
**********************************************************************************************/
use crate::{
    error_handeling::error,
    lexer::{Lexer, TokenType},
    parser::{block::Block, types::type_def},
};

use super::{block::BlockType, types::VariableType};

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
pub struct FunctionDef {
    pub decl: FunctionDecl,
    pub block: Block,
    pub defer_block: Block,
}

#[derive(Debug, Clone)]
pub struct FunctionDecl {
    pub is_extern: bool,
    pub ident: String,
    pub args: Vec<FunctionArg>,
    pub ret_type: VariableType,
}

pub fn parse_function_declaration(lexer: &mut Lexer) -> FunctionDecl {
    let loc = lexer.get_current_loc();
    let is_extern = match lexer.get_token_type() {
        TokenType::Extern => {
            lexer.match_token(TokenType::Extern);
            true
        }
        _ => false,
    };
    lexer.match_token(TokenType::Func);
    let function_ident_token = lexer.get_token();
    let mut ret_type = VariableType::Void;
    let fn_ident = match function_ident_token.is_empty() {
        true => error("Function defenition without identifier", loc),
        false => function_ident_token.literal,
    };
    lexer.match_token(TokenType::Identifier);
    let args = function_def_args(lexer);
    if lexer.get_token_type() == TokenType::ATSign {
        ret_type = type_def(lexer);
    }
    FunctionDecl {
        is_extern,
        ident: fn_ident,
        args,
        ret_type,
    }
}

/// Parsing Function definition
pub fn parse_function_definition(lexer: &mut Lexer) -> FunctionDef {
    let decl = parse_function_declaration(lexer);
    let mut block = Block::new_global(decl.ident.clone(), BlockType::Function);
    block.parse_block(lexer);
    let mut defer_block = Block::new_global(decl.ident.clone(), BlockType::Function);
    defer_block.stmts = block.defer_stmts.clone();
    block.defer_stmts.clear();
    FunctionDef {
        decl,
        block,
        defer_block,
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
