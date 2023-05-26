use crate::lexer::{expect_token,Lexer,TToken};
use crate::parser::{program::Node, definition::Type};


#[derive(Debug,PartialEq)]
pub struct VariableDelclear {
    pub is_const: bool,
    pub is_static: bool,
    pub ident: String,
    pub kind: Type,
    // TODO: Change this shit
    pub init_value: String,
}

// [ident, ident, semicolon]
// [ident, ident, colon, colon, value, semicolon]
// [ident, ident, colon, value, semicolon]
// [ident, ident, equal, value, semicolon]
// [ident, colon, value, semicolon]
// [ident, equal, value, semicolon]
impl VariableDelclear {
    pub fn new(lexer: &mut Lexer) -> Self {
        let is_const: bool;
        let is_static: bool;
        let ident: String;
        let kind: Type;
        let init_value: String;
        let mut token = expect_token(lexer, vec![TToken::Identifier]);
        ident = token.get_literal_string();
        token = expect_token(lexer, vec![TToken::Identifier,TToken::COLON,TToken::EQ]);
        match token.ttype {
            TToken::Identifier => {
                kind = Type { name: token.get_literal_string() };
                token = expect_token(lexer, vec![TToken::COLON,TToken::EQ,TToken::SEMICOLON]);
                match token.ttype {
                    TToken::COLON => {
                        token = expect_token(lexer, vec![TToken::COLON,TToken::Number,TToken::StringLiteral,TToken::CharLiteral,TToken::Identifier]);
                        if token.ttype == TToken::COLON {
                            is_const = true;
                            is_static = true;
                            token = expect_token(lexer, vec![TToken::Number,TToken::StringLiteral,TToken::CharLiteral,TToken::Identifier]);
                        }else {
                            is_const = true;
                            is_static = false;
                        }
                        init_value = token.get_literal_string();
                        expect_token(lexer, vec![TToken::SEMICOLON]);
                    },
                    TToken::EQ => {
                        is_const = false;
                        is_static = false;
                        token = expect_token(lexer, vec![TToken::Number,TToken::StringLiteral,TToken::CharLiteral,TToken::Identifier]);
                        init_value = token.get_literal_string();
                        expect_token(lexer, vec![TToken::SEMICOLON]);
                    },
                    TToken::SEMICOLON => {
                        is_const = false;
                        is_static = false;
                        init_value = String::new();
                    },
                    _ => {unreachable!();}
                }
            },
            TToken::COLON => {
                kind = Type { name: "undifiend".to_string() };
                is_const = true;
                is_static = false;
                token = expect_token(lexer, vec![TToken::Number,TToken::StringLiteral,TToken::CharLiteral,TToken::Identifier]);
                init_value = token.get_literal_string();
                expect_token(lexer, vec![TToken::SEMICOLON]);
            },
            TToken::EQ => {
                kind = Type { name: "undifiend".to_string() };
                is_const = false;
                is_static = false;
                token = expect_token(lexer, vec![TToken::Number,TToken::StringLiteral,TToken::CharLiteral,TToken::Identifier]);
                init_value = token.get_literal_string();
                expect_token(lexer, vec![TToken::SEMICOLON]);
            },
            _ => {unreachable!();},
        }
        return Self { is_const, is_static, ident, kind, init_value };
    }

    pub fn get_node(lexer: &mut Lexer) -> Node {
        Node::VariableDelclear { var: Self::new(lexer) }
    }
}

