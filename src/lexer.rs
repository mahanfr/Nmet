/**********************************************************************************************
*
*   lexing: Lexical Analysis of the program files
*
*   TokenType - Has the type of every token supported py programming language
*   Token - Turns Source code into An Iteration of tokens
*
*   LICENSE: MIT
*
*   Copyright (c) 2023-2024 Mahan Farzaneh (@mahanfr)
*
*   This software is provided "as-is", without any express or implied warranty. In no event
*   will the authors be held liable for any damages arising from the use of this software.
*
*   Permission is granted to anyone to use this software for any purpose, including commercial
*   applications, and to alter it and redistribute it freely, subject to the following restrictions:
*
*     1. The origin of this software must not be misrepresented; you must not claim that you
*     wrote the original software. If you use this software in a product, an acknowledgment
*     in the product documentation would be appreciated but is not required.
*
*     2. Altered source versions must be plainly marked as such, and must not be misrepresented
*     as being the original software.
*
*     3. This notice may not be removed or altered from any source distribution.
*
**********************************************************************************************/
use std::fmt::Display;

use crate::error_handeling::{error, Loc};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenType {
    /// Identifies a variable or functuin e.g: a, main, print
    Identifier,
    /// Numeric value e.g: 12 ,0xf3, 0b110
    Int(i32),
    /// Floating value e.g: 0.5
    Float(f64),
    /// Character Literal e.g: 'A', '9', '\n'
    Char(char),
    /// String Literal e.g: "Hello world", "hi\nhello"
    String,
    /// Inline Asm
    Asm,
    /// "+" Plus And Pos
    Plus,
    /// "-" Sub and Neg
    Minus,
    /// "*" Multiply
    Multi,
    /// "/" Devide
    Devide,
    /// "%" Modulo
    Mod,
    /// "!" Binary Not Operation
    Not,
    /// ">" Bigger Compare Oparation
    Bigger,
    /// "<" Smaller Compare Operation
    Smaller,
    /// Keyword ffi
    Ffi,
    /// Keyword Struct
    Struct,
    /// Keyword func
    Func,
    /// Keyword if
    If,
    /// Keyword var
    Var,
    /// Keyword else
    Else,
    /// Keyword return
    Return,
    /// Keyword while
    While,
    /// Keyword break
    Break,
    /// Keyword continue
    Continue,
    /// Keyword print
    Print,
    /// Keyword true
    True,
    /// Keyword false
    False,
    /// "@" Type defenition indicator
    ATSign,
    /// "=" Assgin a value to a variable
    Eq,
    /// ":=" Const indication for variable defenition
    ColonEq,
    /// "==" Compare Operation Eq
    DoubleEq,
    /// "!=" Compare Operation Not Eq
    NotEq,
    /// ">=" Compare Operation Bigger Eq
    BiggerEq,
    /// "<=" Compare Operatiom Smaller Eq
    SmallerEq,
    /// "+=" Assgin and add to itself
    PlusEq,
    /// "-=" Assgin and sub to itself
    SubEq,
    /// "*=" Assgin and multiply to itself
    MultiEq,
    /// "/=" Assgin and Devide to itself
    DivEq,
    /// "%=" Assgin and mod to itself
    ModEq,
    /// "<<" Shift Left
    Lsh,
    /// ">>" Shift Right
    Rsh,
    /// "&&" Logical And
    DoubleAnd,
    /// "||" Logical or
    DoubleOr,
    /// "&" Bitwise And
    And,
    /// "|" Bitwise or
    Or,
    /// "#" NOT DEFINED YET
    Hash,
    /// "?" Question Mark
    QMark,
    /// ";" End of stmt
    SemiColon,
    /// ":" NOT DEFINED YET
    Colon,
    /// "::" Const variable definition
    DoubleColon,
    /// "," Seperating Arguments
    Comma,
    /// "$" Dollar Sign
    Dollar,
    /// "("
    OParen,
    /// ")"
    CParen,
    /// "["
    OBracket,
    /// "]"
    CBracket,
    /// "{"
    OCurly,
    /// "}"
    CCurly,
    /// "." Refrence
    Dot,
    /// END OF FILE
    Eof,
    /// START OF FILE
    Sof,
    // Import module
    Import,
    // Macro Definition
    Macro,
    // For Loop
    For,
    // to (range)
    To,
    // Defer
    Defer,
    // Static
    Static,
    // Extern
    Extern,
}

impl TokenType {
    pub fn is_assgin_token(&self) -> bool {
        matches!(
            self,
            Self::Eq | Self::PlusEq | Self::SubEq | Self::MultiEq | Self::DivEq | Self::ModEq
        )
    }
}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenType::Identifier => write!(f, "Identifier"),
            TokenType::Int(i) => write!(f, "{}", i),
            TokenType::Float(fl) => write!(f, "{}", fl),
            TokenType::Char(char) => write!(f, "{}", char),
            TokenType::String => write!(f, "String Literal"),
            TokenType::Asm => write!(f, "Inline Asm"),
            TokenType::Plus => write!(f, "+"),
            TokenType::Minus => write!(f, "-"),
            TokenType::Multi => write!(f, "*"),
            TokenType::Devide => write!(f, "/"),
            TokenType::Mod => write!(f, "%"),
            TokenType::Not => write!(f, "!"),
            TokenType::Bigger => write!(f, ">"),
            TokenType::Smaller => write!(f, "<"),
            TokenType::Func => write!(f, "func"),
            TokenType::Ffi => write!(f, "ffi"),
            TokenType::Struct => write!(f, "struct"),
            TokenType::If => write!(f, "if"),
            TokenType::Var => write!(f, "var"),
            TokenType::Else => write!(f, "else"),
            TokenType::Return => write!(f, "return"),
            TokenType::While => write!(f, "while"),
            TokenType::Break => write!(f, "break"),
            TokenType::Continue => write!(f, "continue"),
            TokenType::Print => write!(f, "print"),
            TokenType::True => write!(f, "true"),
            TokenType::False => write!(f, "false"),
            TokenType::ATSign => write!(f, "@"),
            TokenType::Eq => write!(f, "="),
            TokenType::ColonEq => write!(f, ":="),
            TokenType::DoubleEq => write!(f, "=="),
            TokenType::NotEq => write!(f, "!="),
            TokenType::BiggerEq => write!(f, ">="),
            TokenType::SmallerEq => write!(f, "<="),
            TokenType::PlusEq => write!(f, "+="),
            TokenType::SubEq => write!(f, "-="),
            TokenType::MultiEq => write!(f, "*="),
            TokenType::DivEq => write!(f, "/="),
            TokenType::ModEq => write!(f, "%="),
            TokenType::Lsh => write!(f, "<<"),
            TokenType::Rsh => write!(f, ">>"),
            TokenType::And => write!(f, "&"),
            TokenType::Or => write!(f, "|"),
            TokenType::Hash => write!(f, "#"),
            TokenType::QMark => write!(f, "?"),
            TokenType::SemiColon => write!(f, ";"),
            TokenType::Colon => write!(f, ":"),
            TokenType::DoubleColon => write!(f, "::"),
            TokenType::Comma => write!(f, ","),
            TokenType::Dollar => write!(f, "$"),
            TokenType::OParen => write!(f, "("),
            TokenType::CParen => write!(f, ")"),
            TokenType::OBracket => write!(f, "["),
            TokenType::CBracket => write!(f, "]"),
            TokenType::OCurly => write!(f, "{{"),
            TokenType::CCurly => write!(f, "}}"),
            TokenType::Dot => write!(f, "."),
            TokenType::Eof => write!(f, "Eof"),
            TokenType::Sof => write!(f, "Sof"),
            TokenType::Import => write!(f, "import"),
            TokenType::Macro => write!(f, "macro"),
            TokenType::DoubleOr => write!(f, "||"),
            TokenType::DoubleAnd => write!(f, "&&"),
            TokenType::For => write!(f, "for"),
            TokenType::To => write!(f, "to"),
            TokenType::Defer => write!(f, "defer"),
            TokenType::Static => write!(f, "static"),
            TokenType::Extern => write!(f, "extern"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    pub loc: Loc,
    pub literal: String,
    pub t_type: TokenType,
}

impl Token {
    /// Returns a Token Structure
    ///
    /// # Arguments
    ///
    /// * `t_type` - TokenType extracted by lexer
    /// * `literal` - The String Literal related to the token
    /// * `loc` - The location of the token
    ///
    /// # Examples
    ///
    /// ```
    /// Token::new(TokenType::Int(0),
    ///     "0".to_string(),
    ///     ("./path.nmt".to_string(),1,1)
    ///     );
    /// ```
    pub fn new(t_type: TokenType, literal: String, loc: Loc) -> Self {
        Self {
            literal,
            t_type,
            loc,
        }
    }

    /// Check if The Type of token is indicating the start
    /// or the end of a file
    pub fn is_empty(&self) -> bool {
        matches!(self.t_type, TokenType::Eof | TokenType::Sof)
    }

    /// Creates an empty token wich acts as None in Option<T>
    pub fn empty() -> Token {
        Self {
            literal: String::new(),
            t_type: TokenType::Sof,
            loc: Loc::new("??".to_string(), 1, 1),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Lexer {
    pub file_path: String,
    source: Vec<char>,
    pub token: Token,
    cur: usize,
    bol: usize,
    row: usize,
}

impl Lexer {
    /// Returns an instance of lexer
    ///
    /// # Argments
    ///
    /// * `file_path` - Path of code file mostly for error reporting
    /// * `source` - Code source String extracted from code file
    pub fn new(file_path: String, source: String) -> Self {
        Self {
            file_path,
            source: source.chars().collect::<Vec<char>>(),
            token: Token::empty(),
            cur: 0,
            bol: 0,
            row: 0,
        }
    }

    /// Chacks if self.cur is referencing outside of the code file
    fn is_empty(&self) -> bool {
        self.cur >= self.source.len()
    }

    /// Ignores Every character until a newline reached
    fn drop_line(&mut self) {
        while !self.is_empty() {
            if self.source[self.cur] == '\n' {
                self.drop();
                break;
            } else {
                self.drop();
            }
        }
    }

    /// Increments cur and adjusts bol and row if char under cur is a newline
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

    /// Drops all the whitespaces until reaching a non-WS char
    fn trim_left(&mut self) {
        while !self.is_empty() && self.source[self.cur].is_whitespace() {
            self.drop();
        }
    }

    /// Returns type of the current token
    /// Can exit the program if token is EOF
    pub fn get_token_type(&self) -> TokenType {
        let tk = self.token.clone();
        if tk.t_type == TokenType::Eof {
            error("Expected a Token, found Eof", tk.loc);
        };
        tk.t_type
    }

    /// Checks if the current token type matches the giver token type
    /// Will exit the program if token is not matching
    ///
    /// # Arguments
    ///
    /// * `t_type` - TokenType for matching
    pub fn match_token(&mut self, t_type: TokenType) {
        let tk = self.token.clone();
        if tk.t_type == t_type {
            self.next_token();
        } else {
            error(
                format!("Expected ({}), found ({})", t_type, tk.t_type),
                tk.loc,
            );
        }
    }

    /// Returns the current token
    pub fn get_token(&self) -> Token {
        self.token.clone()
    }

    pub fn get_token_loc(&self) -> Loc {
        self.token.loc.clone()
    }

    pub fn get_current_loc(&self) -> Loc {
        Loc::new(
            self.file_path.clone(),
            self.row + 1,
            self.cur - self.bol + 1,
        )
    }

    /// Scans the next token and sets the current token to the new token
    pub fn next_token(&mut self) -> Token {
        let token = self._next_token();
        self.token = token.clone();
        token
    }

    /// Scans the next token
    fn _next_token(&mut self) -> Token {
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
            return Token::empty();
        }
        let first = self.source[self.cur];
        let loc = self.get_current_loc();

        if first.is_ascii_alphabetic() || first == '_' {
            let index = self.cur;
            while !self.is_empty()
                && (self.source[self.cur].is_ascii_alphanumeric() || self.source[self.cur] == '_')
            {
                self.drop();
            }
            let literal = String::from_iter(self.source[index..self.cur].to_vec());
            match Self::is_keyword(&literal) {
                Some(keyword_token) => return Token::new(keyword_token, literal, loc),
                None => return Token::new(TokenType::Identifier, literal, loc),
            }
        }
        if first.is_ascii_digit() {
            let index = self.cur;
            self.drop();
            while !self.is_empty()
                && (self.source[self.cur].is_ascii_alphanumeric() || self.source[self.cur] == '.')
            {
                self.drop();
            }
            let literal = String::from_iter(self.source[index..self.cur].to_vec());
            let ttype_and_val = self.parse_numeric_literal(&literal);
            return Token::new(ttype_and_val, literal, loc);
        }
        if first == '\'' {
            return self.tokenize_char_literal();
        }

        if first == '"' {
            return self.tokenize_string_literal();
        }

        if let Some(tt) = Self::is_single_char_token(first) {
            self.drop();
            if !self.is_empty() {
                let next = self.source[self.cur];
                if Self::is_single_char_token(next).is_some() {
                    if let Some(dtt) = Self::is_double_char_token(first, next) {
                        self.drop();
                        return Token::new(dtt, String::from_iter(vec![first, next]), loc);
                    }
                }
            }
            return Token::new(tt, first.to_string(), loc);
        }

        error("Unexpected Character", loc);
    }

    /// Tokenses the char literal
    /// ONLY call when current char is (')
    fn tokenize_char_literal(&mut self) -> Token {
        self.drop();
        let literal;
        let char = self.source[self.cur];
        let loc = self.get_current_loc();
        if char == '\'' {
            error("char literal can not be empty", loc);
        }
        if char == '\\' {
            self.drop();
            if self.is_empty() {
                error("char literal unfinished escape sequence", loc);
            }
            let escape = self.source[self.cur];
            match escape {
                'n' => {
                    literal = '\n';
                }
                '\'' => {
                    literal = '\'';
                }
                't' => {
                    literal = '\t';
                }
                'r' => {
                    literal = '\r';
                }
                '\\' => {
                    literal = '\\';
                }
                '0' => {
                    literal = '\0';
                }
                _ => {
                    error(format!("unsupported escape sequence (\\{})", escape), loc);
                }
            }
            self.drop();
        } else {
            literal = char;
            self.drop();
        }

        if !self.is_empty() {
            if self.source[self.cur] != '\'' {
                error("unsupported char", loc);
            }
            self.drop();
            Token::new(TokenType::Char(literal), literal.to_string(), loc)
        } else {
            error("Error: Char literal is not closed properly", loc);
        }
    }

    /// Tokenses the string literal
    /// ONLY call when current char is (")
    fn tokenize_string_literal(&mut self) -> Token {
        self.drop();
        let mut literal = String::new();
        let loc = self.get_current_loc();
        while !self.is_empty() {
            let char = self.source[self.cur];
            if char == '\"' {
                break;
            }
            if char == '\n' {
                error("string literal not closed before end of line", loc);
            }
            if char == '\\' {
                self.drop();
                if self.is_empty() {
                    error("string literal unfinished escape sequence", loc);
                }

                let escape = self.source[self.cur];
                match escape {
                    'n' => {
                        literal.push('\n');
                        self.drop();
                    }
                    '"' => {
                        literal.push('"');
                        self.drop();
                    }
                    't' => {
                        literal.push('\t');
                        self.drop();
                    }
                    'r' => {
                        literal.push('\r');
                        self.drop();
                    }
                    '0' => {
                        literal.push('\0');
                        self.drop();
                    }
                    '\\' => {
                        literal.push('\\');
                        self.drop();
                    }
                    _ => {
                        error(format!("unsupported escape sequence (\\{})", escape), loc);
                    }
                }
            } else {
                literal.push(char);
                self.drop();
            }
        }
        if !self.is_empty() {
            self.drop();
            Token::new(TokenType::String, literal, loc)
        } else {
            error("Error: String literal is not closed properly", loc);
        }
    }

    /// Checks if literal is a keyword
    /// Returns Some(Token) if matches and None if not
    ///
    /// # Arguments
    ///
    /// * `literal` - token literal that we whant to check
    fn is_keyword(literal: &str) -> Option<TokenType> {
        match literal {
            "if" => Some(TokenType::If),
            "else" => Some(TokenType::Else),
            "func" => Some(TokenType::Func),
            "struct" => Some(TokenType::Struct),
            "var" => Some(TokenType::Var),
            "return" => Some(TokenType::Return),
            "while" => Some(TokenType::While),
            "break" => Some(TokenType::Break),
            "continue" => Some(TokenType::Continue),
            "print" => Some(TokenType::Print),
            "true" => Some(TokenType::True),
            "false" => Some(TokenType::False),
            "asm" => Some(TokenType::Asm),
            "import" => Some(TokenType::Import),
            "macro" => Some(TokenType::Macro),
            "ffi" => Some(TokenType::Ffi),
            "for" => Some(TokenType::For),
            "to" => Some(TokenType::To),
            "defer" => Some(TokenType::Defer),
            "static" => Some(TokenType::Static),
            "extern" => Some(TokenType::Extern),
            _ => None,
        }
    }

    /// Checks if a char in literal is a token
    /// Returns Some(Token) if matches and None if not
    ///
    /// # Arguments
    ///
    /// * `literal` - token literal that we whant to check
    fn is_single_char_token(char: char) -> Option<TokenType> {
        match char {
            '{' => Some(TokenType::OCurly),
            '}' => Some(TokenType::CCurly),
            '[' => Some(TokenType::OBracket),
            ']' => Some(TokenType::CBracket),
            '(' => Some(TokenType::OParen),
            ')' => Some(TokenType::CParen),
            '-' => Some(TokenType::Minus),
            '+' => Some(TokenType::Plus),
            '*' => Some(TokenType::Multi),
            '/' => Some(TokenType::Devide),
            '#' => Some(TokenType::Hash),
            '$' => Some(TokenType::Dollar),
            '?' => Some(TokenType::QMark),
            ':' => Some(TokenType::Colon),
            ';' => Some(TokenType::SemiColon),
            '.' => Some(TokenType::Dot),
            ',' => Some(TokenType::Comma),
            '=' => Some(TokenType::Eq),
            '!' => Some(TokenType::Not),
            '>' => Some(TokenType::Bigger),
            '<' => Some(TokenType::Smaller),
            '@' => Some(TokenType::ATSign),
            '%' => Some(TokenType::Mod),
            '&' => Some(TokenType::And),
            '|' => Some(TokenType::Or),
            _ => None,
        }
    }

    /// Checks if a string in literal is a token
    /// Returns Some(Token) if matches and None if not
    ///
    /// # Arguments
    ///
    /// * `literal` - token literal that we whant to check
    fn is_double_char_token(first: char, next: char) -> Option<TokenType> {
        let mut double_char = String::new();
        double_char.push(first);
        double_char.push(next);
        match double_char.as_str() {
            "+=" => Some(TokenType::PlusEq),
            "-=" => Some(TokenType::SubEq),
            "*=" => Some(TokenType::MultiEq),
            "/=" => Some(TokenType::DivEq),
            "%=" => Some(TokenType::ModEq),
            "==" => Some(TokenType::DoubleEq),
            ":=" => Some(TokenType::ColonEq),
            "::" => Some(TokenType::DoubleColon),
            "!=" => Some(TokenType::NotEq),
            ">=" => Some(TokenType::BiggerEq),
            "<=" => Some(TokenType::SmallerEq),
            "<<" => Some(TokenType::Lsh),
            ">>" => Some(TokenType::Rsh),
            "||" => Some(TokenType::DoubleOr),
            "&&" => Some(TokenType::DoubleAnd),
            _ => None,
        }
    }

    /// Parse numeric literal to a numeric TokenType
    /// Can Exit the program if can not parse the lietal
    ///
    /// # Arguments
    ///
    /// * `literal` - token literal that we whant to check
    fn parse_numeric_literal(&self, literal: &String) -> TokenType {
        // 0x001 0xff 0b0010
        let loc = self.get_current_loc();
        let mut lit_chars = literal.chars();
        if literal.contains('x') {
            self.expect_char(&lit_chars.next(), vec!['0']);
            self.expect_char(&lit_chars.next(), vec!['x']);
            let mut value: i64 = 0;
            for ch in lit_chars {
                let digit = ch.to_digit(16).unwrap_or_else(|| {
                    error(
                        format!("Unknown character in parsing ({})", literal),
                        loc.clone(),
                    );
                });
                value = (value * 16i64) + digit as i64;
            }
            TokenType::Int(value as i32)
        } else if literal.contains('b') {
            self.expect_char(&lit_chars.next(), vec!['0']);
            self.expect_char(&lit_chars.next(), vec!['b']);
            let mut value: i32 = 0;
            for ch in lit_chars {
                let digit = ch.to_digit(2).unwrap_or_else(|| {
                    error(
                        format!("Unknown character in parsing ({})", literal),
                        loc.clone(),
                    );
                });
                value = (value * 2i32) + digit as i32;
            }
            TokenType::Int(value)
        } else if literal.contains('.') {
            let value: f64 = literal.parse::<f64>().unwrap();
            TokenType::Float(value)
        } else {
            let value: i32 = literal.parse::<i32>().unwrap();
            TokenType::Int(value)
        }
    }

    /// Returns char if exits in a list
    /// Will Exit the program if no match
    fn expect_char(&self, copt: &Option<char>, chars: Vec<char>) -> char {
        let loc = self.get_current_loc();
        let char = copt.unwrap_or_else(|| {
            error("Undifined character set for numbers", loc);
        });
        if chars.contains(&char) {
            return char;
        }
        char
    }
}

#[test]
fn expr_tokens() {
    let mut lexer = Lexer::new(String::new(), "a + (3 * 4) - 2".to_string());
    assert_eq!(lexer.next_token().t_type, TokenType::Identifier);
    assert_eq!(lexer.next_token().t_type, TokenType::Plus);
    assert_eq!(lexer.next_token().t_type, TokenType::OParen);
    assert_eq!(lexer.next_token().t_type, TokenType::Int(3));
    assert_eq!(lexer.next_token().t_type, TokenType::Multi);
    assert_eq!(lexer.next_token().t_type, TokenType::Int(4));
    assert_eq!(lexer.next_token().t_type, TokenType::CParen);
    assert_eq!(lexer.next_token().t_type, TokenType::Minus);
    assert_eq!(lexer.next_token().t_type, TokenType::Int(2));
}

#[test]
fn string_literal() {
    let mut lexer = Lexer::new(String::new(), "\"test\"".to_string());
    assert_eq!(lexer.tokenize_string_literal().t_type, TokenType::String);
}

#[test]
fn string_literal_escape_seq() {
    let mut lexer = Lexer::new(String::new(), "\"test\\ntest\"".to_string());
    assert_eq!(lexer.tokenize_string_literal().t_type, TokenType::String);
    let mut lexer = Lexer::new(String::new(), "\"\\\"test\\\"\"".to_string());
    assert_eq!(lexer.tokenize_string_literal().t_type, TokenType::String);
}
