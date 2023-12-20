use std::str::FromStr;

use crate::lexer::{Lexer, TokenType};

use super::{
    instructions::{Instr, Opr},
    memory::{Mem, MemOp},
    mnmemonic::Mnemonic,
    register::Reg,
};

pub fn parse_asm(source: String) -> Instr {
    let mut lexer = Lexer::new(format!("asm:{source}"), source);
    let mnmemonic = parse_mnemonic(&mut lexer);
    let mut ops = Vec::<Opr>::new();
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
        Instr::new_instr2(mnmemonic, ops[0].clone(), ops[1].clone())
        //format!("{mnmemonic} {}, {}", ops[0], ops[1])
    } else if airty == 1 {
        Instr::new_instr1(mnmemonic, ops[0].clone())
        //format!("{mnmemonic} {}", ops[0])
    } else {
        Instr::new_instr0(mnmemonic)
        // format!("{mnmemonic}")
    }
}

fn parse_mnemonic(lexer: &mut Lexer) -> Mnemonic {
    let ident = lexer.next_token();
    lexer.match_token(TokenType::Identifier);
    Mnemonic::from_str(&ident.literal).unwrap()
}

fn parse_op(lexer: &mut Lexer) -> Opr {
    match lexer.get_token_type() {
        TokenType::Identifier => {
            if let Ok(reg) = Reg::from_str(&lexer.get_token().literal) {
                lexer.match_token(TokenType::Identifier);
                return reg.into();
            }
            match lexer.get_token().literal.to_lowercase().as_str() {
                "qword" => {
                    lexer.match_token(TokenType::Identifier);
                    Mem::Qword(parse_memop(lexer)).into()
                }
                "dword" => {
                    lexer.match_token(TokenType::Identifier);
                    Mem::Dword(parse_memop(lexer)).into()
                }
                "word" => {
                    lexer.match_token(TokenType::Identifier);
                    Mem::Word(parse_memop(lexer)).into()
                }
                "byte" => {
                    lexer.match_token(TokenType::Identifier);
                    Mem::Byte(parse_memop(lexer)).into()
                }
                _ => panic!("Unexpected Asm Operation {}!", lexer.get_token().literal),
            }
        }
        TokenType::OBracket => Mem::U(parse_memop(lexer)).into(),
        TokenType::Int(i) => i.into(),
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
                    m1
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
        Instr::new_instr2(Mnemonic::Mov, Reg::RAX, 1),
        parse_asm("mov rax, 1".to_string())
    );
    assert_eq!(
        Instr::new_instr0(Mnemonic::Syscall),
        parse_asm("syscall".to_string())
    );
    assert_eq!(
        Instr::new_instr1(Mnemonic::Push, Reg::RAX),
        parse_asm("push rax".to_string())
    );
    assert_eq!(
        Instr::new_instr2(Mnemonic::Mov, Reg::RAX, Mem::U(Reg::RBX.into())),
        parse_asm("mov rax, [rbx]".to_string())
    );
    assert_eq!(
        Instr::new_instr2(Mnemonic::Mov, Reg::RAX, Mem::Qword(Reg::RBX + 2)),
        parse_asm("mov rax, qword [rbx+2]".to_string())
    );
    assert_eq!(
        Instr::new_instr2(
            Mnemonic::Mov,
            Reg::RAX,
            Mem::Qword(Reg::RBX + 2 - Reg::RAX * 3)
        ),
        parse_asm("mov rax, qword [rbx+2-rax*3]".to_string())
    );
}
