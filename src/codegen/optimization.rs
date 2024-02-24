use crate::{parser::{types::VariableType, expr::Op}, error_handeling::CompilationError};

use super::instructions::Opr;

#[derive(Debug, Clone, PartialEq)]
pub struct ExprOpr {
    pub value: Opr,
    pub vtype: VariableType,
}

impl ExprOpr {
    pub fn new(value: impl Into<Opr>, vtype: VariableType) -> Self {
        Self {
            value: value.into(),
            vtype
        }
    }
}

pub fn compact_binary_expr(left: &ExprOpr, right: &ExprOpr, op: &Op) -> Result<ExprOpr, CompilationError> {
    let vtype = left.vtype.cast(&right.vtype)?;
    let l_val = left.value.get_literal_value();
    let r_val = right.value.get_literal_value();
    match op {
        Op::Plus => Ok(ExprOpr::new(l_val + r_val, vtype)),
        Op::Sub => Ok(ExprOpr::new(l_val - r_val, vtype)),
        Op::Multi => Ok(ExprOpr::new(l_val * r_val, vtype)),
        Op::Devide => Ok(ExprOpr::new(l_val / r_val, vtype)),
        Op::Mod => Ok(ExprOpr::new(l_val % r_val, vtype)),
        Op::Or => Ok(ExprOpr::new(l_val | r_val, vtype)),
        Op::And => Ok(ExprOpr::new(l_val & r_val, vtype)),
        Op::Lsh => Ok(ExprOpr::new(l_val << r_val, vtype)),
        Op::Rsh => Ok(ExprOpr::new(l_val >> r_val, vtype)),
        Op::LogicalOr => Ok(ExprOpr::new(l_val | r_val, VariableType::Bool)),
        Op::LogicalAnd => Ok(ExprOpr::new(l_val & r_val, VariableType::Bool)),
        Op::Not => {
            return Err(CompilationError::InValidBinaryOperation(
                    op.to_owned(),
                    left.vtype.to_string(),
                    right.vtype.to_string(),
                    ));
        }
    }
}
