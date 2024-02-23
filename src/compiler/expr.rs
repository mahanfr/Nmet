/**********************************************************************************************
*
*   compiler/expr: compile expressions
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
    codegen::{
        instructions::Opr,
        memory::MemAddr,
        mnemonic::Mnemonic::*,
        register::Reg::{self, *},
    },
    error_handeling::{error, CompilationError},
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

pub fn compile_expr(
    cc: &mut CompilerContext,
    expr: &Expr,
) -> Result<VariableType, CompilationError> {
    // left = compile expr
    // right = compile expr
    // +
    match &expr.etype {
        ExprType::Variable(v) => {
            let Some(v_map) = get_vriable_map(cc, v) else {
                return Err(CompilationError::UndefinedVariable(v.to_owned()));
            };
            let mem_acss = MemAddr::new_disp_s(v_map.vtype.item_size(), RBP, v_map.stack_offset());
            cc.codegen
                .instr2(Mov, Reg::AX_sized(&v_map.vtype), mem_acss);
            cc.codegen.instr1(Push, RAX);
            Ok(v_map.vtype)
        }
        ExprType::Access(ident, ac) => {
            let Some(v_map) = get_vriable_map(cc, ident) else {
                return Err(CompilationError::UndefinedVariable(ident.to_owned()));
            };
            let VariableType::Custom(struct_ident) = v_map.vtype.clone() else {
                return Err(CompilationError::UnexpectedType(v_map.vtype.to_string()));
            };
            let Some(struc) = cc.structs_map.get(&struct_ident) else {
                return Err(CompilationError::UndifiendStruct(struct_ident));
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
                return Err(CompilationError::UnknownRefrence);
            }
            cc.codegen.instr2(Mov, RDX, mem!(RBP, v_map.stack_offset()));
            cc.codegen.instr2(Add, RDX, offset);
            cc.codegen.instr2(
                Mov,
                Reg::AX_sized(&actype),
                MemAddr::new_s(actype.item_size(), RDX),
            );
            cc.codegen.instr1(Push, RAX);
            Ok(actype)
        }
        ExprType::Bool(b) => {
            cc.codegen.instr1(Push, *b as i32);
            Ok(VariableType::Bool)
        }
        ExprType::Char(x) => {
            cc.codegen.instr1(Push, *x as i32);
            Ok(VariableType::Char)
        }
        ExprType::Int(x) => {
            // push x
            cc.codegen.instr1(Push, *x);
            Ok(VariableType::Int)
        }
        ExprType::Float(_) => {
            todo!()
            // cc.codegen.instr1(Push, format!("__float64__({f})"));
            // VariableType::Float
        }
        ExprType::Compare(c) => {
            let left_type = compile_expr(cc, c.left.as_ref())?;
            let right_type = compile_expr(cc, c.right.as_ref())?;
            cc.codegen.instr2(Mov, RCX, 0);
            cc.codegen.instr2(Mov, RDX, 1);
            let mut reg_type = left_type.clone();
            if right_type != left_type {
                if right_type.is_numeric() && left_type.is_numeric() {
                    if left_type.size() < right_type.size() {
                        reg_type = left_type;
                    } else {
                        reg_type = right_type;
                    }
                } else {
                    return Err(CompilationError::InvalidComparison(
                        left_type.to_string(),
                        right_type.to_string(),
                    ));
                }
            }
            // Make sure rbx is first so the order is correct
            cc.codegen.instr1(Pop, RBX);
            cc.codegen.instr1(Pop, RAX);
            cc.codegen
                .instr2(Cmp, Reg::AX_sized(&reg_type), Reg::BX_sized(&reg_type));
            match c.op {
                CompareOp::Eq => {
                    cc.codegen.instr2(Cmove, RCX, RDX);
                }
                CompareOp::NotEq => {
                    cc.codegen.instr2(Cmovne, RCX, RDX);
                }
                CompareOp::Bigger => {
                    cc.codegen.instr2(Cmovg, RCX, RDX);
                }
                CompareOp::Smaller => {
                    cc.codegen.instr2(Cmovl, RCX, RDX);
                }
                CompareOp::BiggerEq => {
                    cc.codegen.instr2(Cmovge, RCX, RDX);
                }
                CompareOp::SmallerEq => {
                    cc.codegen.instr2(Cmovle, RCX, RDX);
                }
            }
            cc.codegen.instr1(Push, RCX);
            Ok(VariableType::Bool)
        }
        ExprType::Binary(b) => {
            let left_type = compile_expr(cc, b.left.as_ref())?;
            let right_type = compile_expr(cc, b.right.as_ref())?;
            cc.codegen.instr1(Pop, RBX);
            cc.codegen.instr1(Pop, RAX);
            match b.op {
                Op::Plus => {
                    cc.codegen.instr2(Add, RAX, RBX);
                    cc.codegen.instr1(Push, RAX);
                }
                Op::Sub => {
                    cc.codegen.instr2(Sub, RAX, RBX);
                    cc.codegen.instr1(Push, RAX);
                }
                Op::Multi => {
                    cc.codegen.instr2(Imul, RAX, RBX);
                    cc.codegen.instr1(Push, RAX);
                }
                Op::Devide => {
                    cc.codegen.instr0(Cqo);
                    cc.codegen.instr1(Idiv, RBX);
                    cc.codegen.instr1(Push, RAX);
                }
                Op::Mod => {
                    cc.codegen.instr0(Cqo);
                    cc.codegen.instr1(Idiv, RBX);
                    cc.codegen.instr1(Push, RDX);
                }
                Op::Or => {
                    cc.codegen.instr2(Or, RAX, RBX);
                    cc.codegen.instr1(Push, RAX);
                }
                Op::And => {
                    cc.codegen.instr2(And, RAX, RBX);
                    cc.codegen.instr1(Push, RAX);
                }
                Op::Lsh => {
                    cc.codegen.instr2(Mov, RCX, RBX);
                    cc.codegen.instr2(Sal, RAX, CL);
                    cc.codegen.instr1(Push, RAX);
                }
                Op::Rsh => {
                    cc.codegen.instr2(Mov, RCX, RBX);
                    cc.codegen.instr2(Sar, RAX, CL);
                    cc.codegen.instr1(Push, RAX);
                }
                Op::LogicalOr => {
                    cc.codegen.instr2(Or, RAX, RBX);
                    cc.codegen.instr1(Push, RAX);
                    return Ok(VariableType::Bool);
                }
                Op::LogicalAnd => {
                    cc.codegen.instr2(And, RAX, RBX);
                    cc.codegen.instr1(Push, RAX);
                    return Ok(VariableType::Bool);
                }
                Op::Not => {
                    return Err(CompilationError::InValidBinaryOperation(
                        b.op.to_owned(),
                        left_type.to_string(),
                        right_type.to_string(),
                    ));
                }
            }
            if right_type.is_numeric() && left_type.is_numeric() {
                left_type.cast(&right_type)
            } else {
                Err(CompilationError::InValidBinaryOperation(
                    b.op.to_owned(),
                    left_type.to_string(),
                    right_type.to_string(),
                ))
            }
        }
        ExprType::String(str) => {
            let id = cc
                .codegen
                .add_data(str.as_bytes().to_vec(), VariableType::String);
            cc.codegen.instr1(Push, Opr::Rela(id));
            cc.codegen.instr1(Push, str.len());
            Ok(VariableType::String)
        }
        ExprType::Unary(u) => {
            let right_type = compile_expr(cc, &u.right)?;
            cc.codegen.instr1(Pop, RAX);
            match u.op {
                Op::Sub => {
                    cc.codegen.instr1(Neg, RAX);
                    cc.codegen.instr1(Push, RAX);
                    if right_type == VariableType::UInt {
                        return Ok(VariableType::Int);
                    } else {
                        return Ok(right_type);
                    }
                }
                Op::Plus => {
                    cc.codegen.instr1(Push, RAX);
                }
                Op::Not => {
                    cc.codegen.instr1(Not, RAX);
                    cc.codegen.instr1(Push, RAX);
                }
                _ => {
                    unreachable!();
                }
            }
            if right_type.is_numeric() || right_type == VariableType::Bool {
                Ok(right_type)
            } else {
                Err(CompilationError::InValidUnaryOperation(
                    u.op.to_owned(),
                    right_type.to_string(),
                ))
            }
        }
        ExprType::FunctionCall(fc) => compile_function_call(cc, fc),
        ExprType::Ptr(e) => {
            compile_ptr(cc, e)?;
            Ok(VariableType::Pointer)
        }
        ExprType::DeRef(r) => {
            let t = compile_expr(cc, r)?;
            match t {
                VariableType::Array(_, _) => {
                    todo!("Changed!");
                }
                VariableType::Pointer => {
                    cc.codegen.instr1(Pop, RAX);
                    cc.codegen.instr2(Mov, RCX, memq!(RAX));
                    cc.codegen.instr1(Push, RCX);
                }
                _ => {
                    error(format!("Expected a Pointer found ({t})"), expr.loc.clone());
                }
            }
            Ok(VariableType::Any)
        }
        ExprType::ArrayIndex(ai) => {
            let v_map = find_variable(cc, ai.ident.clone()).unwrap_or_else(|| {
                error(
                    format!("Trying to access an Undifined variable ({})", ai.ident),
                    expr.loc.clone(),
                );
            });
            compile_expr(cc, &ai.indexer)?;
            cc.codegen.instr1(Pop, RBX);
            let mem_acss = mem!(RBP, v_map.stack_offset(), RBX, v_map.vtype.item_size());
            let reg = Reg::AX_sized(&v_map.vtype);
            cc.codegen.instr2(Mov, reg, mem_acss);
            cc.codegen.instr1(Push, RAX);
            match v_map.vtype {
                VariableType::Array(t, _) => Ok(t.as_ref().clone()),
                _ => unreachable!(),
            }
        }
    }
}

fn compile_ptr(cc: &mut CompilerContext, expr: &Expr) -> Result<(), CompilationError> {
    match &expr.etype {
        ExprType::Variable(v) => {
            let Some(v_map) = get_vriable_map(cc, v) else {
                return Err(CompilationError::UndefinedVariable(v.to_owned()));
            };
            match v_map.vtype {
                VariableType::Array(_, _) => {
                    cc.codegen.instr2(Lea, RAX, mem!(RBP, v_map.stack_offset()));
                    cc.codegen.instr1(Push, RAX);
                    Ok(())
                }
                _ => {
                    cc.codegen.instr2(Mov, RAX, RBP);
                    cc.codegen
                        .instr2(Sub, RAX, v_map.offset + v_map.vtype.size());
                    cc.codegen.instr1(Push, RAX);
                    Ok(())
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
) -> Result<VariableType, CompilationError> {
    for (index, arg) in fc.args.iter().enumerate() {
        // let argv_type = expr_type(arg);
        compile_expr(cc, arg)?;
        match arg.etype {
            ExprType::String(_) => {
                cc.codegen.instr1(Pop, RAX);
                cc.codegen.instr1(Pop, function_args_register(index));
            }
            _ => {
                cc.codegen.instr1(Pop, function_args_register(index));
            }
        }
    }
    // TODO: Setup a unresolved function table
    let Some(fun) = cc.functions_map.get(&fc.ident) else {
        return Err(CompilationError::FunctionOutOfScope(fc.ident.clone()));
    };
    cc.codegen.instr2(Mov, RAX, 0);
    cc.codegen.instr1(Pop, RBP);
    match cc.codegen.ffi_map.get(&fc.ident) {
        Some(ident) => {
            cc.codegen.instr1(Call, Opr::Rela(ident.to_owned().clone()));
        }
        None => {
            cc.codegen.instr1(Call, Opr::Loc(fc.ident.clone()));
        }
    }
    cc.codegen.instr1(Push, RBP);
    if fun.ret_type != VariableType::Void {
        cc.codegen.instr1(Push, RAX);
    }
    Ok(fun.ret_type.clone())
}
