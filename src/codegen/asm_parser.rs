use std::str::FromStr;

#[allow(unused_imports)]
use crate::{lexer::{Lexer, TokenType}, mem ,memq};

use super::{
    instructions::{Instr, Opr},
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
                    parse_mem(lexer, 8)
                },
                "dword" => {
                    lexer.match_token(TokenType::Identifier);
                    parse_mem(lexer, 4)
                },
                "word" => {
                    lexer.match_token(TokenType::Identifier);
                    parse_mem(lexer, 2)
                },
                "byte" => {
                    lexer.match_token(TokenType::Identifier);
                    parse_mem(lexer, 1)
                }
                _ => panic!("Unexpected Asm Operation {}!", lexer.get_token().literal),
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
        TokenType::CBracket => Opr::MemAddr(size, r),
        TokenType::Plus | TokenType::Minus => {
            let sign = lexer.get_token_type();
            lexer.next_token();
            let TokenType::Int(mut val) = lexer.get_token_type() else {
                panic!("expected an integer found {}",lexer.get_token_type());
            };
            if sign == TokenType::Minus {
                val = val * -1;
            }
            lexer.next_token();
            if lexer.get_token_type() == TokenType::CBracket {
                Opr::MemDisp(size, r, val)
            } else if lexer.get_token_type() == TokenType::Plus {
                lexer.match_token(TokenType::Plus);
                let r2 = Reg::from_str(&lexer.get_token().literal).unwrap();
                lexer.match_token(TokenType::Identifier);
                if lexer.get_token_type() == TokenType::CBracket {
                    Opr::MemDispSib(size, r, val, r2, 1)
                } else {
                    lexer.match_token(TokenType::Multi);
                    let TokenType::Int(scale) = lexer.get_token_type() else {
                        panic!("expected an integer found {}",lexer.get_token_type());
                    };
                    if !vec![1,2,4,8].contains(&scale) {
                        panic!("unexpected scale for a register");
                    }
                    lexer.next_token();
                    Opr::MemDispSib(size, r, val, r2, scale as u8)
                }
            } else {
                panic!("unsupported operator \"{}\"!",lexer.get_token().literal);
            }
        },
        _ => panic!("unsupported operation!")
    };
    lexer.match_token(TokenType::CBracket);
    res
}

// fn parse_factor(lexer: &mut Lexer) -> Opr {
//     let r = Reg::from_str(&lexer.get_token().literal).unwrap();
//     lexer.match_token(TokenType::Identifier);
//     match lexer.get_token_type() {
//         TokenType::CBracket => {
//             // lexer.match_token(TokenType::CBracket);
//             Opr::MemAddr(r)
//         }
//         TokenType::Plus => {
//             lexer.match_token(TokenType::Plus);
//             let TokenType::Int(offset) = lexer.get_token_type() else {
//                 panic!("Unsupported Operation! {}", lexer.get_token().literal);
//             };
//             lexer.next_token();
//             Opr::MemDisp(r, offset)
//         }
//         TokenType::Minus => {
//             lexer.match_token(TokenType::Minus);
//             let TokenType::Int(offset) = lexer.get_token_type() else {
//                 panic!("Unsupported Operation! {}", lexer.get_token().literal);
//             };
//             lexer.next_token();
//             Opr::MemDisp(r, -offset)
//         }
//         TokenType::Multi => {
//             lexer.match_token(TokenType::Multi);
//             let TokenType::Int(offset) = lexer.get_token_type() else {
//                 panic!("Unsupported Operation! {}", lexer.get_token().literal);
//             };
//             lexer.next_token();
//             
//             MemOp::Multi(r, offset as usize)
//         }
//         _ => panic!("Unsupported Register Operation!"),
//     }
// }
// 
// fn parse_memop(lexer: &mut Lexer) -> MemOp {
//     lexer.match_token(TokenType::OBracket);
//     match lexer.get_token_type() {
//         TokenType::Identifier => {
//             let m1 = parse_factor(lexer);
//             match lexer.get_token_type() {
//                 TokenType::Plus => {
//                     lexer.match_token(TokenType::Plus);
//                     let m2 = parse_factor(lexer);
//                     MemOp::Add(Box::new(m1), Box::new(m2))
//                 }
//                 TokenType::Minus => {
//                     lexer.match_token(TokenType::Minus);
//                     let m2 = parse_factor(lexer);
//                     MemOp::Sub(Box::new(m1), Box::new(m2))
//                 }
//                 TokenType::CBracket => {
//                     lexer.match_token(TokenType::CBracket);
//                     m1
//                 }
//                 _ => panic!("Unknown Memory operation!"),
//             }
//         }
//         _ => {
//             panic!(
//                 "Unsupported ASM memory operation {}!",
//                 lexer.get_token().literal
//             );
//         }
//     }
// }

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
        Instr::new_instr2(Mnemonic::Mov, Reg::RAX, mem!(Reg::RBX)),
        parse_asm("mov rax, [rbx]".to_string())
    );
    assert_eq!(
        Instr::new_instr2(Mnemonic::Mov, Reg::RAX, memq!(Reg::RBX, 2)),
        parse_asm("mov rax, qword [rbx+2]".to_string())
    );
    assert_eq!(
        Instr::new_instr2(
            Mnemonic::Mov,
            Reg::RAX,
            memq!(Reg::RBX, 2, Reg::RAX, 4)
        ),
        parse_asm("mov rax, qword [rbx+2 + rax*4]".to_string())
    );
}
