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
        !self.value.is_literal() || !self.value.is_mem()
    }
}

pub fn optim_compare_expr(
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
