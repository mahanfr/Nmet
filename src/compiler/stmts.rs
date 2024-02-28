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
    codegen::{
        asm_parser::parse_asm, instructions::Opr, memory::MemAddr, mnemonic::Mnemonic::*,
        utils::{mov_unknown_to_register, save_temp_value, restore_last_temp_value}, register::Reg::*,
    },
    compiler::VariableMap,
    error_handeling::CompilationError,
    log_cerror, log_warn, mem,
    parser::{
        assign::{Assign, AssignOp},
        block::BlockType,
        expr::{Expr, ExprType},
        stmt::{ElseBlock, IFStmt, Stmt, StmtType, WhileStmt},
        types::VariableType,
    },
};

use super::{
    bif::Bif,
    block::compile_block,
    expr::compile_expr,
    variables::{get_vriable_map, insert_variable},
    CompilerContext,
};

fn compile_if_stmt(
    cc: &mut CompilerContext,
    ifs: &IFStmt,
    exit_tag: usize,
) -> Result<(), CompilationError> {
    let condition_eo = compile_expr(cc, &ifs.condition)?;
    let last_label = cc.last_main_label();
    VariableType::Bool.cast(&condition_eo.vtype)?;

    let next_tag = match ifs.else_block.as_ref() {
        ElseBlock::None => exit_tag,
        _ => cc.codegen.get_id(),
    };
    mov_unknown_to_register(cc, RAX, condition_eo.value);
    cc.codegen.instr2(Test, RAX, RAX);
    cc.codegen
        .instr1(Jz, Opr::Loc(format!("{last_label}.L{next_tag}")));

    compile_block(cc, &ifs.then_block, BlockType::Condition);
    match ifs.else_block.as_ref() {
        ElseBlock::None => {
            cc.codegen.set_lable(format!("{last_label}.L{next_tag}"));
            Ok(())
        }
        ElseBlock::Else(b) => {
            cc.codegen
                .instr1(Jmp, Opr::Loc(format!("{last_label}.L{exit_tag}")));
            cc.codegen.set_lable(format!("{last_label}.L{next_tag}"));
            compile_block(cc, b, BlockType::Condition);
            cc.codegen.set_lable(format!("{last_label}.L{exit_tag}"));
            Ok(())
        }
        ElseBlock::Elif(iff) => {
            cc.codegen
                .instr1(Jmp, Opr::Loc(format!("{last_label}.L{exit_tag}")));
            cc.codegen.set_lable(format!("{last_label}.L{next_tag}"));
            compile_if_stmt(cc, iff, exit_tag)?;
            Ok(())
        }
    }
}

fn compile_print(cc: &mut CompilerContext, expr: &Expr) -> Result<(), CompilationError> {
    let expr_opr = compile_expr(cc, expr)?;
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

pub fn compile_stmt(cc: &mut CompilerContext, stmt: &Stmt) -> Result<(), CompilationError> {
    match &stmt.stype {
        StmtType::VariableDecl(v) => insert_variable(cc, v),
        StmtType::Print(e) => compile_print(cc, e),
        StmtType::If(ifs) => {
            let exit_tag = cc.codegen.get_id();
            compile_if_stmt(cc, ifs, exit_tag)
        }
        StmtType::Assign(a) => compile_assgin(cc, a),
        StmtType::While(w) => compile_while(cc, w),
        StmtType::Expr(e) => match &e.etype {
            ExprType::FunctionCall(fc) => {
                let eo = compile_expr(cc, e)?;
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
            let ret_expr = compile_expr(cc, e)?;
            mov_unknown_to_register(cc, RAX, ret_expr.value);
            cc.codegen.instr0(Leave);
            cc.codegen.instr0(Ret);
            Ok(())
        }
        StmtType::InlineAsm(instructs) => {
            for instr in instructs {
                match compile_inline_asm(cc, instr) {
                    Ok(_) => (),
                    Err(e) => {
                        cc.error();
                        log_cerror!(stmt.loc, "{e}");
                    }
                }
            }
            Ok(())
        }
        _ => {
            todo!();
        }
    }
}

fn compile_inline_asm(cc: &mut CompilerContext, instr: &String) -> Result<(), CompilationError> {
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
                    let v_map = get_vriable_map(cc, &ident)?;
                    let mem_acss =
                        MemAddr::new_disp_s(v_map.vtype.item_size(), RBP, v_map.stack_offset())
                            .to_string();
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

fn compile_while(cc: &mut CompilerContext, w_stmt: &WhileStmt) -> Result<(), CompilationError> {
    let cond_tag = cc.codegen.get_id();
    cc.codegen.instr1(
        Jmp,
        Opr::Loc(format!("{}.L{cond_tag}", cc.last_main_label())),
    );
    let block_tag = cond_tag + 1;
    cc.codegen
        .set_lable(format!("{}.L{block_tag}", cc.last_main_label()));
    compile_block(cc, &w_stmt.block, BlockType::Loop((cond_tag, block_tag)));
    cc.codegen
        .set_lable(format!("{}.L{cond_tag}", cc.last_main_label()));
    // Jump after a compare
    let condition_eo = compile_expr(cc, &w_stmt.condition)?;
    VariableType::Bool.cast(&condition_eo.vtype)?;
    mov_unknown_to_register(cc, RAX, condition_eo.value);
    cc.codegen.instr2(Test, RAX, RAX);
    // assert!(false, "Not implemented yet!");
    // TODO: MAKE Sure this works!
    cc.codegen.instr1(
        Jne,
        Opr::Loc(format!("{}.L{block_tag}", cc.last_main_label())),
    );
    cc.codegen
        .set_lable(format!("{}.LE{block_tag}", cc.last_main_label()));
    Ok(())
}

fn assgin_op(
    cc: &mut CompilerContext,
    op: &AssignOp,
    opr: Opr,
    v_map: &VariableMap,
) -> Result<(), CompilationError> {
    let mut reg_size = v_map.vtype.item_size();
    let mem_acss = match &v_map.vtype {
        VariableType::Array(_, _) => MemAddr::new_sib_s(
            v_map.vtype.item_size(),
            RBP,
            v_map.stack_offset(),
            RBX,
            v_map.vtype.item_size(),
        ),
        VariableType::Custom(_) => {
            cc.codegen
                .instr2(Mov, RDX, mem!(RBP, -(v_map.offset as i32 + 8)));
            cc.codegen.instr2(Add, RDX, v_map.offset_inner);
            reg_size = v_map.vtype_inner.item_size();
            MemAddr::new_s(v_map.vtype_inner.item_size(), RDX)
        }
        _ => MemAddr::new_disp_s(v_map.vtype.item_size(), RBP, v_map.stack_offset()),
    };
    mov_unknown_to_register(cc, RAX, opr);
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
            mov_unknown_to_register(cc, RBX, mem_acss.into());
            cc.codegen.instr2(Imul, RAX, RBX);
            cc.codegen.instr2(Mov, mem_acss, RAX.convert(reg_size));
            Ok(())
        }
        AssignOp::DevideEq => {
            cc.codegen.instr2(Mov, RBX, RAX);
            mov_unknown_to_register(cc, RAX, mem_acss.into());
            cc.codegen.instr0(Cqo);
            cc.codegen.instr1(Idiv, RBX);
            cc.codegen.instr2(Mov, mem_acss, RAX.convert(reg_size));
            Ok(())
        }
        AssignOp::ModEq => {
            cc.codegen.instr2(Mov, RBX, RAX);
            mov_unknown_to_register(cc, RAX, mem_acss.into());
            cc.codegen.instr0(Cqo);
            cc.codegen.instr1(Idiv, RBX);
            cc.codegen.instr2(Mov, mem_acss, RDX.convert(reg_size));
            Ok(())
        }
    }
}

fn compile_assgin(cc: &mut CompilerContext, assign: &Assign) -> Result<(), CompilationError> {
    match &assign.left.etype {
        ExprType::Variable(v) => {
            let v_map = get_vriable_map(cc, v)?;
            if !v_map.is_mut {
                return Err(CompilationError::ImmutableVariable(v.to_owned()));
            }
            let right_eo = compile_expr(cc, &assign.right)?;
            v_map.vtype.cast(&right_eo.vtype)?;
            assgin_op(cc, &assign.op, right_eo.value, &v_map)?;
            Ok(())
        }
        ExprType::ArrayIndex(ai) => {
            let v_map = get_vriable_map(cc, &ai.ident)?;
            if !v_map.is_mut {
                return Err(CompilationError::ImmutableVariable(ai.ident.clone()));
            }
            let right_eo = compile_expr(cc, &assign.right)?;
            if right_eo.is_temp() {
                save_temp_value(cc, right_eo.value.clone());
            }
            let _ = match &v_map.vtype {
                VariableType::Array(t, _) => t.cast(&right_eo.vtype)?,
                _ => unreachable!(),
            };
            let indexer = compile_expr(cc, &ai.indexer)?;
            mov_unknown_to_register(cc, RBX, indexer.value);
            if right_eo.is_temp() {
                restore_last_temp_value(cc, RAX);
                assgin_op(cc, &assign.op, RAX.into(), &v_map)?;
            } else {
                assgin_op(cc, &assign.op, right_eo.value, &v_map)?;
            }
            Ok(())
        }
        ExprType::Access(ident, expr) => {
            let v_map = get_vriable_map(cc, ident)?;
            let VariableType::Custom(struct_ident) = v_map.vtype.clone() else {
                unreachable!();
            };
            let Some(struc) = cc.structs_map.get(&struct_ident) else {
                return Err(CompilationError::UndifiendStruct(struct_ident));
            };
            match &expr.etype {
                ExprType::Variable(i) => {
                    let mut vtype = VariableType::Any;
                    let mut offset_inner = 0;
                    for item in struc.items.iter() {
                        offset_inner += item.1.size();
                        if &item.0 == i {
                            vtype = item.1.clone();
                            break;
                        }
                    }
                    if vtype.is_any() {
                        return Err(CompilationError::UnknownRefrence);
                    }
                    let right_eo = compile_expr(cc, &assign.right)?;
                    vtype.cast(&right_eo.vtype)?;
                    let mut item_map = v_map.clone();
                    item_map.offset_inner = offset_inner;
                    item_map.vtype_inner = vtype;
                    assgin_op(cc, &assign.op, right_eo.value, &item_map)?;
                }
                ExprType::ArrayIndex(_) => todo!(),
                ExprType::Access(_, _) => todo!(),
                _ => {
                    return Err(CompilationError::UnexpectedType(struct_ident));
                }
            }

            Ok(())
        }
        _ => Err(CompilationError::UnexpectedType("Literal".to_owned())),
    }
}
