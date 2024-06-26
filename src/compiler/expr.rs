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
        register::Reg::*,
        utils::{mov_unknown_to_register, restore_last_temp_value, save_temp_value},
    },
    error_handeling::CompilationError,
    mem, memq,
    optim::{fold_binary_expr, fold_compare_expr, fold_unary_expr, ExprOpr},
    parser::{
        expr::{
            ArrayIndex, BinaryExpr, CompareExpr, CompareOp, Expr, ExprType, FunctionCall, Op,
            UnaryExpr,
        },
        types::VariableType,
    },
};

use super::{function_args_register, variables::get_vriable_map, CompilerContext};

/// This function is part of the Nmet compiler and programming language.
/// It takes expression (Expr) and a compiler context (CompilerContext)
/// as input and generates assembly code for the expression.
pub fn compile_expr(cc: &mut CompilerContext, expr: &Expr) -> Result<ExprOpr, CompilationError> {
    match &expr.etype {
        ExprType::Compare(c) => compile_compare_expr(cc, c),
        ExprType::Binary(b) => compile_binary_expr(cc, b),
        ExprType::Access(ident, ac) => compile_access(cc, ident, ac),
        ExprType::Unary(u) => compile_unaray_expr(cc, u),
        ExprType::FunctionCall(fc) => compile_function_call(cc, fc),
        ExprType::Ptr(e) => compile_ptr(cc, e),
        ExprType::DeRef(e) => compile_deref(cc, e),
        ExprType::ArrayIndex(ai) => compile_array_index(cc, ai),
        ExprType::Variable(v) => {
            let v_map = get_vriable_map(cc, v)?;
            let mem_acss = MemAddr::new_disp_s(v_map.vtype.item_size(), RBP, v_map.stack_offset());
            Ok(ExprOpr::new(mem_acss, v_map.vtype))
        }
        ExprType::Bool(b) => Ok(ExprOpr::new(*b as i32, VariableType::Bool)),
        ExprType::Char(x) => Ok(ExprOpr::new(*x as i32, VariableType::Char)),
        ExprType::Int(x) => Ok(ExprOpr::new(*x, VariableType::Int)),
        ExprType::String(str) => {
            let id = cc
                .codegen
                .add_data(str.as_bytes().to_vec(), VariableType::String);
            Ok(ExprOpr::new(Opr::Rela(id.to_owned()), VariableType::String))
        }
        ExprType::Float(_) => todo!(),
    }
}

pub fn compile_compare_expr(
    cc: &mut CompilerContext,
    cexpr: &CompareExpr,
) -> Result<ExprOpr, CompilationError> {
    // Compile the left Exprssion
    let left = compile_expr(cc, cexpr.left.as_ref())?;
    // Store in memory if register
    if left.is_temp() {
        save_temp_value(cc, left.value.clone());
    }
    // Compile the right Exprssion
    let right = compile_expr(cc, cexpr.right.as_ref())?;

    // Check for possiblity of optimization
    // NOTE: If valuse where literal noting has been added to the codegen
    if left.value.is_literal() && right.value.is_literal() {
        return fold_compare_expr(&left, &right, &cexpr.op);
    }

    // Move Result to RBX Register
    mov_unknown_to_register(cc, RBX, right.value.clone());
    // Retrive the first instr values to RAX
    if left.is_temp() {
        restore_last_temp_value(cc, RAX);
    } else {
        mov_unknown_to_register(cc, RAX, left.value.clone());
    }
    // Result of Compare instruction
    cc.codegen.instr2(Mov, RCX, 0);
    cc.codegen.instr2(Mov, RDX, 1);
    cc.codegen.instr2(Cmp, RAX, RBX);
    // set the result based on flag register
    let mnem = match cexpr.op {
        CompareOp::Eq => Cmove,
        CompareOp::NotEq => Cmovne,
        CompareOp::Bigger => Cmovg,
        CompareOp::Smaller => Cmovl,
        CompareOp::BiggerEq => Cmovge,
        CompareOp::SmallerEq => Cmovle,
    };
    cc.codegen.instr2(mnem, RCX, RDX);
    Ok(ExprOpr::new(RCX, VariableType::Bool))
}

fn compile_binary_expr(
    cc: &mut CompilerContext,
    bexpr: &BinaryExpr,
) -> Result<ExprOpr, CompilationError> {
    // Compile the left Exprssion
    let left = compile_expr(cc, bexpr.left.as_ref())?;
    // Store in memory if register
    if left.is_temp() {
        save_temp_value(cc, left.value.clone());
    }
    // Compile the right Exprssion
    let right = compile_expr(cc, bexpr.right.as_ref())?;

    // Check for possiblity of optimization
    // NOTE: If valuse where literal noting has been added to the codegen
    if left.value.is_literal() && right.value.is_literal() {
        return fold_binary_expr(&left, &right, &bexpr.op);
    }

    // Move Result of right to RBX Register
    mov_unknown_to_register(cc, RBX, right.value.clone());
    // Retrive the left expr result to RAX
    if left.is_temp() {
        restore_last_temp_value(cc, RAX);
    } else {
        mov_unknown_to_register(cc, RAX, left.value.clone());
    }
    match bexpr.op {
        Op::Plus => {
            cc.codegen.instr2(Add, RAX, RBX);
        }
        Op::Sub => {
            cc.codegen.instr2(Sub, RAX, RBX);
        }
        Op::Multi => {
            cc.codegen.instr2(Imul, RAX, RBX);
        }
        Op::Devide => {
            // Result of this operation is rax
            cc.codegen.instr0(Cqo);
            cc.codegen.instr1(Idiv, RBX);
        }
        Op::Mod => {
            cc.codegen.instr0(Cqo);
            cc.codegen.instr1(Idiv, RBX);
            cc.codegen.instr2(Mov, RAX, RDX);
        }
        Op::Or => {
            cc.codegen.instr2(Or, RAX, RBX);
        }
        Op::And => {
            cc.codegen.instr2(And, RAX, RBX);
        }
        Op::Lsh => {
            cc.codegen.instr2(Mov, RCX, RBX);
            cc.codegen.instr2(Sal, RAX, CL);
        }
        Op::Rsh => {
            cc.codegen.instr2(Mov, RCX, RBX);
            cc.codegen.instr2(Sar, RAX, CL);
        }
        Op::LogicalOr => {
            cc.codegen.instr2(Or, RAX, RBX);
            return Ok(ExprOpr::new(RAX, VariableType::Bool));
        }
        Op::LogicalAnd => {
            cc.codegen.instr2(And, RAX, RBX);
            return Ok(ExprOpr::new(RAX, VariableType::Bool));
        }
        Op::Not => {
            return Err(CompilationError::InValidBinaryOperation(
                bexpr.op.to_owned(),
                left.vtype.to_string(),
                right.vtype.to_string(),
            ));
        }
    }
    Ok(ExprOpr::new(RAX, left.vtype.cast(&right.vtype)?))
}

fn compile_array_index(
    cc: &mut CompilerContext,
    ai: &ArrayIndex,
) -> Result<ExprOpr, CompilationError> {
    let v_map = get_vriable_map(cc, &ai.ident)?;
    let indexer = compile_expr(cc, &ai.indexer)?;
    mov_unknown_to_register(cc, RBX, indexer.value);
    let mem_acss = MemAddr::new_sib_s(
        v_map.vtype.item_size(),
        RBP,
        v_map.stack_offset(),
        RBX,
        v_map.vtype.item_size(),
    );
    match v_map.vtype {
        VariableType::Array(t, _) => Ok(ExprOpr::new(mem_acss, t.as_ref().clone())),
        _ => unreachable!(),
    }
}

fn compile_unaray_expr(
    cc: &mut CompilerContext,
    uexpr: &UnaryExpr,
) -> Result<ExprOpr, CompilationError> {
    let left_eo = compile_expr(cc, &uexpr.right)?;
    if left_eo.value.is_literal() {
        return fold_unary_expr(&left_eo, &uexpr.op);
    }
    let new_type = match left_eo.vtype {
        VariableType::UInt => VariableType::Int,
        VariableType::ULong => VariableType::Long,
        VariableType::Char => VariableType::Int,
        _ => left_eo.vtype,
    };
    match uexpr.op {
        Op::Sub => {
            mov_unknown_to_register(cc, RAX, left_eo.value);
            cc.codegen.instr1(Neg, RAX);
            Ok(ExprOpr::new(RAX, new_type))
        }
        Op::Plus => Ok(ExprOpr::new(left_eo.value, new_type)),
        Op::Not => {
            mov_unknown_to_register(cc, RAX, left_eo.value);
            cc.codegen.instr1(Not, RAX);
            Ok(ExprOpr::new(RAX, new_type))
        }
        _ => {
            unreachable!();
        }
    }
}

fn compile_access(
    cc: &mut CompilerContext,
    ident: &str,
    expr: &Expr,
) -> Result<ExprOpr, CompilationError> {
    let v_map = get_vriable_map(cc, ident)?;
    let VariableType::Custom(struct_ident) = v_map.vtype.clone() else {
        return Err(CompilationError::UnexpectedType(v_map.vtype.to_string()));
    };
    let Some(struc) = cc.structs_map.get(&struct_ident) else {
        return Err(CompilationError::UndifiendStruct(struct_ident));
    };
    let mut offset: usize = 0;
    let mut actype = VariableType::Any;
    match &expr.etype {
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
    mov_unknown_to_register(cc, RDX, mem!(RBP, v_map.stack_offset()).into());
    cc.codegen.instr2(Add, RDX, offset);
    mov_unknown_to_register(cc, RAX, MemAddr::new_s(actype.item_size(), RDX).into());
    Ok(ExprOpr::new(RAX, actype))
}

fn compile_ptr(cc: &mut CompilerContext, expr: &Expr) -> Result<ExprOpr, CompilationError> {
    match &expr.etype {
        ExprType::Variable(v) => {
            let v_map = get_vriable_map(cc, v)?;
            match v_map.vtype {
                VariableType::Array(_, _) => {
                    cc.codegen.instr2(Lea, RAX, mem!(RBP, v_map.stack_offset()));
                    Ok(ExprOpr::new(RAX, VariableType::Pointer))
                }
                _ => {
                    cc.codegen.instr2(Mov, RAX, RBP);
                    cc.codegen
                        .instr2(Sub, RAX, v_map.offset + v_map.vtype.size());
                    Ok(ExprOpr::new(RAX, VariableType::Pointer))
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
) -> Result<ExprOpr, CompilationError> {
    let mut expr_list = Vec::new();
    for arg in fc.args.iter().rev() {
        let expr_op = compile_expr(cc, arg)?;
        if expr_op.is_temp() {
            save_temp_value(cc, expr_op.value.clone());
        }
        expr_list.push(expr_op);
    }
    for (i, item) in expr_list.iter().rev().enumerate() {
        if item.is_temp() {
            restore_last_temp_value(cc, function_args_register(i));
        } else {
            mov_unknown_to_register(cc, function_args_register(i), item.value.clone());
        }
    }
    let Some(fun) = cc.functions_map.get(&fc.ident) else {
        return Err(CompilationError::FunctionOutOfScope(fc.ident.clone()));
    };
    cc.codegen.instr2(Mov, RAX, 0);
    let ident = match cc.codegen.ffi_map.get(&fc.ident) {
        Some(id) => id.to_string(),
        None => String::new(),
    };
    if ident.is_empty() {
        cc.codegen.instr1(Call, Opr::Loc(fc.ident.clone()));
    } else {
        cc.codegen.instr1(Pop, RBP);
        cc.codegen.instr1(Call, Opr::Rela(ident.to_string()));
        cc.codegen.instr1(Push, RBP);
        cc.codegen.instr2(Mov, RBP, RSP);
    }
    if fun.ret_type != VariableType::Void {
        Ok(ExprOpr::new(RAX, fun.ret_type.clone()))
    } else {
        Ok(ExprOpr::new(0, fun.ret_type.clone()))
    }
}

fn compile_deref(cc: &mut CompilerContext, expr: &Expr) -> Result<ExprOpr, CompilationError> {
    let t = compile_expr(cc, expr)?;
    match t.vtype {
        VariableType::Array(_, _) => {
            todo!("Changed!");
        }
        VariableType::Pointer => {
            let Opr::R64(r) = t.value else {
                return Err(CompilationError::UnmatchingTypes(
                    VariableType::Pointer,
                    t.vtype,
                ));
            };
            mov_unknown_to_register(cc, RCX, memq!(r).into());
            Ok(ExprOpr::new(RCX, VariableType::Any))
        }
        _ => Err(CompilationError::UnmatchingTypes(
            VariableType::Pointer,
            t.vtype,
        )),
    }
}
