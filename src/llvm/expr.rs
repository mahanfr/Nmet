use crate::{
    error_handeling::error,
    llvm::variables::get_vriable_map,
    parser::{
        expr::{CompareOp, Expr, ExprType, FunctionCall, Op},
        types::VariableType,
    },
};

use super::CompilerContext;

pub fn compile_expr(cc: &mut CompilerContext, expr: &Expr) -> (String, VariableType) {
    match &expr.etype {
        ExprType::Variable(v) => match get_vriable_map(cc, v) {
            Some(m) => {
                return (format!("%{}", m._ident), m.vtype);
            }
            None => {
                error("Variable has been defined in this scope!", expr.loc.clone());
            }
        },
        ExprType::Bool(b) => {
            todo!();
            // VariableType::Bool
        }
        ExprType::Char(x) => {
            todo!();
            // VariableType::Char
        }
        ExprType::Int(x) => (format!("{x}"), VariableType::Int),
        ExprType::Compare(c) => {
            let (mut ltag, mut ltype) = compile_expr(cc, c.left.as_ref());
            let (mut rtag, mut rtype) = compile_expr(cc, c.right.as_ref());
            let mut id = cc.instruct_buf.len();
            if ltag.starts_with("%") {
                let code = format!("%{id} = load {ltype}, ptr {ltag}, align {}", ltype.size());
                ltag = format!("%{id}");
                cc.instruct_buf.push(code);
                id += 1;
            }
            if rtag.starts_with("%") {
                let code = format!("%{id} = load {rtype}, ptr {rtag}, align {}", rtype.size());
                rtag = format!("%{id}");
                cc.instruct_buf.push(code);
                id += 1;
            }
            let cmp_type = match c.op {
                CompareOp::Eq => "eq".to_string(),
                CompareOp::NotEq => "ne".to_string(),
                CompareOp::Bigger => "sgt".to_string(),
                CompareOp::Smaller => "slt".to_string(),
                CompareOp::BiggerEq => "sge".to_string(),
                CompareOp::SmallerEq => "sle".to_string(),
            };
            cc.instruct_buf
                .push(format!("%{id} = icmp {cmp_type} i32 {ltag} {rtag}"));
            if (rtype == ltype) || (rtype.is_numeric() && ltype.is_numeric()) {
                (format!("%{id}"), VariableType::Bool)
            } else {
                error(
                    format!(
                        "Invalid Comparison between types: ({}) and ({})",
                        ltype, rtype
                    ),
                    expr.loc.clone(),
                );
            }
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
            let right = compile_expr(cc, &u.right);
            match u.op {
                Op::Sub => {
                    assert!(false);
                    if right.1 == VariableType::UInt {
                        return (right.0, VariableType::Int);
                    } else {
                        return right;
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
            if right.1.is_numeric() {
                right
            } else {
                error(
                    format!("Invalid Operation ({}) for type ({})", u.op, right.1),
                    expr.loc.clone(),
                );
            }
        }
        ExprType::FunctionCall(fc) => match compile_function_call(cc, fc) {
            Ok(ftype) => todo!(),
            Err(msg) => error(msg, expr.loc.clone()),
        },
        ExprType::Ptr(e) => {
            todo!();
        }
        ExprType::DeRef(r) => {
            todo!();
        }
        ExprType::ArrayIndex(ai) => {
            let (mut tag, mut itype) = compile_expr(cc, &ai.indexer);
            let mut id = cc.instruct_buf.len();
            if tag.starts_with('%') {
                cc.instruct_buf.push(format!(
                    "%{id} = load {}, ptr {tag}, align {}",
                    itype.to_llvm_type(),
                    itype.size()
                ));
                tag = id.to_string();
                id += 1;
            }
            match itype.cast(&VariableType::Long) {
                Ok(t) => {
                    if itype.size() != 8 {
                        cc.instruct_buf
                            .push(format!("%{id} = sext {itype} {tag} to i64"));
                        tag = id.to_string();
                        itype = t;
                        id += 1;
                    }
                }
                Err(msg) => error(msg, ai.indexer.loc.clone()),
            }
            let Some(map) = get_vriable_map(cc,&ai.ident) else {
                error(format!("Undifined variable ({})",ai.ident.clone()),expr.loc.clone());
            };
            let code = format!(
                "%{id} = getelementptr inbounds {}, ptr {}, i32 0, {itype} {tag}",
                map.vtype.to_llvm_type(),
                map._ident
            );
            cc.instruct_buf.push(code);
            match map.vtype {
                VariableType::Array(t, _) => {
                    return (format!("%{id}"), t.as_ref().clone());
                }
                _ => {
                    error(
                        format!("Unable to index variable ({})", map._ident.clone()),
                        expr.loc.clone(),
                    );
                }
            }
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
