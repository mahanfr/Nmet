use crate::lexer::{Lexer, TToken,expect_token};
use crate::parser::{program::Node,expr::{ExprPath,get_expr},definition::{Arg,Type}};

#[derive(Debug)]
pub struct Func {
    pub ident: String,
    pub args: Vec<Arg>,
    pub return_type: Type,
    pub block: Vec<Node>,
}
impl Func {
    pub fn new(lexer: &mut Lexer) -> Self {
        let token = expect_token(lexer, vec![TToken::Identifier]);
        let ident = String::from_utf8(token.literal).unwrap();
        expect_token(lexer, vec![TToken::OPAREN]);
        let mut args = Vec::<Arg>::new();
        loop{
            let token = expect_token(lexer, vec![TToken::Identifier,TToken::CPAREN,TToken::COMMA]);
            if token.ttype == TToken::Identifier {
                let type_token = expect_token(lexer, vec![TToken::Identifier]);
                args.push(Arg {
                    ident: String::from_utf8(token.literal).unwrap(), 
                    kind: Type {name: String::from_utf8(type_token.literal).unwrap()}});
            }
            else if token.ttype == TToken::CPAREN {break;}
        }
        let token = expect_token(lexer, vec![TToken::Identifier]);
        let return_type = Type { name: String::from_utf8(token.literal).unwrap() };
        expect_token(lexer, vec![TToken::OCURLY]);
        loop {
            let mut token = lexer.next_token();
            if token.ttype == TToken::CCURLY {
                break;
            }
            loop{
                if token.ttype == TToken::SEMICOLON {
                    break;
                }

                if token.ttype == TToken::Identifier {
                    let _ident_token = token;
                    token = lexer.next_token();
                    if token.ttype == TToken::EQ {
                        let _left = ExprPath {};
                        let _right = get_expr(lexer);
                        todo!("ExprAssign  left, right ");
                    }
                }else {
                    unimplemented!();
                }
            }

        }
        return Self { ident , args, return_type, block: Vec::new() } ;
    }
    pub fn get_node(lexer: &mut Lexer) -> Node {
        Node::Func { var: Self::new(lexer) }
    }

}

