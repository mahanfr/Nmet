/**********************************************************************************************
*
*   compiler/stmts: Compiler Statements
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
    assembler::{
        asm_parser::parse_asm,
        instructions::Opr,
        memory::MemAddr,
        mnemonic::Mnemonic::*,
        register::Reg::*,
        utils::{mov_unknown_to_register, restore_last_temp_value, save_temp_value},
    },
    error_handeling::CompilationError,
    log_cerror, log_warn, mem,
    parser::{
        assign::{Assign, AssignOp},
        block::Block,
        expr::{CompareExpr, CompareOp, Expr, ExprType},
        stmt::{ElseBlock, ForLoop, IFStmt, Stmt, StmtType, WhileStmt},
        types::VariableType,
    },
};

use super::{
    bif::Bif,
    block::compile_block,
    expr::{compile_compare_expr, compile_expr},
    variables::insert_variable,
    CompilerContext, VariableMapBase,
};

fn compile_if_stmt(
    cc: &mut CompilerContext,
    ifs: &IFStmt,
    exit_tag: String,
) -> Result<(), CompilationError> {
    let condition_eo = compile_expr(cc, &ifs.then_block, &ifs.condition)?;
    VariableType::Bool.cast(&condition_eo.vtype)?;

    let next_loc = match ifs.else_block.as_ref() {
        ElseBlock::None => exit_tag.clone(),
        _ => ifs.then_block.end_name(),
    };

    mov_unknown_to_register(cc, RAX, condition_eo.value);
    cc.codegen.instr2(Test, RAX, RAX);
    cc.codegen.instr1(Jz, Opr::Loc(next_loc.clone()));

    compile_block(cc, &ifs.then_block);
    match ifs.else_block.as_ref() {
        ElseBlock::None => {
            cc.codegen.set_lable(next_loc);
            Ok(())
        }
        ElseBlock::Else(b) => {
            cc.codegen.instr1(Jmp, Opr::Loc(exit_tag.clone()));
            cc.codegen.set_lable(next_loc);
            compile_block(cc, b);
            cc.codegen.set_lable(exit_tag);
            Ok(())
        }
        ElseBlock::Elif(iff) => {
            cc.codegen.instr1(Jmp, Opr::Loc(exit_tag.clone()));
            cc.codegen.set_lable(next_loc);
            compile_if_stmt(cc, iff, exit_tag)?;
            Ok(())
        }
    }
}

fn compile_print(
    cc: &mut CompilerContext,
    block: &Block,
    expr: &Expr,
) -> Result<(), CompilationError> {
    let expr_opr = compile_expr(cc, block, expr)?;
    match &expr.etype {
        ExprType::String(s) => {
            cc.codegen.instr2(Mov, RAX, 1);
            cc.codegen.instr2(Mov, RDI, 1);
            cc.codegen.instr2(Mov, RSI, expr_opr.value);
            cc.codegen.instr2(Mov, RDX, s.len());
            cc.codegen.instr0(Syscall);
        }
        _ => {
            cc.bif_set.insert(Bif::Print);
            mov_unknown_to_register(cc, RDI, expr_opr.value);
            cc.codegen.instr1(Call, Opr::Loc("print".to_string()));
        }
    }
    Ok(())
}

pub fn compile_stmt(
    cc: &mut CompilerContext,
    stmt: &Stmt,
    block: &Block,
) -> Result<(), CompilationError> {
    match &stmt.stype {
        StmtType::VariableDecl(v) => {
            insert_variable(cc, block, v, VariableMapBase::Stack(block.id.clone()))
        }
        StmtType::Print(e) => compile_print(cc, block, e),
        StmtType::If(ifs) => {
            let exit_tag = ifs.then_block.name_with_prefix("IFE");
            compile_if_stmt(cc, ifs, exit_tag)
        }
        StmtType::Assign(a) => compile_assgin(cc, block, a),
        StmtType::While(w) => compile_while(cc, w),
        StmtType::ForLoop(f) => compile_for_loop(cc, f),
        StmtType::Expr(e) => match &e.etype {
            ExprType::FunctionCall(fc) => {
                let eo = compile_expr(cc, block, e)?;
                if eo.vtype != VariableType::Void {
                    log_warn!(
                        "({}), Unused return value of function {}!",
                        stmt.loc,
                        fc.ident
                    );
                }
                Ok(())
            }
            _ => {
                log_warn!("({}) Expression with no effect ignored!", stmt.loc);
                Ok(())
            }
        },
        StmtType::Return(e) => {
            let ret_expr = compile_expr(cc, block, e)?;
            mov_unknown_to_register(cc, RAX, ret_expr.value);
            // cc.codegen.instr0(Leave);
            // cc.codegen.instr0(Ret);
            cc.codegen.instr1(Jmp, Opr::Loc(block.master_end_name()));
            Ok(())
        }
        StmtType::InlineAsm(instructs) => {
            for instr in instructs {
                match compile_inline_asm(cc, block, instr) {
                    Ok(_) => (),
                    Err(e) => {
                        cc.error();
                        log_cerror!(stmt.loc, "{e}");
                    }
                }
            }
            Ok(())
        }
        StmtType::Break => compile_break_coninue(cc, block, true),
        StmtType::Continue => compile_break_coninue(cc, block, false),
    }
}

fn compile_break_coninue(
    cc: &mut CompilerContext,
    block: &Block,
    is_break: bool,
) -> Result<(), CompilationError> {
    let exit_loc = if is_break {
        block.last_loop_end_name()?
    } else {
        block.last_loop_start_name()?
    };
    cc.codegen.instr1(Jmp, Opr::Loc(exit_loc));
    Ok(())
}

fn compile_inline_asm(
    cc: &mut CompilerContext,
    block: &Block,
    instr: &String,
) -> Result<(), CompilationError> {
    if instr.contains('%') {
        let mut final_instr = instr.clone();
        let chars = final_instr.chars().collect::<Vec<char>>();
        let mut index = 0;
        let is_empty = |index: usize| (index >= chars.len());
        while !is_empty(index) {
            if chars[index] == '%' {
                let mut ident = String::new();
                let first_index = index;
                index += 1;
                while !is_empty(index) && (chars[index].is_alphanumeric() || chars[index] == '_') {
                    ident.push(chars[index]);
                    index += 1;
                }
                if !ident.is_empty() {
                    let v_map = cc.variables_map.get(&ident, block)?;
                    let mem_acss = v_map.mem().to_string();
                    let mut temp = String::new();
                    temp.push_str(chars[0..(first_index)].iter().collect::<String>().as_str());
                    temp.push_str(mem_acss.as_str());
                    temp.push_str(chars[index..].iter().collect::<String>().as_str());
                    final_instr = temp;
                    index += mem_acss.len()
                } else {
                    return Err(CompilationError::InvalidInlineAsm(instr.to_string()));
                }
            } else {
                index += 1;
            }
        }
        cc.codegen.new_instr(parse_asm(final_instr));
    } else {
        cc.codegen.new_instr(parse_asm(instr.into()));
    }
    Ok(())
}

fn compile_for_loop(cc: &mut CompilerContext, for_stmt: &ForLoop) -> Result<(), CompilationError> {
    insert_variable(
        cc,
        &for_stmt.block,
        &for_stmt.iterator,
        VariableMapBase::Stack(for_stmt.block.id.clone()),
    )?;
    if !matches!(for_stmt.end_expr.etype, ExprType::Int(_)) {
        return Err(CompilationError::Err(format!(
            "Unsupported iterator type (must be type integer insted of ({:?}))",
            for_stmt.end_expr.etype
        )));
    }
    cc.codegen
        .instr1(Jmp, Opr::Loc(for_stmt.block.name_with_prefix("CND")));
    cc.codegen.set_lable(for_stmt.block.start_name());
    compile_block(cc, &for_stmt.block);

    let v_map = cc
        .variables_map
        .get(&for_stmt.iterator.ident, &for_stmt.block)?;
    let mem_acss = v_map.mem();
    cc.codegen.instr1(Inc, mem_acss);

    cc.codegen.set_lable(for_stmt.block.name_with_prefix("CND"));
    let cmp = CompareExpr {
        left: Box::new(Expr {
            loc: for_stmt.iterator.loc.clone(),
            etype: ExprType::Variable(for_stmt.iterator.ident.clone()),
        }),
        op: CompareOp::Smaller,
        right: Box::new(for_stmt.end_expr.to_owned()),
    };
    let condition_eo = compile_compare_expr(cc, &for_stmt.block, &cmp)?;
    VariableType::Bool.cast(&condition_eo.vtype)?;
    mov_unknown_to_register(cc, RAX, condition_eo.value);
    cc.codegen.instr2(Test, RAX, RAX);
    cc.codegen
        .instr1(Jne, Opr::Loc(for_stmt.block.start_name()));
    cc.codegen.set_lable(for_stmt.block.end_name());
    Ok(())
}

fn compile_while(cc: &mut CompilerContext, w_stmt: &WhileStmt) -> Result<(), CompilationError> {
    cc.codegen
        .instr1(Jmp, Opr::Loc(w_stmt.block.name_with_prefix("CND")));
    cc.codegen.set_lable(w_stmt.block.start_name());
    compile_block(cc, &w_stmt.block);
    cc.codegen.set_lable(w_stmt.block.name_with_prefix("CND"));
    // Jump after a compare
    let condition_eo = compile_expr(cc, &w_stmt.block, &w_stmt.condition)?;
    VariableType::Bool.cast(&condition_eo.vtype)?;
    mov_unknown_to_register(cc, RAX, condition_eo.value);
    cc.codegen.instr2(Test, RAX, RAX);
    cc.codegen.instr1(Jne, Opr::Loc(w_stmt.block.start_name()));
    cc.codegen.set_lable(w_stmt.block.end_name());
    Ok(())
}

fn assgin_op(
    cc: &mut CompilerContext,
    op: &AssignOp,
    opr: Opr,
    mem_acss: MemAddr,
) -> Result<(), CompilationError> {
    mov_unknown_to_register(cc, RAX, opr);
    let reg_size = mem_acss.size;
    match op {
        AssignOp::Eq => {
            cc.codegen.instr2(Mov, mem_acss, RAX.convert(reg_size));
            Ok(())
        }
        AssignOp::PlusEq => {
            cc.codegen.instr2(Add, mem_acss, RAX.convert(reg_size));
            Ok(())
        }
        AssignOp::SubEq => {
            cc.codegen.instr2(Sub, mem_acss, RAX.convert(reg_size));
            Ok(())
        }
        AssignOp::MultiEq => {
            mov_unknown_to_register(cc, RBX, mem_acss.clone().into());
            cc.codegen.instr2(Imul, RAX, RBX);
            cc.codegen.instr2(Mov, mem_acss, RAX.convert(reg_size));
            Ok(())
        }
        AssignOp::DevideEq => {
            cc.codegen.instr2(Mov, RBX, RAX);
            mov_unknown_to_register(cc, RAX, mem_acss.clone().into());
            cc.codegen.instr0(Cqo);
            cc.codegen.instr1(Idiv, RBX);
            cc.codegen.instr2(Mov, mem_acss, RAX.convert(reg_size));
            Ok(())
        }
        AssignOp::ModEq => {
            cc.codegen.instr2(Mov, RBX, RAX);
            mov_unknown_to_register(cc, RAX, mem_acss.clone().into());
            cc.codegen.instr0(Cqo);
            cc.codegen.instr1(Idiv, RBX);
            cc.codegen.instr2(Mov, mem_acss, RDX.convert(reg_size));
            Ok(())
        }
    }
}

fn compile_assgin(
    cc: &mut CompilerContext,
    block: &Block,
    assign: &Assign,
) -> Result<(), CompilationError> {
    match &assign.left.etype {
        ExprType::Variable(v) => {
            let v_map = cc.variables_map.get(v, block)?;
            if !v_map.is_mut {
                return Err(CompilationError::ImmutableVariable(v.to_owned()));
            }
            let right_eo = compile_expr(cc, block, &assign.right)?;
            v_map.vtype.cast(&right_eo.vtype)?;
            assgin_op(cc, &assign.op, right_eo.value, v_map.mem())?;
            Ok(())
        }
        ExprType::ArrayIndex(ai) => {
            let v_map = cc.variables_map.get(&ai.ident, block)?;
            if !v_map.is_mut {
                return Err(CompilationError::ImmutableVariable(ai.ident.clone()));
            }
            let right_eo = compile_expr(cc, block, &assign.right)?;
            if right_eo.is_temp() {
                save_temp_value(cc, right_eo.value.clone());
            }
            let _ = match &v_map.vtype {
                VariableType::Array(t, _) => t.cast(&right_eo.vtype)?,
                _ => unreachable!(),
            };
            let indexer = compile_expr(cc, block, &ai.indexer)?;
            mov_unknown_to_register(cc, RBX, indexer.value);
            let mem = MemAddr::new_sib_s(
                v_map.vtype.item_size(),
                RBP,
                v_map.offset,
                RBX,
                v_map.vtype.item_size(),
            );
            if right_eo.is_temp() {
                restore_last_temp_value(cc, RAX);
                assgin_op(cc, &assign.op, RAX.into(), mem)?;
            } else {
                assgin_op(cc, &assign.op, right_eo.value, mem)?;
            }
            Ok(())
        }
        ExprType::Access(ident, expr) => {
            let v_map = cc.variables_map.get(ident, block)?;
            let VariableType::Struct(struc) = v_map.vtype.clone() else {
                unreachable!();
            };
            match &expr.etype {
                ExprType::Variable(i) => {
                    let inner_var = struc.items.get(i).unwrap();
                    let right_eo = compile_expr(cc, block, &assign.right)?;
                    inner_var.vtype.cast(&right_eo.vtype)?;
                    cc.codegen.instr2(Mov, RDX, mem!(RBP, v_map.offset));
                    cc.codegen.instr2(Add, RDX, inner_var.offset);
                    let mem = MemAddr::new_s(inner_var.vtype.item_size(), RDX);
                    assgin_op(cc, &assign.op, right_eo.value, mem)?;
                }
                ExprType::ArrayIndex(_) => todo!(),
                ExprType::Access(_, _) => todo!(),
                _ => {
                    return Err(CompilationError::UnexpectedType(struc.ident));
                }
            }

            Ok(())
        }
        _ => Err(CompilationError::UnexpectedType("Literal".to_owned())),
    }
}
