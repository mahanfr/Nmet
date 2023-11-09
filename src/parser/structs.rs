use crate::{lexer::{Lexer, TokenType}, error_handeling::error};

use super::types::{VariableType, type_def};

pub type StructItem = (String, VariableType);

pub struct StructDef {
    ident: String,
    items: Vec<StructItem>,
}

pub fn struct_def(lexer: &mut Lexer) -> StructDef {
    lexer.match_token(TokenType::Struct);
    let struct_ident_token = lexer.next_token();
    lexer.match_token(TokenType::OCurly);
    let mut items = Vec::<StructItem>::new();
    loop {
        if lexer.get_token_type() == TokenType::CCurly {
            break;
        }
        let ident = lexer.get_token().literal;
        lexer.match_token(TokenType::Identifier);
        if lexer.get_token_type() == TokenType::ATSign {
            let ttype = type_def(lexer);
            items.push((ident,ttype));
        }
        if lexer.get_token_type() != TokenType::CCurly {
            lexer.match_token(TokenType::Comma);
        }
    }
    StructDef {
        ident: struct_ident_token.literal,
        items,
    }
}
