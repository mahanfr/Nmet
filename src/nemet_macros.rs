use crate::{lexer::{TokenType, Lexer, Token}, parser::{expr::{ExprType, Expr, expr}, types::VariableType}, error_handeling::error};

#[derive(Debug, Clone)]
pub struct MacroCall {
    pub ident: String,
    pub call_args: Vec<Expr>,
}

pub fn parse_macro_call(lexer: &mut Lexer) -> MacroCall {
    lexer.match_token(TokenType::Dollar);
    let macro_ident = lexer.get_token().literal;
    lexer.match_token(TokenType::Identifier);
    let mut args = Vec::<Expr>::new();
    loop {
        if lexer.get_token_type() == TokenType::SemiColon {
            lexer.match_token(TokenType::SemiColon);
            break;
        }
        args.push(expr(lexer));
    }
    MacroCall {
        ident: macro_ident,
        call_args: args,
    }
}

#[derive(Debug, Clone)]
pub enum MacroInjection {
    Token(Token),
    Injection,
}

#[derive(Debug, Clone)]
pub struct MacroRule {
    // ExprType
    pub pattern: VariableType,
    pub ingejector: Vec<MacroInjection>
}

#[derive(Debug, Clone)]
pub struct Macro {
    pub args: u8,
    pub rules: Vec<MacroRule>,
}

pub fn parse_macro_def(lexer: &mut Lexer) -> (String,Macro){
    let loc = lexer.get_token_loc();
    lexer.match_token(TokenType::Macro);
    let macro_ident = lexer.get_token().literal;
    lexer.match_token(TokenType::Identifier);
    lexer.match_token(TokenType::Not);
    let match_ident = lexer.get_token().literal;
    let mut times = 0;
    lexer.match_token(TokenType::Identifier);
    if lexer.get_token_type() == TokenType::Plus {
        times = u8::MAX;
        lexer.match_token(TokenType::Plus);
    }
    if lexer.get_token_type() == TokenType::OCurly {
        let macro_body = parse_single_var_macro_body(lexer,match_ident.clone());
        lexer.match_token(TokenType::CCurly);
        (macro_ident.clone(), Macro {
            args: times,
            rules: macro_body,
        })
    } else {
        error("This Type Of Macros are not supported yet!",loc);
    }
}

fn parse_single_var_macro_body(lexer: &mut Lexer,ident: String) -> Vec<MacroRule> {
    lexer.match_token(TokenType::OCurly);
    let mut macro_rules = Vec::<MacroRule>::new();
    loop {
        if lexer.get_token_type() == TokenType::CCurly {
            break;
        }
        if lexer.get_token_type() == TokenType::QMark {
            lexer.match_token(TokenType::QMark);
            let etype = match lexer.get_token().literal.as_str() {
                "str" => VariableType::String,
                "int" => VariableType::Int,
                _ => unimplemented!()
            };
            lexer.match_token(TokenType::Identifier);
            lexer.match_token(TokenType::OCurly);
            loop {
                let mut macro_injects = Vec::<MacroInjection>::new();
                if lexer.get_token_type() == TokenType::CCurly {
                    lexer.match_token(TokenType::CCurly);
                    break;
                } else {
                    if lexer.get_token_type() == TokenType::String &&
                        lexer.get_token().literal == ident {
                        macro_injects.push(MacroInjection::Injection);
                    } else {
                        macro_injects.push(MacroInjection::Token(lexer.get_token()));
                    }
                    lexer.next_token();
                }
                macro_rules.push(MacroRule{pattern: etype.clone(),ingejector: macro_injects});
            }
        } else {
            unimplemented!();
        }
    }
    macro_rules
}

