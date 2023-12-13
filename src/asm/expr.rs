use crate::{
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
            cc.codegen.mov(rbs("a", &v_map.vtype), mem_acss);
            cc.codegen.push("rax");
            v_map.vtype
        }
        ExprType::Access(ident, ac) => {
            let Some(v_map) = get_vriable_map(cc, ident) else {
                error("Trying to access an Undifined variable", expr.loc.clone());
            };
            let VariableType::Custom(struct_ident) = v_map.vtype.clone() else {
                error("Trying to access an Undifined Structure", expr.loc.clone());
            };
            let Some(struc) = cc.structs_map.get(&struct_ident) else {
                error("Trying to access an Undifined Structure", expr.loc.clone());
            };
            let mut offset: usize = 0;
            let mut actype = VariableType::Any;
            match &ac.etype {
                ExprType::Variable(v) => {
                    for item in struc.items.iter() {
                        offset += item.1.size();
                        if &item.0 == v {
                            actype = item.1.clone();
                            break;
                        }
                    }
                }
                _ => todo!(),
            }
            if actype.is_any() {
                error(
                    "Trying to access unknown item from the list",
                    expr.loc.clone(),
                );
            }
            // cc.instruct_buf
            //     .push(asm!("mov rdx, [rbp-{}]", v_map.offset + v_map.vtype.size()));
            cc.codegen.mov(
                "rdx",
                format!("[rbp-{}]", v_map.offset + v_map.vtype.size()),
            );
            cc.codegen.add("rdx", offset);
            cc.codegen
                .mov(rbs("a", &actype), format!("{} [rdx]", mem_word(&actype)));
            // cc.instruct_buf.push(asm!(
            //     "mov {}, {} [rdx]",
            //     rbs("a", &actype),
            //     mem_word(&actype)
            // ));
            cc.codegen.push("rax");
            actype
        }
        ExprType::Bool(b) => {
            cc.codegen.push(b);
            VariableType::Bool
        }
        ExprType::Char(x) => {
            cc.codegen.push(x);
            VariableType::Char
        }
        ExprType::Int(x) => {
            // push x
            cc.codegen.push(x);
            VariableType::Int
        }
        ExprType::Float(f) => {
            cc.codegen.push(format!("__float64__({f})"));
            VariableType::Float
        }
        ExprType::Compare(c) => {
            let left_type = compile_expr(cc, c.left.as_ref());
            let right_type = compile_expr(cc, c.right.as_ref());
            cc.codegen.mov("rcx", 0);
            cc.codegen.mov("rdx", 1);
            let mut reg_type = left_type.clone();
            if right_type != left_type {
                if right_type.is_numeric() && left_type.is_numeric() {
                    if left_type.size() < right_type.size() {
                        reg_type = left_type;
                    } else {
                        reg_type = right_type;
                    }
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
            // Make sure rbx is first so the order is correct
            cc.codegen.pop("rbx");
            cc.codegen.pop("rax");
            cc.codegen.cmp(rbs("a", &reg_type), rbs("b", &reg_type));
            match c.op {
                CompareOp::Eq => {
                    cc.codegen.cmove("rcx", "rdx");
                }
                CompareOp::NotEq => {
                    cc.codegen.cmovne("rcx", "rdx");
                }
                CompareOp::Bigger => {
                    cc.codegen.cmovg("rcx", "rdx");
                }
                CompareOp::Smaller => {
                    cc.codegen.cmovl("rcx", "rdx");
                }
                CompareOp::BiggerEq => {
                    cc.codegen.cmovge("rcx", "rdx");
                }
                CompareOp::SmallerEq => {
                    cc.codegen.cmovle("rcx", "rdx");
                }
            }
            cc.codegen.push("rcx");
            VariableType::Bool
        }
        ExprType::Binary(b) => {
            let left_type = compile_expr(cc, b.left.as_ref());
            let right_type = compile_expr(cc, b.right.as_ref());
            cc.codegen.pop("rbx");
            cc.codegen.pop("rax");
            match b.op {
                Op::Plus => {
                    cc.codegen.add("rax", "rbx");
                    cc.codegen.push("rax");
                }
                Op::Sub => {
                    cc.codegen.sub("rax", "rbx");
                    cc.codegen.push("rax");
                }
                Op::Multi => {
                    cc.codegen.imul("rax", "rbx");
                    cc.codegen.push("rax");
                }
                Op::Devide => {
                    cc.codegen.cqo();
                    cc.codegen.idiv("rbx");
                    cc.codegen.push("rax");
                }
                Op::Mod => {
                    cc.codegen.cqo();
                    cc.codegen.idiv("rbx");
                    cc.codegen.push("rdx");
                }
                Op::Or => {
                    cc.codegen.or("rax", "rbx");
                    cc.codegen.push("rax");
                }
                Op::And => {
                    cc.codegen.and("rax", "rbx");
                    cc.codegen.push("rax");
                }
                Op::Lsh => {
                    cc.codegen.mov("rcx", "rbx");
                    cc.codegen.sal("rax", "cl");
                    cc.codegen.push("rax");
                }
                Op::Rsh => {
                    cc.codegen.mov("rcx", "rbx");
                    cc.codegen.sar("rax", "cl");
                    cc.codegen.push("rax");
                }
                Op::LogicalOr => {
                    cc.codegen.or("rax", "rbx");
                    cc.codegen.push("rax");
                    return VariableType::Bool;
                }
                Op::LogicalAnd => {
                    cc.codegen.and("rax", "rbx");
                    cc.codegen.push("rax");
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
            let data_array = asmfy_string(str);
            let id = cc.codegen.add_data_seg(data_array, 8);
            cc.codegen.push(format!("data{id}"));
            cc.codegen.push(format!("len{id}"));
            VariableType::String
        }
        ExprType::Unary(u) => {
            let right_type = compile_expr(cc, &u.right);
            cc.codegen.pop("rax");
            match u.op {
                Op::Sub => {
                    cc.codegen.neg("rax");
                    cc.codegen.push("rax");
                    if right_type == VariableType::UInt {
                        return VariableType::Int;
                    } else {
                        return right_type;
                    }
                }
                Op::Plus => {
                    cc.codegen.push("rax");
                }
                Op::Not => {
                    cc.codegen.not("rax");
                    cc.codegen.push("rax");
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
            match t {
                VariableType::Array(_, _) | VariableType::Pointer => {
                    cc.codegen.pop("rax");
                    cc.codegen.mov("rcx", "qword [rax]");
                    cc.codegen.push("rcx");
                }
                _ => {
                    error(format!("Expected a Pointer found ({t})"), expr.loc.clone());
                }
            }
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
            cc.codegen.pop("rbx");
            // cc.instruct_buf
            //     .push(asm!("mov rdx, [rbp-{}]", v_map.offset + v_map.vtype.size()));
            // cc.instruct_buf
            //     .push(asm!("imul rbx, {}", v_map.vtype.item_size()));
            // cc.instruct_buf.push(asm!("add rdx, rbx"));

            // let mem_acss = format!("{} [rdx]", mem_word(&v_map.vtype));
            let mem_acss = format!(
                "{} [rbp-{}+rbx*{}]",
                mem_word(&v_map.vtype),
                v_map.offset + v_map.vtype.size(),
                v_map.vtype.item_size()
            );
            let reg = rbs("a", &v_map.vtype);

            cc.codegen.mov(reg, mem_acss);
            cc.codegen.push("rax");
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
            match v_map.vtype {
                VariableType::Array(_, _) => {
                    let mov_addr = format!("qword [rbp - {}]", v_map.offset + v_map.vtype.size());
                    cc.codegen.mov("rax", mov_addr);
                    cc.codegen.push("rax");
                }
                _ => {
                    cc.codegen.mov("rax", "rbp");
                    cc.codegen.sub("rax", v_map.offset + v_map.vtype.size());
                    cc.codegen.push("rax");
                }
            }
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
                cc.codegen.pop("rax");
                cc.codegen.pop(function_args_register(index));
            }
            _ => {
                cc.codegen.pop(function_args_register(index));
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
    cc.codegen.mov("rax", 0);
    cc.codegen.call(fc.ident.clone());
    if fun.ret_type != VariableType::Void {
        cc.codegen.push("rax");
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
