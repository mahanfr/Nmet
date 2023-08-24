use crate::{
    error_handeling::{error, Loc},
    lexer::{Lexer, TokenType},
};
use core::fmt::Display;

/// Expr
/// Contains Informationn about and expression
/// * loc - location of current expr
/// * etype - expression token type
#[derive(Debug, PartialEq, Clone)]
pub struct Expr {
    pub loc: Loc,
    pub etype: ExprType,
}

/// All Supported Expression types
#[derive(Debug, PartialEq, Clone)]
pub enum ExprType {
    /// Op + Expr
    /// e.g: -2 +var !cat()
    Unary(UnaryExpr),
    /// Expr + Op + Expr
    /// e.g: 12 + 3 , 4 * a
    Binary(BinaryExpr),
    /// Expr + CompareOp + Expr
    /// e.g: 1 == 2, a <= 8
    Compare(CompareExpr),
    /// Integer values
    /// e.g: 10
    Int(i32),
    /// Character values
    /// e.g: 'a', '\n', 10
    Char(u8),
    /// Address of expr in memory
    /// e.g: ptr a
    Ptr(Box<Expr>),
    /// String values
    /// e.g: "Hello\n"
    String(String),
    /// Variables
    /// e.g:a, x, y
    Variable(String),
    /// Function Call
    /// e.g: cat(), is_odd(10)
    FunctionCall(FunctionCall),
    /// Array Index
    /// e.g: list[10]
    ArrayIndex(ArrayIndex),
    /// Bool
    /// e.g: True, False
    Bool(u8),
}
impl ExprType {
    /// returns true if token type is used in binary operations
    pub fn is_binary_op(t_token: TokenType) -> bool {
        matches!(
            t_token,
            TokenType::Plus | TokenType::Minus | TokenType::And | TokenType::Or
        )
    }

    /// returns true if token type is used in logical operations
    pub fn is_logical_op(t_token: TokenType) -> bool {
        matches!(t_token, TokenType::DoubleOr | TokenType::DoubleAnd)
    }

    /// returns true if token type is used in term operations
    pub fn is_term_op(t_token: TokenType) -> bool {
        matches!(
            t_token,
            TokenType::Multi | TokenType::Devide | TokenType::Mod | TokenType::Rsh | TokenType::Lsh
        )
    }

    /// returns true if token type is used in compare operations
    pub fn is_compare_op(t_token: TokenType) -> bool {
        matches!(
            t_token,
            TokenType::DoubleEq
                | TokenType::NotEq
                | TokenType::Bigger
                | TokenType::Smaller
                | TokenType::BiggerEq
                | TokenType::SmallerEq
        )
    }
}

/// Unaray Expr
/// Used for Exprssion with On Expression and one Operation
/// * op: Operation Operation
/// * right: Expression in the right side of operation
#[derive(Debug, PartialEq, Clone)]
pub struct UnaryExpr {
    pub op: Op,
    pub right: Box<Expr>,
}

/// Binary Expr
/// Used of Expressions with one Operation and two Expression
/// * left: Expression in the left side of operation
/// * op: Operation Operation
/// * right: Expression in the right side of operation
#[derive(Debug, PartialEq, Clone)]
pub struct BinaryExpr {
    pub left: Box<Expr>,
    pub op: Op,
    pub right: Box<Expr>,
}

/// All Supported Mathematical Operations
#[derive(Debug, PartialEq, Clone)]
pub enum Op {
    /// addition and plus sign
    Plus,
    /// subtraction and negative sign
    Sub,
    /// multipication
    Multi,
    /// divition
    Devide,
    /// Binary Not
    Not,
    /// Modulo
    Mod,
    /// Binary And
    And,
    /// Binary Or
    Or,
    /// Binary Left Shift
    Lsh,
    /// Binary Right Shift
    Rsh,
    /// Logical And
    LogicalAnd,
    /// Logical Or
    LogicalOr,
}
impl Op {
    /// Convert token type to Op
    pub fn from_token_type(token_type: TokenType) -> Self {
        match token_type {
            TokenType::Plus => Self::Plus,
            TokenType::Minus => Self::Sub,
            TokenType::Multi => Self::Multi,
            TokenType::Devide => Self::Devide,
            TokenType::Not => Self::Not,
            TokenType::Mod => Self::Mod,
            TokenType::And => Self::And,
            TokenType::Or => Self::Or,
            TokenType::Lsh => Self::Lsh,
            TokenType::Rsh => Self::Rsh,
            TokenType::DoubleAnd => Self::LogicalAnd,
            TokenType::DoubleOr => Self::LogicalOr,
            _ => {
                todo!("return Error");
            }
        }
    }
}
impl Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Op::Plus => write!(f, "+"),
            Op::Sub => write!(f, "-"),
            Op::Multi => write!(f, "*"),
            Op::Devide => write!(f, "/"),
            Op::Not => write!(f, "!"),
            Op::Mod => write!(f, "%"),
            Op::And => write!(f, "&"),
            Op::Or => write!(f, "|"),
            Op::Lsh => write!(f, "<<"),
            Op::Rsh => write!(f, ">>"),
            Op::LogicalOr => write!(f, "||"),
            Op::LogicalAnd => write!(f, "&&"),
        }
    }
}

/// Function call
/// Used for Function call Expression
/// * ident - name of the function
/// * args  - function call argumnets
#[derive(Debug, PartialEq, Clone)]
pub struct FunctionCall {
    pub ident: String,
    pub args: Vec<Expr>,
}

/// Array Index
/// Used for Array Indexing Expression
/// * ident - name of the variable
/// * indexer - expresstion that indexes the array
#[derive(Debug, PartialEq, Clone)]
pub struct ArrayIndex {
    pub ident: String,
    pub indexer: Box<Expr>,
}

/// Compare Expr
/// Used of iCompare Expressions with one Operation and two Expression
/// * left: Expression in the left side of operation
/// * op: Compare Operation
/// * right: Expression in the right side of operation
#[derive(Debug, PartialEq, Clone)]
pub struct CompareExpr {
    pub left: Box<Expr>,
    pub op: CompareOp,
    pub right: Box<Expr>,
}

/// All Supported Compare Operation
#[derive(Debug, PartialEq, Clone)]
pub enum CompareOp {
    /// != Not Equal
    NotEq,
    /// == Equal
    Eq,
    /// > Bigger
    Bigger,
    /// < Smaller
    Smaller,
    /// >= Bigger Equal
    BiggerEq,
    /// <= Smaller Equal
    SmallerEq,
}
impl CompareOp {
    /// Converts token type to compare operation
    pub fn from_token_type(t_type: TokenType) -> Self {
        match t_type {
            TokenType::DoubleEq => Self::Eq,
            TokenType::NotEq => Self::NotEq,
            TokenType::Bigger => Self::Bigger,
            TokenType::Smaller => Self::Smaller,
            TokenType::BiggerEq => Self::BiggerEq,
            TokenType::SmallerEq => Self::SmallerEq,
            _ => {
                unreachable!();
            }
        }
    }
}

/// Parsing Expr
/// returns Least Prioraty operations
/// e.g: Plus, Minus, bitwise Or
pub fn expr(lexer: &mut Lexer) -> Expr {
    let mut term_expr = term(lexer);
    loop {
        let t_type = lexer.get_token_type();
        if ExprType::is_binary_op(t_type) || ExprType::is_logical_op(t_type) {
            let op = Op::from_token_type(t_type);
            lexer.next_token();
            let right = term(lexer);
            term_expr = Expr {
                etype: ExprType::Binary(BinaryExpr {
                    left: Box::new(term_expr),
                    op,
                    right: Box::new(right),
                }),
                loc: lexer.get_token_loc(),
            };
        } else {
            break;
        }
    }
    term_expr
}

/// Parsing Expr
/// returns second Prioraty operations
/// e.g: Multi, Devide, Logical And
pub fn term(lexer: &mut Lexer) -> Expr {
    let mut left = factor(lexer);
    let mut cur_token = lexer.get_token_type();
    while ExprType::is_term_op(cur_token) || ExprType::is_compare_op(cur_token) {
        if ExprType::is_compare_op(cur_token) {
            let op = CompareOp::from_token_type(lexer.get_token_type());
            lexer.next_token();
            let right = term(lexer);
            left = Expr {
                etype: ExprType::Compare(CompareExpr {
                    left: Box::new(left),
                    op,
                    right: Box::new(right),
                }),
                loc: lexer.get_token_loc(),
            };
        } else if ExprType::is_term_op(cur_token) {
            let op = Op::from_token_type(lexer.get_token_type());
            lexer.next_token();
            let right = factor(lexer);
            left = Expr {
                etype: ExprType::Binary(BinaryExpr {
                    left: Box::new(left),
                    op,
                    right: Box::new(right),
                }),
                loc: lexer.get_token_loc(),
            };
        }
        cur_token = lexer.get_token_type();
    }
    left
}

/// Parsing Expr
/// returns first Prioraty operations
/// e.g: Unary, Paran, Power
pub fn factor(lexer: &mut Lexer) -> Expr {
    let loc = lexer.get_current_loc();
    match lexer.get_token_type() {
        TokenType::OParen => {
            lexer.match_token(TokenType::OParen);
            let value = expr(lexer);
            lexer.match_token(TokenType::CParen);
            value
        }
        TokenType::Plus | TokenType::Minus | TokenType::Not => {
            let op = Op::from_token_type(lexer.get_token_type());
            lexer.next_token();
            let value = factor(lexer);
            Expr {
                etype: ExprType::Unary(UnaryExpr {
                    op,
                    right: Box::new(value),
                }),
                loc,
            }
        }
        TokenType::String => {
            let str_token = lexer.get_token();
            lexer.next_token();
            Expr {
                etype: ExprType::String(str_token.literal),
                loc,
            }
        }
        TokenType::Ptr => {
            lexer.match_token(TokenType::Ptr);
            let value = expr(lexer);
            Expr {
                etype: ExprType::Ptr(Box::new(value)),
                loc,
            }
        }
        TokenType::True => {
            lexer.match_token(TokenType::True);
            Expr {
                etype: ExprType::Bool(1),
                loc,
            }
        }
        TokenType::False => {
            lexer.match_token(TokenType::False);
            Expr {
                etype: ExprType::Bool(0),
                loc,
            }
        }
        TokenType::Char(c) => {
            lexer.next_token();
            Expr {
                etype: ExprType::Char(c as u8),
                loc,
            }
        }
        TokenType::Int(val) => {
            lexer.next_token();
            Expr {
                etype: ExprType::Int(val),
                loc,
            }
        }
        TokenType::Identifier => {
            let ident_name = lexer.get_token().literal;
            if lexer.next_token().is_empty() {
                return Expr {
                    etype: ExprType::Variable(ident_name),
                    loc,
                };
            }
            match lexer.get_token_type() {
                TokenType::OParen => {
                    let args = function_call_args(lexer);
                    Expr {
                        etype: ExprType::FunctionCall(FunctionCall {
                            ident: ident_name,
                            args,
                        }),
                        loc,
                    }
                }
                TokenType::OBracket => {
                    let indexer = array_indexer(lexer);
                    Expr {
                        etype: ExprType::ArrayIndex(ArrayIndex {
                            ident: ident_name,
                            indexer: Box::new(indexer),
                        }),
                        loc,
                    }
                }
                _ => Expr {
                    etype: ExprType::Variable(ident_name),
                    loc,
                },
            }
        }
        _ => {
            error(
                format!(
                    "Unexpected Token ({}) while parsing expr",
                    lexer.get_token_type(),
                ),
                loc,
            );
        }
    }
}

/// Parsing Array Index
pub fn array_indexer(lexer: &mut Lexer) -> Expr {
    lexer.match_token(TokenType::OBracket);
    let index = expr(lexer);
    lexer.match_token(TokenType::CBracket);
    index
}

/// Parsing Function call
/// Returns Function call argumets
pub fn function_call_args(lexer: &mut Lexer) -> Vec<Expr> {
    let mut args = Vec::<Expr>::new();
    lexer.match_token(TokenType::OParen);
    loop {
        //|| | expr | expr , expr
        match lexer.get_token_type() {
            TokenType::CParen => {
                lexer.match_token(TokenType::CParen);
                break;
            }
            _ => {
                args.push(expr(lexer));
                if lexer.get_token_type() == TokenType::Comma {
                    lexer.match_token(TokenType::Comma);
                }
            }
        }
    }
    args
}
