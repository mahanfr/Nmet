use crate::{parser::{expr::{Expr, ExprType, Op}, types::VariableType}, error_handeling::error};

pub fn expr_type(expr: &Expr) -> VariableType {
    match &expr.etype {
        ExprType::Int(_) => VariableType::Int,
        ExprType::Char(_) => VariableType::Char,
        ExprType::String(_) => VariableType::String,
        ExprType::Ptr(_) => VariableType::Pointer,
        ExprType::Variable(_) => VariableType::Any,
        ExprType::FunctionCall(_) => VariableType::Any,
        ExprType::ArrayIndex(_) => VariableType::Any,
        ExprType::Unary(u) => {
            let right_type = expr_type(&u.right);
            match u.op {
                Op::Sub => {
                    if right_type == VariableType::UInt {
                        VariableType::Int
                    } else {
                        right_type
                    }
                }
                Op::Plus | Op::Not => {
                    if matches!(right_type, VariableType::Int | VariableType::UInt | VariableType::Char) {
                       right_type
                    } else {
                        error(format!("Invalid Operation ({}) for type ({})",u.op,right_type),expr.loc.clone());
                    }
                }
                _ => unreachable!()

            }
        }
        ExprType::Compare(cmp) => {
            let right_type = expr_type(&cmp.right);
            let left_type = expr_type(&cmp.left);
            if  (right_type == left_type) ||
                (right_type.is_numeric() && left_type.is_numeric()) {
                VariableType::Bool
            } else {
                error(format!("Invalid Comparison between types: ({}) and ({})",left_type,right_type),expr.loc.clone());
            }
        }
        ExprType::Binary(b) => {
            let right_type = expr_type(&b.right);
            let left_type = expr_type(&b.left);
            if right_type.is_numeric() && left_type.is_numeric() {
                match left_type.binary_cast(&right_type) {
                    Ok(t) => t,
                    Err(msg) => error(msg,expr.loc.clone())
                }
            } else {
                error(
                    format!("Invalid Operation ({}) on non-numeric types: ({}) and ({})",
                        b.op,
                        left_type,
                        right_type),
                    expr.loc.clone());
            }
        }
    }
}
