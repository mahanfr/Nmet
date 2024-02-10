/**********************************************************************************************
*
*   parser/assign: parse assign syntax
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
use crate::{
    error_handeling::error,
    lexer::{Lexer, TokenType},
};

use super::{
    expr::{expr, Expr},
    stmt::{Stmt, StmtType},
};

/// Assign Operation
/// Part of stmts, holds information on assgining a expr to a memory location
///
/// * Left: left side of assignment usually should be a memory variable
/// * Right: right side of assignment that can be any valid expression
/// * op: assginment operation (== or += or ...)
#[derive(Debug, Clone)]
pub struct Assign {
    pub left: Expr,
    pub right: Expr,
    pub op: AssignOp,
}

/// Assgin operand
/// Different supported assgin operands
#[derive(Debug, Clone)]
pub enum AssignOp {
    /// == move value to memory
    Eq,
    /// += move and add to current momory
    PlusEq,
    /// -= move and sub from current momory
    SubEq,
    /// *= move and multiply to the current memory
    MultiEq,
    /// /= move and devide from the current memory
    DevideEq,
    /// %= move the modulo to current memory
    ModEq,
}
impl AssignOp {
    /// Convert TokenType to AssignOp
    pub fn from_token_type(ttype: &TokenType) -> Self {
        match ttype {
            TokenType::Eq => Self::Eq,
            TokenType::PlusEq => Self::PlusEq,
            TokenType::SubEq => Self::SubEq,
            TokenType::MultiEq => Self::MultiEq,
            TokenType::DivEq => Self::DevideEq,
            TokenType::ModEq => Self::ModEq,
            _ => {
                unreachable!();
            }
        }
    }
}

/// parse assignemts
pub fn assign(lexer: &mut Lexer) -> Stmt {
    // Location to Start of the stmt
    let loc = lexer.get_token_loc();
    let left_expr = expr(lexer);
    let token_type = lexer.get_token_type();
    // Stmt is an expr if you encounter a semicolon
    if token_type == TokenType::SemiColon {
        lexer.match_token(TokenType::SemiColon);
        Stmt {
            stype: StmtType::Expr(left_expr),
            loc,
        }
    } else if token_type.is_assgin_token() {
        let op_type = AssignOp::from_token_type(&token_type);
        lexer.match_token(token_type);
        let right_expr = expr(lexer);
        lexer.match_token(TokenType::SemiColon);
        return Stmt {
            stype: StmtType::Assign(Assign {
                left: left_expr,
                right: right_expr,
                op: op_type,
            }),
            loc,
        };
    } else {
        error(
            format!("Expected Semicolon found ({})", lexer.get_token_type()),
            loc,
        );
    }
}
