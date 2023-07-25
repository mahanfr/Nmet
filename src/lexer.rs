use std::process::exit;
type Loc = (String,usize,usize);

#[derive(Debug,Clone,Copy,PartialEq)]
pub enum TokenType {
    Identifier,
    Int(i32),
    Float(f32),
    Char(char),
    String,
    
    Define, // :=

    Plus, // +
    Minus, // -
    Multi, // *
    Devide, // /

    Fun, // fun
    If, // if
    Else, // else

    Eq, // =
    DoubleEq, // ==
    ColonEq, // :=

    Log, // #
    SemiColon, // ;
    Colon, // :
    DoubleColon, // ::
    Comma, // ,

    OParen,
    CParen,
    OBracket,
    CBracket,
    OCurly,
    CCurly,
}

#[derive(Debug,Clone)]
pub struct Token {
    pub file_path: String,
    pub col: usize,
    pub line: usize,
    pub literal: String,
    pub t_type: TokenType,
}

impl Token {
    pub fn new(t_type: TokenType,literal: String, loc: Loc) -> Self {
        Self{
            literal,
            t_type,
            file_path: loc.0,
            line: loc.1,
            col: loc.2,
        }
    }

    pub fn get_loc_string(&self) -> String {
        format!("{}:{}:{}",self.file_path,self.line,self.col)
    }
}

#[derive(Debug,Clone)]
pub struct Lexer {
    pub file_path: String,
    source: Vec<char>,
    pub token: Option<Token>,
    cur: usize,
    bol: usize,
    row: usize,
}

impl Lexer {
    pub fn new(file_path: String,source: String) -> Self {
        Self {
            file_path,
            source: source.chars().collect::<Vec<char>>(),
            token: None,
            cur: 0,
            bol: 0,
            row: 0,
        }
    }

    fn is_empty(&self) -> bool {
        self.cur >= self.source.len()
    }

    fn drop_line(&mut self) {
        while !self.is_empty() {
            if self.source[self.cur] == '\n'{
                self.drop();
                break;
            } else {
                self.drop();
            }
        }
    }

    fn drop(&mut self) {
        if !self.is_empty() {
            let char = self.source[self.cur];
            self.cur += 1;
            if char == '\n' {
                self.bol = self.cur;
                self.row += 1;
            }
        }
    }

    fn trim_left(&mut self) {
        while !self.is_empty() && self.source[self.cur].is_whitespace() {
            self.drop();
        } 
    }

    fn get_loc(&self) -> Loc {
        (self.file_path.clone(), self.row + 1, self.cur - self.bol + 1)
    }

    pub fn get_loc_string(&self) -> String {
        let loc:Loc = (self.file_path.clone(), self.row + 1, self.cur - self.bol + 1);
        format!("{}:{}:{}",loc.0,loc.1,loc.2)
    }

    pub fn peek(&mut self) -> Option<Token> {
        let cur = self.cur;
        let bol = self.bol;
        let row = self.row;
        let token = self._next_token();
        self.cur = cur;
        self.bol = bol;
        self.row = row;
        return token;
    }

    pub fn get_token_type(&self) -> TokenType{
        let Some(tk) = self.token.clone() else {
            eprintln!("Expected a Token, found Eof at {}",self.get_loc_string());
            exit(-1);
        };
        return tk.t_type.clone();
    }

    pub fn match_token(&mut self, t_type: TokenType) {
        let Some(tk) = self.token.clone() else {
            eprintln!("Expected {:?}, found Eof at {}",t_type,self.get_loc_string());
            exit(-1);
        };
        if tk.t_type == t_type {
            self.next_token();
        } else {
            eprintln!("Expected {:?}, found {:?} at {}",t_type,tk.t_type,self.get_loc_string());
            exit(-1);
        }
    }

    pub fn get_token(&self) -> Option<Token> {
        return self.token.clone();
    }

    pub fn next_token(&mut self) -> Option<Token> {
        let token = self._next_token();
        self.token = token.clone();
        return token;
    }

    fn _next_token(&mut self) -> Option<Token> {
        self.trim_left();
        while !self.is_empty() {
            if self.source[self.cur] == '~' {
                self.drop_line();
                self.trim_left();
            } else {
                break;
            }
        }
        if self.is_empty() {
            return None
        }
        let first = self.source[self.cur];
        if first.is_ascii_alphabetic() || first == '_' {
            let index = self.cur;
            while !self.is_empty() && 
                (self.source[self.cur].is_ascii_alphanumeric() || self.source[self.cur] == '_') {
                self.drop();
            }
            let literal = String::from_iter(self.source[index..self.cur].to_vec());
            match Self::is_keyword(&literal) {
                Some(keyword_token) => return Some(Token::new(keyword_token,literal,self.get_loc())),
                None => return Some(Token::new(TokenType::Identifier,literal,self.get_loc()))
            }
        }
        if first.is_ascii_digit() {
            let index = self.cur;
            self.drop();
            while !self.is_empty() && 
                (self.source[self.cur].is_ascii_alphanumeric() || self.source[self.cur] == '.' ) {
                self.drop();
            }
            let literal = String::from_iter(self.source[index..self.cur].to_vec());
            let ttype_and_val = Self::parse_numeric_literal(&literal);
            return Some(Token::new(ttype_and_val,literal,self.get_loc()));
        }
        if first == '\'' {
            self.drop();
            let mut literal = String::new();
            let char = self.source[self.cur];
            if char == '\'' {
                println!("char literal can not be empty {}",self.get_loc_string());
                exit(1);
            }
            if char == '\\' {
                self.drop();
                if self.is_empty() {
                    println!("char literal unfinished escape sequence {}",self.get_loc_string());
                    exit(1);
                }
                let escape = self.source[self.cur];
                match escape {
                    'n' => {
                        literal.push('\n');
                        self.drop();
                    },
                    '\'' => {
                        literal.push('\'');
                        self.drop();
                    },
                    't' => {
                        literal.push('\t');
                        self.drop();
                    },
                    'r' => {
                        literal.push('\r');
                        self.drop();
                    },
                    '\\' => {
                        literal.push('\\');
                        self.drop();
                    },
                    _ => {
                        println!("unsupported escape sequence (\\{}) {}",escape,self.get_loc_string());
                        exit(1);
                    }
                }
            }else{
                literal.push(char);
                self.drop();
            }

            if !self.is_empty() {
                if self.source[self.cur] != '\'' {
                    println!("unsupported char {}",self.get_loc_string());
                    exit(1);
                }
                self.drop();
                return Some(Token::new(TokenType::Char(first),literal,self.get_loc()));
            }
        }

        if first == '"' {
            self.drop();
            let mut literal = String::new();
            while !self.is_empty() {
                let char = self.source[self.cur];
                if char == '"' {break;}
                if char == '\n' {
                    println!("string literal not closed before end of line {}",self.get_loc_string());
                    exit(1);
                }
                if char == '\\' {
                    self.drop();
                    if self.is_empty() {
                        println!("string literal unfinished escape sequence {}",self.get_loc_string());
                        exit(1);
                    }

                    let escape = self.source[self.cur];
                    match escape {
                        'n' => {
                            literal.push('\n');
                            self.drop();
                        },
                        '"' => {
                            literal.push('"');
                            self.drop();
                        },
                        't' => {
                            literal.push('\t');
                            self.drop();
                        },
                        'r' => {
                            literal.push('\r');
                            self.drop();
                        },
                        '\\' => {
                            literal.push('\\');
                            self.drop();
                        },
                        _ => {
                            println!("unsupported escape sequence (\\{}) {}",escape,self.get_loc_string());
                            exit(1);
                        }
                    }
                }
                literal.push(char);
                self.drop();
            }
            if !self.is_empty() {
                self.drop();
                return Some(Token::new(TokenType::String,literal,self.get_loc()));
            }
        }

        // TODO: Seperate single and double tokens
        match Self::is_single_char_token(first) {
            Some(tt) => {
                self.drop();
                if !self.is_empty(){
                    let next = self.source[self.cur];
                    if Self::is_single_char_token(next).is_some() {
                        match Self::is_double_char_token(first, next) {
                            Some(dtt) => {
                                self.drop();
                                return Some(Token::new(
                                        dtt,
                                        String::from_iter(vec![first,next]),
                                        self.get_loc())
                                    );
                            },
                            None => ()
                        }
                    }
                }
                return Some(Token::new(tt,first.to_string(),self.get_loc()));
            },
            None => ()
        }

        eprintln!("Unexpected Character at {}",self.get_loc_string());
        exit(1);
    }

    pub fn expect_token(&mut self,tokens: Vec<TokenType>) -> Token {
        let Some(token) = self.next_token() else {
            println!("Error: Unexpected EOF at {}",self.get_loc_string());
            exit(-1);
        };
        if tokens.contains(&token.t_type) {
            return token
        }else {
            println!("Error: Unexpected token ({:?}) at {}",token.t_type,self.get_loc_string());
            exit(-1);
        }
    }

    pub fn expect_some_token(&mut self) -> Token {
        let Some(token) = self.next_token() else {
            println!("Error: Unexpected EOF at {}",self.get_loc_string());
            exit(-1);
        };
        token
    }

    fn is_keyword(literal: &String) -> Option<TokenType>{
        match literal.as_str() {
            "if" => {return Some(TokenType::If)},
            "else" => {return Some(TokenType::Else)},
            "fun" => {return Some(TokenType::Fun)},
            _ => None
        }
    }

    fn is_single_char_token(char: char) -> Option<TokenType> {
        match char {
            '{' => {Some(TokenType::OCurly)},
            '}' => {Some(TokenType::CCurly)},
            '[' => {Some(TokenType::OBracket)},
            ']' => {Some(TokenType::CBracket)},
            '(' => {Some(TokenType::OParen)},
            ')' => {Some(TokenType::CParen)},
            '-' => {Some(TokenType::Minus)},
            '+' => {Some(TokenType::Plus)},
            '*' => {Some(TokenType::Multi)},
            '/' => {Some(TokenType::Devide)},
            '#' => {Some(TokenType::Log)},
            ':' => {Some(TokenType::Colon)},
            ';' => {Some(TokenType::SemiColon)},
            ',' => {Some(TokenType::Comma)},
            '=' => {Some(TokenType::Eq)},
            _ => {None}
        }
    }

    fn is_double_char_token(first: char,next: char) -> Option<TokenType> {
        let mut double_char = String::new();
        double_char.push(first);
        double_char.push(next);
        match double_char.as_str() {
            "==" => {Some(TokenType::DoubleEq)},
            ":=" => {Some(TokenType::ColonEq)},
            "::" => {Some(TokenType::DoubleColon)}
            _ => {None}
        }
    }

    fn parse_numeric_literal(literal: &String) -> TokenType {
        // 0x001 0xff 0b0010
        let mut lit_chars = literal.chars();
        if literal.contains('x') {
            Self::expect_char(&lit_chars.next(),vec!['0']);
            Self::expect_char(&lit_chars.next(),vec!['x']);
            let mut value: i32 = 0;
            loop {
                let Some(ch) = lit_chars.next() else {break;};
                let Some(digit) = ch.to_digit(16) else{
                    println!("Error: Unknown character in parsing: {}",literal);
                    exit(-1);
                };
                value = (value * 16i32) + digit as i32;
            }
            return TokenType::Int(value);
        } else if literal.contains('b') {
            Self::expect_char(&lit_chars.next(),vec!['0']);
            Self::expect_char(&lit_chars.next(),vec!['b']);
            let mut value: i32 = 0;
            loop {
                let Some(ch) = lit_chars.next() else {break;};
                let Some(digit) = ch.to_digit(2) else{
                    println!("Error: Unknown character in parsing: {}",literal);
                    exit(-1);
                };
                value = (value * 2i32) + digit as i32;
            }
            return TokenType::Int(value);
        } else if literal.contains('.') {
            let value : f32 = literal.parse::<f32>().unwrap();
            return TokenType::Float(value);
        } else {
            let value : i32 = literal.parse::<i32>().unwrap();
            return TokenType::Int(value);
        }
    }

    fn expect_char(copt: &Option<char>, chars: Vec<char>) -> char {
        let Some(char) = copt else {
            println!("Error: Undifined character set for numbers");
            exit(-1);
        };
        if chars.contains(&char) {
            return char.clone();
        }
        return char.clone();
    }
}
