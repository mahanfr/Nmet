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
        asm_parser::parse_asm,
        instructions::Opr,
        memory::MemAddr,
        mnemonic::Mnemonic::*,
        register::Reg::{self, *},
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
    compile_block,
    expr::compile_expr,
    variables::{find_variable, get_vriable_map, insert_variable},
    CompilerContext,
};

fn compile_if_stmt(
    cc: &mut CompilerContext,
    ifs: &IFStmt,
    exit_tag: usize,
) -> Result<(), CompilationError> {
    let condition_type = compile_expr(cc, &ifs.condition)?;
    let last_label = cc.last_main_label();
    VariableType::Bool.cast(&condition_type)?;

    let next_tag = match ifs.else_block.as_ref() {
        ElseBlock::None => exit_tag,
        _ => cc.codegen.get_id(),
    };
    cc.codegen.instr1(Pop, RAX);
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
    compile_expr(cc, expr)?;
    match expr.etype {
        ExprType::String(_) => {
            cc.codegen.instr2(Mov, RAX, 1);
            cc.codegen.instr2(Mov, RDI, 1);
            cc.codegen.instr1(Pop, RBX);
            cc.codegen.instr1(Pop, RCX);
            cc.codegen.instr2(Mov, RSI, RCX);
            cc.codegen.instr2(Mov, RDX, RBX);
            cc.codegen.instr0(Syscall);
        }
        _ => {
            cc.bif_set.insert(Bif::Print);
            cc.codegen.instr1(Pop, RDI);
            // assert!(false, "Not Implemented yet!");
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
        StmtType::Expr(e) => match e.etype {
            ExprType::FunctionCall(_) => {
                compile_expr(cc, e)?;
                Ok(())
            }
            _ => {
                log_warn!("Expression with no effect ignored!");
                Ok(())
            }
        },
        StmtType::Return(e) => {
            compile_expr(cc, e)?;
            cc.codegen.instr1(Pop, RAX);
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
                    let Some(v_map) = find_variable(cc, ident.clone()) else {
                        return Err(CompilationError::UndefinedVariable(ident.clone()));
                    };
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
    let condition_type = compile_expr(cc, &w_stmt.condition)?;
    VariableType::Bool.cast(&condition_type)?;
    cc.codegen.instr1(Pop, RAX);
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
    v_map: &VariableMap,
) -> Result<(), CompilationError> {
    let reg: Reg;
    let mem_acss = match &v_map.vtype {
        VariableType::Array(t, _) => {
            reg = Reg::AX_sized(t);
            MemAddr::new_sib_s(
                v_map.vtype.item_size(),
                RBP,
                v_map.stack_offset(),
                RBX,
                v_map.vtype.item_size(),
            )
        }
        VariableType::Custom(_) => {
            cc.codegen
                .instr2(Mov, RDX, mem!(RBP, -(v_map.offset as i32 + 8)));
            cc.codegen.instr2(Add, RDX, v_map.offset_inner);
            reg = Reg::AX_sized(&v_map.vtype_inner);
            // format!("{} [rdx]", mem_word(&v_map.vtype_inner))
            MemAddr::new_s(v_map.vtype_inner.item_size(), RDX)
        }
        _ => {
            reg = Reg::AX_sized(&v_map.vtype);
            MemAddr::new_disp_s(v_map.vtype.item_size(), RBP, v_map.stack_offset())
        }
    };
    cc.codegen.instr1(Pop, RAX);
    match op {
        AssignOp::Eq => {
            cc.codegen.instr2(Mov, mem_acss, reg);
            Ok(())
        }
        AssignOp::PlusEq => {
            cc.codegen.instr2(Add, mem_acss, reg);
            Ok(())
        }
        AssignOp::SubEq => {
            cc.codegen.instr2(Sub, mem_acss, reg);
            Ok(())
        }
        AssignOp::MultiEq => {
            let b_reg = Reg::BX_sized(&v_map.vtype);
            cc.codegen.instr2(Mov, b_reg, mem_acss);
            cc.codegen.instr2(Imul, reg, b_reg);
            cc.codegen.instr2(Mov, mem_acss, reg);
            Ok(())
        }
        AssignOp::DevideEq => {
            let b_reg = Reg::BX_sized(&v_map.vtype);
            cc.codegen.instr2(Mov, b_reg, reg);
            cc.codegen.instr2(Mov, reg, mem_acss);
            cc.codegen.instr0(Cqo);
            cc.codegen.instr1(Idiv, RBX);
            cc.codegen.instr2(Mov, mem_acss, reg);
            Ok(())
        }
        AssignOp::ModEq => {
            let b_reg = Reg::BX_sized(&v_map.vtype);
            cc.codegen.instr2(Mov, b_reg, reg);
            cc.codegen.instr2(Mov, reg, mem_acss);
            cc.codegen.instr0(Cqo);
            cc.codegen.instr1(Idiv, RBX);
            let d_reg = Reg::DX_sized(&v_map.vtype);
            cc.codegen.instr2(Mov, mem_acss, d_reg);
            Ok(())
        }
    }
}

fn compile_assgin(cc: &mut CompilerContext, assign: &Assign) -> Result<(), CompilationError> {
    match &assign.left.etype {
        ExprType::Variable(v) => {
            let Some(v_map) = get_vriable_map(cc, v) else {
                return Err(CompilationError::UndefinedVariable(v.to_owned()));
            };
            if !v_map.is_mut {
                return Err(CompilationError::ImmutableVariable(v.to_owned()));
            }
            let right_type = compile_expr(cc, &assign.right)?;
            v_map.vtype.cast(&right_type)?;
            assgin_op(cc, &assign.op, &v_map)?;
            Ok(())
        }
        ExprType::ArrayIndex(ai) => {
            let Some(v_map) = get_vriable_map(cc, &ai.ident) else {
                return Err(CompilationError::UndefinedVariable(ai.ident.clone()));
            };
            if !v_map.is_mut {
                return Err(CompilationError::ImmutableVariable(ai.ident.clone()));
            }
            let right_type = compile_expr(cc, &assign.right)?;
            let _ = match &v_map.vtype {
                VariableType::Array(t, _) => t.cast(&right_type)?,
                _ => unreachable!(),
            };
            compile_expr(cc, &ai.indexer)?;
            cc.codegen.instr1(Pop, RBX);
            assgin_op(cc, &assign.op, &v_map)?;
            Ok(())
        }
        ExprType::Access(ident, expr) => {
            let Some(v_map) = get_vriable_map(cc, ident) else {
                return Err(CompilationError::UndefinedVariable(ident.to_owned()));
            };
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
                    let right_type = compile_expr(cc, &assign.right)?;
                    vtype.cast(&right_type)?;
                    let mut item_map = v_map.clone();
                    item_map.offset_inner = offset_inner;
                    item_map.vtype_inner = vtype;
                    assgin_op(cc, &assign.op, &item_map)?;
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
