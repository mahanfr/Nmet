use crate::{
    error_handeling::CompilationError,
    parser::{expr::{Op, CompareOp}, types::VariableType}, codegen::instructions::Opr
};

#[derive(Debug, Clone, PartialEq)]
pub struct ExprOpr {
    pub value: Opr,
    pub vtype: VariableType,
}

impl ExprOpr {
    pub fn new(value: impl Into<Opr>, vtype: VariableType) -> Self {
        Self {
            value: value.into(),
            vtype,
        }
    }

    pub fn needs_stack(&self) -> bool {
        !self.value.is_literal() && !self.value.is_mem()
    }
}

pub fn fold_binary_expr(
    left: &ExprOpr,
    right: &ExprOpr,
    op: &Op,
) -> Result<ExprOpr, CompilationError> {
    let res_type = left.vtype.cast(&right.vtype)?;
    let l_val = left.value.get_literal_value();
    let r_val = right.value.get_literal_value();
    let val = match op {
        Op::Plus => l_val + r_val,
        Op::Sub => l_val - r_val,
        Op::Multi => l_val * r_val,
        Op::Devide => l_val / r_val,
        Op::Mod => l_val % r_val,
        Op::And => l_val & r_val,
        Op::Or => l_val | r_val,
        Op::Lsh => l_val << r_val,
        Op::Rsh => l_val >> r_val,
        Op::LogicalOr => ((l_val != 0) || (r_val != 0)) as i64,
        Op::LogicalAnd => ((l_val != 0) && (r_val != 0)) as i64,
        Op::Not => {
            return Err(CompilationError::InValidBinaryOperation(
                op.to_owned(),
                left.vtype.to_string(),
                right.vtype.to_string(),
            ));
        }
    };
    Ok(ExprOpr::new(val, res_type))
}

pub fn fold_compare_expr(
    left: &ExprOpr,
    right: &ExprOpr,
    op: &CompareOp,
) -> Result<ExprOpr, CompilationError> {
    let _ = left.vtype.cast(&right.vtype)?;
    let l_val = left.value.get_literal_value();
    let r_val = right.value.get_literal_value();
    let val = match op {
        CompareOp::Eq => (l_val == r_val) as i32,
        CompareOp::NotEq => (l_val != r_val) as i32,
        CompareOp::Bigger => (l_val > r_val) as i32,
        CompareOp::Smaller => (l_val < r_val) as i32,
        CompareOp::BiggerEq => (l_val >= r_val) as i32,
        CompareOp::SmallerEq => (l_val <= r_val) as i32,
    };
    Ok(ExprOpr::new(val, VariableType::Bool))
}

pub fn fold_unary_expr(
    left: &ExprOpr,
    op: &Op,
) -> Result<ExprOpr, CompilationError> {
    let l_val = left.value.get_literal_value();
    let val = match op {
        Op::Sub => -l_val,
        Op::Plus => l_val,
        Op::Not => !l_val,
        _ => {
            unreachable!();
        }
    };
    Ok(ExprOpr::new(val, left.vtype.to_owned()))
}
