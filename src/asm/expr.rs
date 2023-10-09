use crate::{
    asm,
    error_handeling::error,
    parser::{
        expr::{CompareOp, Expr, ExprType, FunctionCall, Op},
        types::VariableType,
    },
};

use super::{
    function_args_register, mem_word, rbs,
    variables::{find_variable, get_vriable_map},
    CompilerContext,
};

pub fn compile_expr(cc: &mut CompilerContext, expr: &Expr) -> VariableType {
    // left = compile expr
    // right = compile expr
    // +
    match &expr.etype {
        ExprType::Variable(v) => {
            let Some(v_map) = get_vriable_map(cc, v) else {
                error("Trying to access an Undifined variable", expr.loc.clone());
            };
            let mem_acss = format!(
                "{} [rbp-{}]",
                mem_word(&v_map.vtype),
                v_map.offset + v_map.vtype.size()
            );
            cc.instruct_buf
                .push(asm!("mov {},{mem_acss}", rbs("a", &v_map.vtype)));
            cc.instruct_buf.push(asm!("push rax"));
            v_map.vtype
        }
        ExprType::Bool(b) => {
            cc.instruct_buf.push(asm!("push {b}"));
            VariableType::Bool
        }
        ExprType::Char(x) => {
            cc.instruct_buf.push(asm!("push {x}"));
            VariableType::Char
        }
        ExprType::Int(x) => {
            // push x
            cc.instruct_buf.push(asm!("push {x}"));
            VariableType::Int
        }
        ExprType::Float(f) => {
            cc.instruct_buf.push(asm!("push __float64__({f})"));
            VariableType::Float
        }
        ExprType::Compare(c) => {
            let left_type = compile_expr(cc, c.left.as_ref());
            let right_type = compile_expr(cc, c.right.as_ref());
            cc.instruct_buf.push(asm!("mov rcx, 0"));
            cc.instruct_buf.push(asm!("mov rdx, 1"));
            cc.instruct_buf.push(asm!("pop rbx"));
            cc.instruct_buf.push(asm!("pop rax"));
            match c.op {
                CompareOp::Eq => {
                    cc.instruct_buf.push(asm!("cmp rax, rbx"));
                    cc.instruct_buf.push(asm!("cmove rcx, rdx"));
                }
                CompareOp::NotEq => {
                    cc.instruct_buf.push(asm!("cmp rax, rbx"));
                    cc.instruct_buf.push(asm!("cmovne rcx, rdx"));
                }
                CompareOp::Bigger => {
                    cc.instruct_buf.push(asm!("cmp rax, rbx"));
                    cc.instruct_buf.push(asm!("cmovg rcx, rdx"));
                }
                CompareOp::Smaller => {
                    cc.instruct_buf.push(asm!("cmp rax, rbx"));
                    cc.instruct_buf.push(asm!("cmovl rcx, rdx"));
                }
                CompareOp::BiggerEq => {
                    cc.instruct_buf.push(asm!("cmp rax, rbx"));
                    cc.instruct_buf.push(asm!("cmovge rcx, rdx"));
                }
                CompareOp::SmallerEq => {
                    cc.instruct_buf.push(asm!("cmp rax, rbx"));
                    cc.instruct_buf.push(asm!("cmovle rcx, rdx"));
                }
            }
            cc.instruct_buf.push(asm!("push rcx"));
            if (right_type == left_type) || (right_type.is_numeric() && left_type.is_numeric()) {
                VariableType::Bool
            } else {
                error(
                    format!(
                        "Invalid Comparison between types: ({}) and ({})",
                        left_type, right_type
                    ),
                    expr.loc.clone(),
                );
            }
        }
        ExprType::Binary(b) => {
            let left_type = compile_expr(cc, b.left.as_ref());
            let right_type = compile_expr(cc, b.right.as_ref());
            cc.instruct_buf.push(asm!("pop rbx"));
            cc.instruct_buf.push(asm!("pop rax"));
            match b.op {
                Op::Plus => {
                    cc.instruct_buf.push(asm!("add rax, rbx"));
                    cc.instruct_buf.push(asm!("push rax"));
                }
                Op::Sub => {
                    cc.instruct_buf.push(asm!("sub rax, rbx"));
                    cc.instruct_buf.push(asm!("push rax"));
                }
                Op::Multi => {
                    cc.instruct_buf.push(asm!("imul rax, rbx"));
                    cc.instruct_buf.push(asm!("push rax"));
                }
                Op::Devide => {
                    cc.instruct_buf.push(asm!("cqo"));
                    cc.instruct_buf.push(asm!("idiv rbx"));
                    cc.instruct_buf.push(asm!("push rax"));
                }
                Op::Mod => {
                    cc.instruct_buf.push(asm!("cqo"));
                    cc.instruct_buf.push(asm!("idiv rbx"));
                    cc.instruct_buf.push(asm!("push rdx"));
                }
                Op::Or => {
                    cc.instruct_buf.push(asm!("or rax, rbx"));
                    cc.instruct_buf.push(asm!("push rax"));
                }
                Op::And => {
                    cc.instruct_buf.push(asm!("and rax, rbx"));
                    cc.instruct_buf.push(asm!("push rax"));
                }
                Op::Lsh => {
                    cc.instruct_buf.push(asm!("mov rcx, rbx"));
                    cc.instruct_buf.push(asm!("sal rax, cl"));
                    cc.instruct_buf.push(asm!("push rax"));
                }
                Op::Rsh => {
                    cc.instruct_buf.push(asm!("mov rcx, rbx"));
                    cc.instruct_buf.push(asm!("sar rax, cl"));
                    cc.instruct_buf.push(asm!("push rax"));
                }
                Op::LogicalOr => {
                    cc.instruct_buf.push(asm!("or rax, rbx"));
                    cc.instruct_buf.push(asm!("push rax"));
                    return VariableType::Bool;
                }
                Op::LogicalAnd => {
                    cc.instruct_buf.push(asm!("and rax, rbx"));
                    cc.instruct_buf.push(asm!("push rax"));
                    return VariableType::Bool;
                }
                Op::Not => {
                    panic!("Unvalid binary operation");
                }
            }
            if right_type.is_numeric() && left_type.is_numeric() {
                match left_type.cast(&right_type) {
                    Ok(t) => t,
                    Err(msg) => error(msg, expr.loc.clone()),
                }
            } else {
                error(
                    format!(
                        "Invalid Operation ({}) on non-numeric types: ({}) and ({})",
                        b.op, left_type, right_type
                    ),
                    expr.loc.clone(),
                );
            }
        }
        ExprType::String(str) => {
            let id = cc.data_buf.len();
            let data_array = asmfy_string(str);
            cc.data_buf.push(asm!("data{id} db {}", data_array));
            cc.data_buf.push(asm!("len{id} equ $ - data{id}"));
            cc.instruct_buf.push(asm!("push data{id}"));
            cc.instruct_buf.push(asm!("push len{id}"));
            VariableType::String
        }
        ExprType::Unary(u) => {
            let right_type = compile_expr(cc, &u.right);
            cc.instruct_buf.push(asm!("pop rax"));
            match u.op {
                Op::Sub => {
                    cc.instruct_buf.push(asm!("neg rax"));
                    cc.instruct_buf.push(asm!("push rax"));
                    if right_type == VariableType::UInt {
                        return VariableType::Int;
                    } else {
                        return right_type;
                    }
                }
                Op::Plus => {
                    cc.instruct_buf.push(asm!("push rax"));
                }
                Op::Not => {
                    cc.instruct_buf.push(asm!("not rax"));
                    cc.instruct_buf.push(asm!("push rax"));
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
            cc.instruct_buf.push(asm!("pop rax"));
            cc.instruct_buf.push(asm!("mov rcx, qword [rax]"));
            cc.instruct_buf.push(asm!("push rcx"));
            VariableType::Any
        }
        ExprType::ArrayIndex(ai) => {
            let v_map = find_variable(cc, ai.ident.clone()).unwrap_or_else(|| {
                error(
                    format!("Trying to access an Undifined variable ({})", ai.ident),
                    expr.loc.clone(),
                );
            });
            compile_expr(cc, &ai.indexer);
            cc.instruct_buf.push(asm!("pop rbx"));
            // TODO: Add Item size to v_map
            cc.instruct_buf
                .push(asm!("mov rdx, [rbp-{}]", v_map.offset + v_map.vtype.size()));
            cc.instruct_buf
                .push(asm!("imul rbx, {}", v_map.vtype.item_size()));
            cc.instruct_buf.push(asm!("add rdx, rbx"));
            let mem_acss = format!("{} [rdx]", mem_word(&v_map.vtype));
            let reg = rbs("a", &v_map.vtype);
            cc.instruct_buf.push(asm!("mov {reg},{mem_acss}"));
            cc.instruct_buf.push(asm!("push rax"));
            match v_map.vtype {
                VariableType::Array(t, _) => t.as_ref().clone(),
                _ => unreachable!(),
            }
        }
    }
}

fn compile_ptr(cc: &mut CompilerContext, expr: &Expr) {
    match &expr.etype {
        ExprType::Variable(v) => {
            let Some(v_map) = get_vriable_map(cc, v) else {
                error("Trying to access an Undifined variable", expr.loc.clone());
            };
            cc.instruct_buf.push(asm!("mov rax, rbp"));
            cc.instruct_buf
                .push(asm!("sub rax, {}", v_map.offset + v_map.vtype.size()));
            cc.instruct_buf.push(asm!("push rax"));
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
                cc.instruct_buf.push(asm!("pop rax"));
                cc.instruct_buf
                    .push(asm!("pop {}", function_args_register(index)));
            }
            _ => {
                cc.instruct_buf
                    .push(asm!("pop {}", function_args_register(index)));
            }
        }
    }
    // TODO: Setup a unresolved function table
    let Some(fun) = cc.functions_map.get(&fc.ident) else {
        return Err(format!(
            "Error: Function {} is not avaliable in this scope. {}",
            &fc.ident, "Make sure you are calling the correct function"
        ));
    };
    cc.instruct_buf.push(asm!("mov rax, 0"));
    cc.instruct_buf.push(asm!("call {}", fc.ident));
    if fun.ret_type != VariableType::Void {
        cc.instruct_buf.push(asm!("push rax"));
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
