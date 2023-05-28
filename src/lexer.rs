use std::{fs, process::exit, fmt::Display};

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum TToken {
    Identifier,
    Number,
    StringLiteral,
    DOLLAR,
    ATSIGN,
    NOTATSIGN,
    HASH,
    OPAREN,
    CPAREN,
    OCURLY,
    CCURLY,
    OBRACE,
    CBRACE,
    COMMA,
    DOT,
    COLON,
    SEMICOLON,
    // relational oprators
    EQEQ, // ==
    NOTEQ, // !=
    LESSEQ, // <=
    MOREEQ, // >=
    LESS, // <
    MORE, // >
    // assign
    EQ, // =
    PLUSEQ, // +=
    SUBEQ, // -=
    MULTYEQ, // *=
    DEVIDEEQ, // /=
    MODEQ, // %=
    ANDEQ, // &=
    OREQ, // |=
    XOREQ, // ^=
    // logical opration
    NOT, // !
    ANDAND, // &&
    OROR, // ||
    // bitwise 
    AND, // &
    OR, // |
    XOR, // ^
    // opration
    SUB,
    PLUS,
    MULTY,
    DEVIDE,
    MOD,
    // keywords
    IF,
    FUNCTION,
    IMAGE,
    VIDEO,
    SOUND,
    ELSE,
    FOR,
    ENDIF,
    ENDFOR,
    BREAK,
    CONTINUE,
    RETURN,
    TO,
    IN,
    EOF,
}

impl TToken {
    pub fn is_single_char_token(char: u8) -> Option<TToken> {
        match char {
            b'{' => {Some(TToken::OCURLY)},
            b'}' => {Some(TToken::CCURLY)},
            b'[' => {Some(TToken::OBRACE)},
            b']' => {Some(TToken::CBRACE)},
            b'(' => {Some(TToken::OPAREN)},
            b')' => {Some(TToken::CPAREN)},
            b',' => {Some(TToken::COMMA)},
            b'.' => {Some(TToken::DOT)},
            b';' => {Some(TToken::SEMICOLON)},
            b'-' => {Some(TToken::SUB)},
            b'+' => {Some(TToken::PLUS)},
            b'*' => {Some(TToken::MULTY)},
            b'/' => {Some(TToken::DEVIDE)},
            b'%' => {Some(TToken::MOD)},
            b'!' => {Some(TToken::NOT)},
            b'$' => {Some(TToken::DOLLAR)},
            b'@' => {Some(TToken::ATSIGN)},
            b':' => {Some(TToken::COLON)},
            b'=' => {Some(TToken::EQ)},
            b'<' => {Some(TToken::LESS)},
            b'>' => {Some(TToken::MORE)},
            b'&' => {Some(TToken::AND)},
            b'|' => {Some(TToken::OR)},
            _ => {None}
        }
    }

    pub fn is_double_char_token(first: u8,next: u8) -> Option<TToken> {
        let double_char = [first,next];
        match double_char.as_ref() {
            b"==" => {Some(TToken::EQEQ)},
            b"!=" => {Some(TToken::NOTEQ)},
            b"<=" => {Some(TToken::LESSEQ)},
            b">=" => {Some(TToken::MOREEQ)},

            b"+=" => {Some(TToken::PLUSEQ)},
            b"-=" => {Some(TToken::SUBEQ)},
            b"*=" => {Some(TToken::MULTYEQ)},
            b"/=" => {Some(TToken::DEVIDEEQ)},
            b"%=" => {Some(TToken::MODEQ)},
            b"&=" => {Some(TToken::ANDEQ)},
            b"|=" => {Some(TToken::OREQ)},
            b"^=" => {Some(TToken::XOREQ)},

            b"&&" => {Some(TToken::ANDAND)},
            b"||" => {Some(TToken::OROR)},
            
            b"!@" => {Some(TToken::NOTATSIGN)},
            _ => {None}
        }
    }

}

type Loc = (String,usize,usize);

#[allow(dead_code)]
#[derive(Debug)]
pub struct Lexer {
    file_path: String,
    source: Vec<u8>,
    cur: usize,
    bol: usize,
    row: usize,
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub struct Token {
    pub ttype: TToken,
    pub literal: Vec<u8>,
    pub file_path : String,
    pub col: usize,
    pub line: usize,
}
impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"Token ({:?}) \"{}\" {}:{}:{}",
            self.ttype,String::from_utf8_lossy(self.literal.as_slice()),
            self.file_path,
            self.line,
            self.col
        )
    }

}
impl Token {
    pub fn new(ttype: TToken, literal: Vec<u8>,loc : Loc) -> Self {
        Self {ttype, literal, file_path: loc.0, line: loc.1, col: loc.2}
    }
    
    pub fn loc_string(&self) -> String {
        format!("{}:{}:{}",self.file_path,self.line,self.col)
    }

    pub fn get_literal_string(&self) -> String {
        String::from_utf8(self.literal.to_vec()).unwrap()
    }
}

impl Lexer {
    pub fn new(file_path: impl ToString) -> Self {
        let buf = fs::read(file_path.to_string()).unwrap();
        Self { file_path: file_path.to_string(), source: buf , cur: 0, bol: 0, row: 0 }
    }

    pub fn from_str(source: impl ToString) -> Self {
        Self {
            file_path: "INTERNAL".to_string(), 
            source: source.to_string().as_bytes().to_vec(), 
            cur: 0, 
            bol: 0, 
            row: 0
        }
    }

    fn drop_char(&mut self) {
        if !self.is_empty() {
            let char = self.source[self.cur];
            self.cur += 1;
            if char == b'\n'{
                self.bol = self.cur;
                self.row += 1;
            }
        }
    }

    fn drop_line(&mut self) {
        while !self.is_empty() && self.source[self.cur] != b'\n' {
            self.drop_char();
        }
        if !self.is_empty() {
            self.drop_char();
        }
    }

    fn is_empty(&self) -> bool {
        self.cur >= self.source.len()
    }

    fn trim_left(&mut self) {
        while !self.is_empty() && self.source[self.cur].is_ascii_whitespace() {
            self.drop_char()
        }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        self.trim_left();
        while !self.is_empty() && self.cur + 1 < self.source.len() {
            let sub1 = self.source[self.cur];
            let sub2 = self.source[self.cur+1];
            if !(sub1 == b'/' && sub2 == b'/')  {break;}
            self.drop_line();
            self.trim_left();
        }
        let loc = (self.file_path.to_string(), self.row + 1, self.cur - self.bol + 1);
        if self.is_empty() {return None;}
        
        let first = self.source[self.cur];

        if first.is_ascii_alphabetic() || first == b'_' {
            let index = self.cur;
            while !self.is_empty() && 
                (self.source[self.cur].is_ascii_alphanumeric() || self.source[self.cur] == b'_') {
                    self.drop_char();
            }
            let literal = self.source[index..self.cur].to_vec();
            match literal.as_slice() {
                b"if" => {return Some(Token::new(TToken::IF,literal,loc));}
                b"else" => {return Some(Token::new(TToken::ELSE,literal,loc));}
                b"endif" => {return Some(Token::new(TToken::ENDIF,literal,loc));}
                b"for" => {return Some(Token::new(TToken::FOR,literal,loc));}
                b"endfor" => {return Some(Token::new(TToken::ENDFOR,literal,loc));}
                b"break" => {return Some(Token::new(TToken::BREAK,literal,loc));}
                b"continue" => {return Some(Token::new(TToken::CONTINUE,literal,loc));}
                b"return" => {return Some(Token::new(TToken::RETURN,literal,loc));}
                b"to" => {return Some(Token::new(TToken::TO,literal,loc));}
                b"in" => {return Some(Token::new(TToken::IN,literal,loc));}
                b"function" => {return Some(Token::new(TToken::FUNCTION,literal,loc));}
                b"image" => {return Some(Token::new(TToken::IMAGE,literal,loc));}
                b"sound" => {return Some(Token::new(TToken::SOUND,literal,loc));}
                b"video" => {return Some(Token::new(TToken::VIDEO,literal,loc));}
                _ => {
                    return Some(Token::new(TToken::Identifier,literal,loc));
                }
            }
        }

        if first.is_ascii_digit() {
            let index = self.cur;
            while !self.is_empty() && self.source[self.cur].is_ascii_digit() {
                self.drop_char();
            }
            let literal = self.source[index..self.cur].to_vec();
            return Some(Token::new(TToken::Number,literal,loc));
        }
        
        if first == b'"' {
            self.drop_char();
            let mut literal = Vec::<u8>::new();
            while !self.is_empty() {
                let char = self.source[self.cur];
                if char == b'"' {break;}
                if char == b'\n' {
                    println!("string literal not closed before end of line :{}:{}:{}",loc.0,loc.1,loc.2);
                    exit(1);
                }
                if char == b'\\' {
                    self.drop_char();
                    if self.is_empty() {
                        println!("string literal unfinished escape sequence :{}:{}:{}",loc.0,loc.1,loc.2);
                        exit(1);
                    }

                    let escape = self.source[self.cur];
                    match escape {
                        b'n' => {
                            literal.push(b'\n');
                            self.drop_char();
                        },
                        b'"' => {
                            literal.push(b'"');
                            self.drop_char();
                        },
                        b't' => {
                            literal.push(b'\t');
                            self.drop_char();
                        },
                        b'r' => {
                            literal.push(b'\r');
                            self.drop_char();
                        },
                        b'\\' => {
                            literal.push(b'\\');
                            self.drop_char();
                        },
                        _ => {
                            println!("unsupported escape sequence (\\{}) :{}:{}:{}",escape,loc.0,loc.1,loc.2);
                            exit(1);
                        }
                    }
                }
                literal.push(char);
                self.drop_char();
            }
            if !self.is_empty() {
                self.drop_char();
                return Some(Token::new(TToken::StringLiteral,literal,loc));
            }
        }

        match TToken::is_single_char_token(first) {
            Some(tt) => {
                self.drop_char();
                if !self.is_empty(){
                    let next = self.source[self.cur];
                    if TToken::is_single_char_token(next).is_some() {
                        match TToken::is_double_char_token(first, next) {
                            Some(dtt) => {
                                self.drop_char();
                                return Some(Token::new(dtt,vec![first,next],loc));
                            },
                            None => ()
                        }
                    }
                }
                return Some(Token::new(tt,[first].to_vec(),loc));
            },
            None => ()
        }

        println!("string literal not closed before EOF :{}:{}:{}",loc.0,loc.1,loc.2);
        exit(1);
    }
}

pub fn expect_non_empty_token(lexer: &mut Lexer) -> Token{
    let token_option = lexer.next_token();
    if let Some(token) = token_option {
        return token;
    } else {
        println!("Expected Token found EOF {}:{}:{}",
                 lexer.file_path,lexer.row + 1,lexer.cur - lexer.bol + 1);
        exit(1);
    }
}

pub fn expect_any_token(lexer: &mut Lexer, callback: fn()) -> Token {
    let token_option = lexer.next_token();
    if let Some(token) = token_option {
        return token;
    } else {
        callback();
        return Token::new(TToken::EOF, vec![], (lexer.file_path.to_string(), lexer.row + 1, lexer.cur - lexer.bol + 1));
    }
}

pub fn expect_token(lexer: &mut Lexer, types:Vec<TToken>) -> Token {
    let token_option = lexer.next_token();
    
    if let Some(token) = token_option {
        if types.contains(&token.ttype) {
            return token;
        }else {
            println!("expect one of {:?} found {:?} {}:{}:{}",types,
                     token.ttype,token.file_path,token.line,token.col);
            exit(1);
        }
    } else {
        println!("expect one of {:?} found EOF {}:{}:{}",types,
                 lexer.file_path,lexer.row + 1,lexer.cur - lexer.bol + 1);
        exit(1);
    }
}
