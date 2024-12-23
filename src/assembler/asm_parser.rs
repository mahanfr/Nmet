use std::str::FromStr;

use crate::lexer::{Lexer, TokenType};

#[allow(unused_imports)]
use super::{
    instructions::{Instr, Opr, Oprs},
    memory::MemAddr,
    mnemonic::Mnemonic::{self, *},
    register::Reg,
};

pub fn parse_asm(source: String) -> Instr {
    let mut lexer = Lexer::new(format!("asm:{source}"), source);
    let mnmemonic = parse_mnemonic(&mut lexer);
    let mut ops = Vec::<Opr>::new();
    loop {
        if lexer.get_token().is_empty() {
            break;
        }
        ops.push(parse_op(&mut lexer));
        if lexer.get_token_type() == TokenType::Comma {
            lexer.match_token(TokenType::Comma);
            continue;
        } else {
            break;
        }
    }
    let oprs = match ops.len() {
        0 => Oprs::None,
        1 => Oprs::One(ops[0].clone()),
        2 => Oprs::Two(ops[0].clone(), ops[1].clone()),
        _ => unreachable!(),
    };
    Instr::new(mnmemonic, oprs)
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
                    parse_mem(lexer, 8)
                }
                "dword" => {
                    lexer.match_token(TokenType::Identifier);
                    parse_mem(lexer, 4)
                }
                "word" => {
                    lexer.match_token(TokenType::Identifier);
                    parse_mem(lexer, 2)
                }
                "byte" => {
                    lexer.match_token(TokenType::Identifier);
                    parse_mem(lexer, 1)
                }
                _ => panic!(
                    "[{}] Unexpected Asm Operation ({})!",
                    lexer.get_token_loc(),
                    lexer.get_token().literal
                ),
            }
        }
        TokenType::OBracket => parse_mem(lexer, 0),
        TokenType::Int(i) => i.into(),
        _ => panic!("Unsupported ASM operation!"),
    }
}

fn parse_mem(lexer: &mut Lexer, size: u8) -> Opr {
    lexer.match_token(TokenType::OBracket);
    let r = Reg::from_str(&lexer.get_token().literal).unwrap();
    lexer.match_token(TokenType::Identifier);
    let res: Opr = match lexer.get_token_type() {
        TokenType::CBracket => Opr::Mem(MemAddr::new_s(size, r)),
        TokenType::Plus | TokenType::Minus => {
            let sign = lexer.get_token_type();
            lexer.next_token();
            let TokenType::Int(mut val) = lexer.get_token_type() else {
                panic!("expected an integer found {}", lexer.get_token_type());
            };
            if sign == TokenType::Minus {
                val = -val;
            }
            lexer.next_token();
            if lexer.get_token_type() == TokenType::CBracket {
                Opr::Mem(MemAddr::new_disp_s(size, r, val))
            } else if lexer.get_token_type() == TokenType::Plus {
                lexer.match_token(TokenType::Plus);
                let r2 = Reg::from_str(&lexer.get_token().literal).unwrap();
                lexer.match_token(TokenType::Identifier);
                if lexer.get_token_type() == TokenType::CBracket {
                    Opr::Mem(MemAddr::new_sib_s(size, r, val, r2, 1))
                } else {
                    lexer.match_token(TokenType::Multi);
                    let TokenType::Int(scale) = lexer.get_token_type() else {
                        panic!("expected an integer found {}", lexer.get_token_type());
                    };
                    if ![1, 2, 4, 8].contains(&scale) {
                        panic!("unexpected scale for a register");
                    }
                    lexer.next_token();
                    Opr::Mem(MemAddr::new_sib_s(size, r, val, r2, scale as u8))
                }
            } else {
                panic!("unsupported operator \"{}\"!", lexer.get_token().literal);
            }
        }
        _ => panic!("unsupported operation!"),
    };
    lexer.match_token(TokenType::CBracket);
    res
}

#[test]
fn test_mnemonic_parsing() {
    assert_eq!(
        Instr::new2(Mov, Reg::RAX, Opr::Imm8(1)),
        parse_asm("mov rax, 1".to_string())
    );
    assert_eq!(Instr::new0(Syscall), parse_asm("syscall".to_string()));
    assert_eq!(
        Instr::new1(Push, Reg::RAX),
        parse_asm("push rax".to_string())
    );
    assert_eq!(
        Instr::new2(Mov, Reg::RAX, MemAddr::new(Reg::RBX)),
        parse_asm("mov rax, [rbx]".to_string())
    );
    assert_eq!(
        Instr::new2(Mov, Reg::RAX, MemAddr::new_disp_s(8, Reg::RBX, 2)),
        parse_asm("mov rax, qword [rbx+2]".to_string())
    );
    assert_eq!(
        Instr::new2(
            Mov,
            Reg::RAX,
            MemAddr::new_sib_s(8, Reg::RBX, 2, Reg::RAX, 4)
        ),
        parse_asm("mov rax, qword [rbx+2 + rax*4]".to_string())
    );
}
