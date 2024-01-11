use crate::{
    codegen::{
        instructions::Instr,
        memory::MemAddr,
        mnmemonic::Mnemonic::*,
        register::Reg::{self, *},
    },
    error_handeling::error,
    mem, memq,
    parser::{
        expr::{CompareOp, Expr, ExprType, FunctionCall, Op},
        types::VariableType,
    },
};

use super::{
    function_args_register,
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
            // let mem_acss = format!(
            //     "{} [rbp-{}]",
            //     mem_word(&v_map.vtype),
            //     v_map.offset + v_map.vtype.size()
            // );
            let mem_acss = MemAddr::new_disp_s(v_map.vtype.item_size(), RBP, v_map.stack_offset());
            cc.codegen
                .push_instr(Instr::mov(Reg::AX_sized(&v_map.vtype), mem_acss));
            cc.codegen.push_instr(Instr::push(RAX));
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
            cc.codegen.push_instr(Instr::mov(RDX, mem!(RBP, v_map.stack_offset())));
            cc.codegen.push_instr(Instr::add(RDX, offset));
            cc.codegen.push_instr(
                Instr::mov(
                Reg::AX_sized(&actype),
                MemAddr::new_s(actype.item_size(), RDX),
            ));
            // cc.instruct_buf.push(asm!(
            //     "mov {}, {} [rdx]",
            //     rbs("a", &actype),
            //     mem_word(&actype)
            // ));
            cc.codegen.push_instr(Instr::push(RAX));
            actype
        }
        ExprType::Bool(b) => {
            cc.codegen.push_instr(Instr::push(*b as i32));
            VariableType::Bool
        }
        ExprType::Char(x) => {
            cc.codegen.push_instr(Instr::push(*x as i32));
            VariableType::Char
        }
        ExprType::Int(x) => {
            // push x
            cc.codegen.push_instr(Instr::push(*x));
            VariableType::Int
        }
        ExprType::Float(_) => {
            todo!()
            // cc.codegen.instr1(Push, format!("__float64__({f})"));
            // VariableType::Float
        }
        ExprType::Compare(c) => {
            let left_type = compile_expr(cc, c.left.as_ref());
            let right_type = compile_expr(cc, c.right.as_ref());
            cc.codegen.push_instr(Instr::mov(RCX, 0));
            cc.codegen.push_instr(Instr::mov(RDX, 1));
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
            cc.codegen.push_instr(Instr::pop(RBX));
            cc.codegen.push_instr(Instr::pop(RAX));
            cc.codegen
                .push_instr(Instr::cmp(Reg::AX_sized(&reg_type), Reg::BX_sized(&reg_type)));
            match c.op {
                CompareOp::Eq => {
                    cc.codegen.push_instr(Instr::cmove(RCX, RDX));
                }
                CompareOp::NotEq => {
                    cc.codegen.push_instr(Instr::cmovne(RCX, RDX));
                }
                CompareOp::Bigger => {
                    cc.codegen.push_instr(Instr::cmovg(RCX, RDX));
                }
                CompareOp::Smaller => {
                    cc.codegen.push_instr(Instr::cmovl(RCX, RDX));
                }
                CompareOp::BiggerEq => {
                    cc.codegen.push_instr(Instr::cmovge(RCX, RDX));
                }
                CompareOp::SmallerEq => {
                    cc.codegen.push_instr(Instr::cmovle(RCX, RDX));
                }
            }
            cc.codegen.push_instr(Instr::push(RCX));
            VariableType::Bool
        }
        ExprType::Binary(b) => {
            let left_type = compile_expr(cc, b.left.as_ref());
            let right_type = compile_expr(cc, b.right.as_ref());
            cc.codegen.push_instr(Instr::pop(RBX));
            cc.codegen.push_instr(Instr::pop(RAX));
            match b.op {
                Op::Plus => {
                    cc.codegen.push_instr(Instr::add(RAX, RBX));
                    cc.codegen.push_instr(Instr::push(RAX));
                }
                Op::Sub => {
                    cc.codegen.push_instr(Instr::sub(RAX, RBX));
                    cc.codegen.push_instr(Instr::push(RAX));
                }
                Op::Multi => {
                    cc.codegen.push_instr(Instr::imul(RAX, RBX));
                    cc.codegen.push_instr(Instr::push(RAX));
                }
                Op::Devide => {
                    cc.codegen.push_instr(Instr::Cqo);
                    cc.codegen.push_instr(Instr::idiv(RBX));
                    cc.codegen.push_instr(Instr::push(RAX));
                }
                Op::Mod => {
                    cc.codegen.push_instr(Instr::Cqo);
                    cc.codegen.push_instr(Instr::idiv(RBX));
                    cc.codegen.push_instr(Instr::push(RDX));
                }
                Op::Or => {
                    cc.codegen.push_instr(Instr::or(RAX, RBX));
                    cc.codegen.push_instr(Instr::push(RAX));
                }
                Op::And => {
                    cc.codegen.push_instr(Instr::and(RAX, RBX));
                    cc.codegen.push_instr(Instr::push(RAX));
                }
                Op::Lsh => {
                    cc.codegen.push_instr(Instr::mov(RCX, RBX));
                    cc.codegen.push_instr(Instr::sal(RAX, CL));
                    cc.codegen.push_instr(Instr::push(RAX));
                }
                Op::Rsh => {
                    cc.codegen.push_instr(Instr::mov(RCX, RBX));
                    cc.codegen.push_instr(Instr::sar(RAX, CL));
                    cc.codegen.push_instr(Instr::push(RAX));
                }
                Op::LogicalOr => {
                    cc.codegen.push_instr(Instr::or(RAX, RBX));
                    cc.codegen.push_instr(Instr::push(RAX));
                    return VariableType::Bool;
                }
                Op::LogicalAnd => {
                    cc.codegen.push_instr(Instr::and(RAX, RBX));
                    cc.codegen.push_instr(Instr::push(RAX));
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
            // assert!(false, "Not Implemented yet!");
            cc.codegen.asm_push(format!("data{id}"));
            cc.codegen.asm_push(format!("len{id}"));
            VariableType::String
        }
        ExprType::Unary(u) => {
            let right_type = compile_expr(cc, &u.right);
            cc.codegen.push_instr(Instr::pop(RAX));
            match u.op {
                Op::Sub => {
                    cc.codegen.push_instr(Instr::neg(RAX));
                    cc.codegen.push_instr(Instr::push(RAX));
                    if right_type == VariableType::UInt {
                        return VariableType::Int;
                    } else {
                        return right_type;
                    }
                }
                Op::Plus => {
                    cc.codegen.push_instr(Instr::push(RAX));
                }
                Op::Not => {
                    cc.codegen.push_instr(Instr::not(RAX));
                    cc.codegen.push_instr(Instr::push(RAX));
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
                VariableType::Array(_, _) => {
                    todo!("Changed!");
                }
                VariableType::Pointer => {
                    cc.codegen.push_instr(Instr::pop(RAX));
                    cc.codegen.push_instr(Instr::mov(RCX, memq!(RAX)));
                    cc.codegen.push_instr(Instr::push(RCX));
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
            cc.codegen.push_instr(Instr::pop(RBX));
            // cc.instruct_buf
            //     .push(asm!("mov rdx, [rbp-{}]", v_map.offset + v_map.vtype.size()));
            // cc.instruct_buf
            //     .push(asm!("imul rbx, {}", v_map.vtype.item_size()));
            // cc.instruct_buf.push(asm!("add rdx, rbx"));

            // let mem_acss = format!("{} [rdx]", mem_word(&v_map.vtype));
            // let mem_acss = format!(
            //     "{} [rbp-{}+rbx*{}]",
            //     mem_word(&v_map.vtype),
            //     v_map.offset + v_map.vtype.size(),
            //     v_map.vtype.item_size()
            // );
            let mem_acss = mem!(RBP, v_map.stack_offset(), RBX, v_map.vtype.item_size());
            let reg = Reg::AX_sized(&v_map.vtype);
            cc.codegen.push_instr(Instr::mov(reg, mem_acss));
            cc.codegen.push_instr(Instr::push(RAX));
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
                    //todo!("ReImplemnt");
                    //let mov_addr = format!("qword [rbp - {}]", v_map.offset + v_map.vtype.size());
                    //cc.codegen.mov(RAX, mov_addr);
                    //cc.codegen.push(RAX);

                    //cc.codegen.mov(RAX, RBP);
                    cc.codegen.push_instr(Instr::lea(RAX, mem!(RBP, v_map.stack_offset())));
                    cc.codegen.push_instr(Instr::push(RAX));
                }
                _ => {
                    cc.codegen.push_instr(Instr::mov(RAX, RBP));
                    cc.codegen
                        .push_instr(Instr::sub(RAX, v_map.offset + v_map.vtype.size()));
                    cc.codegen.push_instr(Instr::push(RAX));
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
                cc.codegen.push_instr(Instr::pop(RAX));
                cc.codegen.push_instr(Instr::pop(function_args_register(index)));
            }
            _ => {
                cc.codegen.push_instr(Instr::pop(function_args_register(index)));
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
    cc.codegen.push_instr(Instr::mov(RAX, 0));
    // assert!(false, "Not Implemented yet!");
    cc.codegen.call(fc.ident.clone());
    if fun.ret_type != VariableType::Void {
        cc.codegen.push_instr(Instr::push(RAX));
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
