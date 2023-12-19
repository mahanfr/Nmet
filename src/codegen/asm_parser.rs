use std::str::FromStr;

use crate::lexer::{Lexer, TokenType};

use super::{
    memory::{Mem, MemOp},
    mnmemonic::Mnemonic,
    register::Reg,
};

pub fn parse_asm(source: String) -> String {
    let mut lexer = Lexer::new(format!("asm:{source}"), source);
    let mnmemonic = parse_mnemonic(&mut lexer);
    let mut ops = Vec::new();
    let mut airty = 0;
    loop {
        if lexer.get_token().is_empty() {
            break;
        }
        ops.push(parse_op(&mut lexer));
        airty += 1;
        if lexer.get_token_type() == TokenType::Comma {
            lexer.match_token(TokenType::Comma);
            continue;
        } else {
            break;
        }
    }
    //mnmemonic.to_string()
    if airty == 2 {
        format!("{mnmemonic} {}, {}", ops[0], ops[1])
    } else if airty == 1 {
        format!("{mnmemonic} {}", ops[0])
    } else {
        format!("{mnmemonic}")
    }
}

fn parse_mnemonic(lexer: &mut Lexer) -> Mnemonic {
    let ident = lexer.next_token();
    lexer.match_token(TokenType::Identifier);
    Mnemonic::from_str(&ident.literal).unwrap()
}

fn parse_op(lexer: &mut Lexer) -> String {
    match lexer.get_token_type() {
        TokenType::Identifier => {
            match Reg::from_str(&lexer.get_token().literal) {
                Ok(reg) => {
                    lexer.match_token(TokenType::Identifier);
                    return reg.to_string();
                }
                Err(_) => (),
            }
            match lexer.get_token().literal.to_lowercase().as_str() {
                "qword" => {
                    lexer.match_token(TokenType::Identifier);
                    return Mem::Qword(parse_memop(lexer)).to_string();
                }
                "dword" => {
                    lexer.match_token(TokenType::Identifier);
                    return Mem::Dword(parse_memop(lexer)).to_string();
                }
                "word" => {
                    lexer.match_token(TokenType::Identifier);
                    return Mem::Word(parse_memop(lexer)).to_string();
                }
                "byte" => {
                    lexer.match_token(TokenType::Identifier);
                    return Mem::Byte(parse_memop(lexer)).to_string();
                }
                _ => panic!("Unexpected Asm Operation {}!", lexer.get_token().literal),
            }
        }
        TokenType::OBracket => {
            return Mem::U(parse_memop(lexer)).to_string();
        }
        TokenType::Int(i) => i.to_string(),
        _ => panic!("Unsupported ASM operation!"),
    }
}

fn parse_factor(lexer: &mut Lexer) -> MemOp {
    let r = Reg::from_str(&lexer.get_token().literal).unwrap();
    lexer.match_token(TokenType::Identifier);
    match lexer.get_token_type() {
        TokenType::CBracket => {
            // lexer.match_token(TokenType::CBracket);
            MemOp::Single(r)
        }
        TokenType::Plus => {
            lexer.match_token(TokenType::Plus);
            let TokenType::Int(offset) = lexer.get_token_type() else {
                panic!("Unsupported Operation! {}", lexer.get_token().literal);
            };
            lexer.next_token();
            MemOp::Offset(r, offset as usize)
        }
        TokenType::Minus => {
            lexer.match_token(TokenType::Minus);
            let TokenType::Int(offset) = lexer.get_token_type() else {
                panic!("Unsupported Operation! {}", lexer.get_token().literal);
            };
            lexer.next_token();
            MemOp::Negate(r, offset as usize)
        }
        TokenType::Multi => {
            lexer.match_token(TokenType::Multi);
            let TokenType::Int(offset) = lexer.get_token_type() else {
                panic!("Unsupported Operation! {}", lexer.get_token().literal);
            };
            lexer.next_token();
            MemOp::Multi(r, offset as usize)
        }
        _ => panic!("Unsupported Register Operation!"),
    }
}

fn parse_memop(lexer: &mut Lexer) -> MemOp {
    lexer.match_token(TokenType::OBracket);
    match lexer.get_token_type() {
        TokenType::Identifier => {
            let m1 = parse_factor(lexer);
            match lexer.get_token_type() {
                TokenType::Plus => {
                    lexer.match_token(TokenType::Plus);
                    let m2 = parse_factor(lexer);
                    MemOp::Add(Box::new(m1), Box::new(m2))
                }
                TokenType::Minus => {
                    lexer.match_token(TokenType::Minus);
                    let m2 = parse_factor(lexer);
                    MemOp::Sub(Box::new(m1), Box::new(m2))
                }
                TokenType::CBracket => {
                    lexer.match_token(TokenType::CBracket);
                    return m1;
                }
                _ => panic!("Unknown Memory operation!"),
            }
        }
        _ => {
            panic!(
                "Unsupported ASM memory operation {}!",
                lexer.get_token().literal
            );
        }
    }
}

#[test]
fn test_mnemonic_parsing() {
    assert_eq!(
        "mov rax, 1".to_string(),
        parse_asm("mov rax, 1".to_string())
    );
    assert_eq!("syscall".to_string(), parse_asm("syscall".to_string()));
    assert_eq!("push rax".to_string(), parse_asm("push rax".to_string()));
    assert_eq!(
        "mov rax, [rbx]".to_string(),
        parse_asm("mov rax, [rbx]".to_string())
    );
    assert_eq!(
        "mov rax, Qword [rbx + 2]".to_string(),
        parse_asm("mov rax, qword [rbx+2]".to_string())
    );
    assert_eq!(
        "mov rax, Qword [rbx + 2 - rax * 3]".to_string(),
        parse_asm("mov rax, qword [rbx+2-rax*3]".to_string())
    );
}
