/*
 * // Code For Image generation
 * @[image]
 * <path = ./> // Default
 * <format = png> // Default
 * <size = 800*600> // Default
 * @.fill = 255 255 255 0;
 * @[0,0] = 255 0 0 255;
 * @[10..20,10..50] = 0 255 0 255;
 * @.fin
 * */

mod lexer;

use std::process::exit;

use lexer::{expect_token, TToken, Token, expect_non_empty_token};

use crate::lexer::Lexer;

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
enum MediaFormat{
    PNG,
    PPM,
}
impl MediaFormat {
    pub fn get_format(token: &Token) -> Self {
        let ident = String::from_utf8(token.literal.clone()).unwrap();
        match ident.as_str() {
            "png" => {return Self::PNG;}
            "ppm" => {return Self::PPM;}
            _ => {
                println!("Unexpected format {ident} at {}",&token.loc_string());
                exit(1);
            }
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
enum Expr{
    Assign,
    NotImplemented,
}
impl Expr {
    pub fn get_expr(lexer: &mut Lexer,_tk: Token) -> Expr { 
        loop {
            let token = expect_non_empty_token(lexer);
            if token.ttype == TToken::SEMICOLON {break;}
        }
        return Expr::NotImplemented;
    }
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
enum Section{
    ImageBlock{
        export_path: String,
        format: MediaFormat,
        size: (usize,usize),
        block: Vec<Expr>,
    },
    Fun{
        ident: String,
    }
}
impl Section {
    pub fn parse_image_block(lexer: &mut Lexer) -> Self {
        let mut export_path = "./".to_string();
        let mut format = MediaFormat::PPM;
        let mut size = (800 , 600);
        let mut block = Vec::<Expr>::new();
        expect_token(lexer, vec![TToken::CBRACE]);
        let mut token: Token;
        loop {
            token = expect_non_empty_token(lexer);
            if token.ttype == TToken::LESS {
                token = expect_token(lexer, vec![TToken::Identifier]);
                if token.literal == b"path" {
                    expect_token(lexer, vec![TToken::EQ]);
                    token = expect_token(lexer, vec![TToken::StringLiteral]);
                    export_path = token.get_literal_string();
                    expect_token(lexer, vec![TToken::MORE]);
                }
                else if token.literal == b"format" {
                    expect_token(lexer, vec![TToken::EQ]);
                    token = expect_token(lexer, vec![TToken::Identifier]);
                    format = MediaFormat::get_format(&token);
                    expect_token(lexer, vec![TToken::MORE]);
                }
                else if token.literal == b"size" {
                    expect_token(lexer, vec![TToken::EQ]);
                    token = expect_token(lexer, vec![TToken::Number]);
                    size.0 = token.get_literal_string().parse::<usize>().unwrap();
                    expect_token(lexer, vec![TToken::MULTY]);
                    token = expect_token(lexer, vec![TToken::Number]);
                    size.1 = token.get_literal_string().parse::<usize>().unwrap();
                    expect_token(lexer, vec![TToken::MORE]);
                }
            }else if token.ttype == TToken::NOTATSIGN {
                expect_token(lexer, vec![TToken::OBRACE]);
                expect_token(lexer, vec![TToken::IMAGE]);
                expect_token(lexer, vec![TToken::CBRACE]);
                break; 
            }else {
                block.push(Expr::get_expr(lexer,token));
            }
        }

        Section::ImageBlock { export_path, format, size, block }
    }
}

#[derive(Debug, PartialEq)]
struct Parser{
    block: Vec<Section>
}
impl Parser{
    pub fn new() -> Self {
        Self { block: Vec::new() }
    }
    pub fn parse(&mut self,lexer: &mut Lexer) {
        expect_token(lexer, vec![TToken::ATSIGN]);
        expect_token(lexer, vec![TToken::OBRACE]);
        let token = expect_token(lexer, vec![TToken::IMAGE]);
        if token.ttype == TToken::IMAGE {
            let sec = Section::parse_image_block(lexer);
            println!("{:?}",sec);
            self.block.push(sec);
        }else {
            todo!("Not Implemented");
        }
    }
}

fn main() {
    let mut lexer = Lexer::new("test.nmt");
    let mut parser = Parser::new();
    parser.parse(&mut lexer);
    // loop {
    //     if let Some(token) = lexer.next_token() {
    //         println!("{:?}",token);
    //     } else { break; }
    // }
}

