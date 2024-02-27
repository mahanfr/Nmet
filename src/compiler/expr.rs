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
    error_handeling::CompilationError,
    mem, memq,
    parser::{
        expr::{
            ArrayIndex, BinaryExpr, CompareExpr, CompareOp, Expr, ExprType, FunctionCall, Op,
            UnaryExpr,
        },
        types::VariableType,
    }, optim::{ExprOpr, optim_compare_expr},
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
        ExprType::Bool(b) => {
            Ok(ExprOpr::new(*b as i32, VariableType::Bool))
        }
        ExprType::Char(x) => {
            Ok(ExprOpr::new(*x as i32, VariableType::Char))
        }
        ExprType::Int(x) => {
            Ok(ExprOpr::new(*x, VariableType::Int))
        }
        ExprType::String(str) => {
            let id = cc
                .codegen
                .add_data(str.as_bytes().to_vec(), VariableType::String);
            cc.codegen.instr1(Push, Opr::Rela(id.clone()));
            cc.codegen.instr1(Push, str.len());
            Ok(ExprOpr::new(Opr::Rela(id.to_owned()), VariableType::String))
        }
        ExprType::Float(_) => todo!(),
    }
}

fn compile_compare_expr(
    cc: &mut CompilerContext,
    cexpr: &CompareExpr,
) -> Result<ExprOpr, CompilationError> {
    // Compile the left Exprssion
    let left = compile_expr(cc, cexpr.left.as_ref())?;
    // Store in memory if register
    if left.needs_stack() {
        cc.codegen.instr1(Push, left.value.clone());
    }
    // Compile the right Exprssion
    let right = compile_expr(cc, cexpr.right.as_ref())?;

    // Check for possiblity of optimization
    // NOTE: If valuse where literal noting has been added to the codegen
    if left.value.is_literal() && right.value.is_literal() {
        return optim_compare_expr(&left, &right, &cexpr.op);
    }

    // Move Result to RBX Register
    cc.codegen.instr2(Mov, RBX, right.value.clone());
    // Retrive the first instr values to RAX
    if left.needs_stack() {
        cc.codegen.instr1(Pop, RAX);
    } else {
        cc.codegen.instr2(Mov, RAX, left.value.clone());
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
    let left = compile_expr(cc, bexpr.left.as_ref())?;
    let right = compile_expr(cc, bexpr.right.as_ref())?;
    cc.codegen.instr1(Pop, RBX);
    cc.codegen.instr1(Pop, RAX);
    match bexpr.op {
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
            return Ok(ExprOpr::new(RAX, VariableType::Bool));
        }
        Op::LogicalAnd => {
            cc.codegen.instr2(And, RAX, RBX);
            cc.codegen.instr1(Push, RAX);
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
    if right.vtype.is_numeric() && left.vtype.is_numeric() {
        Ok(ExprOpr::new(RAX, left.vtype.cast(&right.vtype)?))
    } else {
        Err(CompilationError::InValidBinaryOperation(
            bexpr.op.to_owned(),
            left.vtype.to_string(),
            right.vtype.to_string(),
        ))
    }
}

fn compile_array_index(
    cc: &mut CompilerContext,
    ai: &ArrayIndex,
) -> Result<ExprOpr, CompilationError> {
    let v_map = get_vriable_map(cc, &ai.ident)?;
    compile_expr(cc, &ai.indexer)?;
    cc.codegen.instr1(Pop, RBX);
    let mem_acss = mem!(RBP, v_map.stack_offset(), RBX, v_map.vtype.item_size());
    let reg = Reg::AX_sized(&v_map.vtype);
    cc.codegen.instr2(Mov, reg, mem_acss);
    cc.codegen.instr1(Push, RAX);
    match v_map.vtype {
        VariableType::Array(t, _) => Ok(ExprOpr::new(RAX, t.as_ref().clone())),
        _ => unreachable!(),
    }
}

fn compile_unaray_expr(
    cc: &mut CompilerContext,
    uexpr: &UnaryExpr,
) -> Result<ExprOpr, CompilationError> {
    let right_eo = compile_expr(cc, &uexpr.right)?;
    cc.codegen.instr1(Pop, RAX);
    match uexpr.op {
        Op::Sub => {
            cc.codegen.instr1(Neg, RAX);
            cc.codegen.instr1(Push, RAX);
            if right_eo.vtype == VariableType::UInt {
                return Ok(ExprOpr::new(RAX, VariableType::Int));
            } else {
                return Ok(right_eo);
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
    if right_eo.vtype.is_numeric() || right_eo.vtype == VariableType::Bool {
        Ok(right_eo)
    } else {
        Err(CompilationError::InValidUnaryOperation(
            uexpr.op.to_owned(),
            right_eo.vtype.to_string(),
        ))
    }
}

fn compile_access(
    cc: &mut CompilerContext,
    ident: &str,
    expr: &Expr,
) -> Result<ExprOpr, CompilationError> {
    let v_map = get_vriable_map(cc, &ident)?;
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
    cc.codegen.instr2(Mov, RDX, mem!(RBP, v_map.stack_offset()));
    cc.codegen.instr2(Add, RDX, offset);
    cc.codegen.instr2(
        Mov,
        Reg::AX_sized(&actype),
        MemAddr::new_s(actype.item_size(), RDX),
    );
    cc.codegen.instr1(Push, RAX);
    Ok(ExprOpr::new(RAX, actype))
}

fn compile_ptr(cc: &mut CompilerContext, expr: &Expr) -> Result<ExprOpr, CompilationError> {
    match &expr.etype {
        ExprType::Variable(v) => {
            let v_map = get_vriable_map(cc, v)?;
            match v_map.vtype {
                VariableType::Array(_, _) => {
                    cc.codegen.instr2(Lea, RAX, mem!(RBP, v_map.stack_offset()));
                    cc.codegen.instr1(Push, RAX);
                    Ok(ExprOpr::new(RAX, VariableType::Pointer))
                }
                _ => {
                    cc.codegen.instr2(Mov, RAX, RBP);
                    cc.codegen
                        .instr2(Sub, RAX, v_map.offset + v_map.vtype.size());
                    cc.codegen.instr1(Push, RAX);
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
    cc.codegen.instr2(Mov, RBP, RSP);
    if fun.ret_type != VariableType::Void {
        cc.codegen.instr1(Push, RAX);
        Ok(ExprOpr::new(RAX, fun.ret_type.clone()))
    } else {
        Ok(ExprOpr::new(RBP, fun.ret_type.clone()))
    }
}

fn compile_deref(cc: &mut CompilerContext, expr: &Expr) -> Result<ExprOpr, CompilationError> {
    let t = compile_expr(cc, expr)?;
    match t.vtype {
        VariableType::Array(_, _) => {
            todo!("Changed!");
        }
        VariableType::Pointer => {
            cc.codegen.instr1(Pop, RAX);
            cc.codegen.instr2(Mov, RCX, memq!(RAX));
            cc.codegen.instr1(Push, RCX);
            Ok(ExprOpr::new(RCX, VariableType::Any))
        }
        _ => Err(CompilationError::UnmatchingTypes(
            VariableType::Pointer,
            t.vtype,
        )),
    }
}
