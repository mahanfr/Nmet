use std::collections::BTreeMap;

/**********************************************************************************************
*
*   parser/structs: parsing strucure defenitions
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
use crate::lexer::{Lexer, TokenType};

use super::types::{type_def, StructItemType, StructType};

pub fn struct_def(lexer: &mut Lexer) -> StructType {
    lexer.match_token(TokenType::Struct);
    let struct_ident_token = lexer.get_token();
    lexer.match_token(TokenType::Identifier);
    lexer.match_token(TokenType::OCurly);
    let mut items = BTreeMap::<String, StructItemType>::new();
    let mut offset = 0;
    loop {
        if lexer.get_token_type() == TokenType::CCurly {
            lexer.match_token(TokenType::CCurly);
            break;
        }
        let ident = lexer.get_token().literal;
        lexer.match_token(TokenType::Identifier);
        if lexer.get_token_type() == TokenType::ATSign {
            let ttype = type_def(lexer);
            offset += ttype.item_size();
            items.insert(
                ident.clone(),
                StructItemType::new(ident.clone(), offset as i32, ttype),
            );
        }
        if lexer.get_token_type() != TokenType::CCurly {
            lexer.match_token(TokenType::Comma);
        }
    }
    StructType {
        ident: struct_ident_token.literal,
        items,
    }
}
