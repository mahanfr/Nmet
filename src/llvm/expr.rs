use crate::{
    error_handeling::error,
    parser::{
        expr::{CompareOp, Expr, ExprType, FunctionCall, Op},
        types::VariableType,
    },
};

use super::{
    variables::{find_variable, get_vriable_map},
    CompilerContext,
};

pub fn compile_expr(cc: &mut CompilerContext, expr: &Expr) -> VariableType {
    match &expr.etype {
        ExprType::Variable(v) => {
            todo!();
        }
        ExprType::Bool(b) => {
            todo!();
            // VariableType::Bool
        }
        ExprType::Char(x) => {
            todo!();
            // VariableType::Char
        }
        ExprType::Int(x) => {
            todo!();
            // VariableType::Int
        }
        ExprType::Compare(c) => {
            let left_type = compile_expr(cc, c.left.as_ref());
            let right_type = compile_expr(cc, c.right.as_ref());
            match c.op {
                CompareOp::Eq => {
                    todo!();
                }
                CompareOp::NotEq => {
                    todo!();
                }
                CompareOp::Bigger => {
                    todo!();
                }
                CompareOp::Smaller => {
                    todo!();
                }
                CompareOp::BiggerEq => {
                    todo!();
                }
                CompareOp::SmallerEq => {
                    todo!();
                }
            }
            // if (right_type == left_type) || (right_type.is_numeric() && left_type.is_numeric()) {
            //     VariableType::Bool
            // } else {
            //     error(
            //         format!(
            //             "Invalid Comparison between types: ({}) and ({})",
            //             left_type, right_type
            //         ),
            //         expr.loc.clone(),
            //     );
            // }
        }
        ExprType::Binary(b) => {
            let left_type = compile_expr(cc, b.left.as_ref());
            let right_type = compile_expr(cc, b.right.as_ref());
            match b.op {
                Op::Plus => {
                    todo!();
                }
                Op::Sub => {
                    todo!();
                }
                Op::Multi => {
                    todo!();
                }
                Op::Devide => {
                    todo!();
                }
                Op::Mod => {
                    todo!();
                }
                Op::Or => {
                    todo!();
                }
                Op::And => {
                    todo!();
                }
                Op::Lsh => {
                    todo!();
                }
                Op::Rsh => {
                    todo!();
                }
                Op::LogicalOr => {
                    todo!();
                    // return VariableType::Bool;
                }
                Op::LogicalAnd => {
                    todo!();
                    // return VariableType::Bool;
                }
                Op::Not => {
                    panic!("Unvalid binary operation");
                }
            }
            // if right_type.is_numeric() && left_type.is_numeric() {
            //     match left_type.cast(&right_type) {
            //         Ok(t) => t,
            //         Err(msg) => error(msg, expr.loc.clone()),
            //     }
            // } else {
            //     error(
            //         format!(
            //             "Invalid Operation ({}) on non-numeric types: ({}) and ({})",
            //             b.op, left_type, right_type
            //         ),
            //         expr.loc.clone(),
            //     );
            // }
        }
        ExprType::String(str) => {
            todo!();
            // VariableType::String
        }
        ExprType::Unary(u) => {
            let right_type = compile_expr(cc, &u.right);
            match u.op {
                Op::Sub => {
                    assert!(false);
                    if right_type == VariableType::UInt {
                        return VariableType::Int;
                    } else {
                        return right_type;
                    }
                }
                Op::Plus => {
                    assert!(false);
                }
                Op::Not => {
                    assert!(false);
                }
                _ => {
                    unreachable!();
                }
            }
            if right_type.is_numeric() {
                right_type
            } else {
                error(
                    format!("Invalid Operation ({}) for type ({})", u.op, right_type),
                    expr.loc.clone(),
                );
            }
        }
        ExprType::FunctionCall(fc) => match compile_function_call(cc, fc) {
            Ok(ftype) => ftype,
            Err(msg) => error(msg, expr.loc.clone()),
        },
        ExprType::Ptr(e) => {
            compile_ptr(cc, e);
            VariableType::Pointer
        }
        ExprType::DeRef(r) => {
            let t = compile_expr(cc, r);
            if t != VariableType::Pointer {
                error(format!("Expected a Pointer found ({t})"), expr.loc.clone());
            }
            assert!(false);
            VariableType::Any
        }
        ExprType::ArrayIndex(ai) => {
            todo!();
        }
    }
}

fn compile_ptr(cc: &mut CompilerContext, expr: &Expr) {
    match &expr.etype {
        ExprType::Variable(v) => {
            assert!(false);
        }
        _ => {
            todo!("Impl Pointers");
        }
    }
}

fn compile_function_call(
    cc: &mut CompilerContext,
    fc: &FunctionCall,
) -> Result<VariableType, String> {
    for (index, arg) in fc.args.iter().enumerate() {
        // let argv_type = expr_type(arg);
        compile_expr(cc, arg);
        match arg.etype {
            ExprType::String(_) => {
                todo!();
            }
            _ => {
                todo!();
            }
        }
    }
    // TODO: Setup a unresolved function table
    let Some(fun) = cc.functions_map.get(&fc.ident) else {
            return Err(
            format!(
                "Error: Function {} is not avaliable in this scope. {}",
                &fc.ident,
                "Make sure you are calling the correct function"
            ))
        };
    assert!(false);
    if fun.ret_type != VariableType::Void {
        todo!();
    }
    Ok(fun.ret_type.clone())
}

fn asmfy_string(str: &str) -> String {
    let mut res = String::new();
    let source: Vec<char> = str.chars().collect();
    let mut i = 0;
    while i < source.len() {
        match source[i] {
            '\n' => {
                if !res.is_empty() {
                    res.push(',');
                }
                res.push_str(" 10");
                i += 1;
            }
            '\t' => {
                if !res.is_empty() {
                    res.push(',');
                }
                res.push_str(" 9");
                i += 1;
            }
            '\r' => {
                if !res.is_empty() {
                    res.push(',');
                }
                res.push_str(" 13");
                i += 1;
            }
            '\"' => {
                if !res.is_empty() {
                    res.push(',');
                }
                res.push_str(" 34");
                i += 1;
            }
            _ => {
                if !res.is_empty() {
                    res.push(',');
                }
                res.push('\"');
                while i < source.len() {
                    if source[i] == '\n'
                        || source[i] == '\"'
                        || source[i] == '\t'
                        || source[i] == '\r'
                    {
                        break;
                    }
                    res.push(source[i]);
                    i += 1;
                }
                res.push('\"');
            }
        }
    }
    res
}
