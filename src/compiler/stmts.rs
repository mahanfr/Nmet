use crate::{
    codegen::{
        asm_parser::parse_asm,
        memory::Mem,
        mnmemonic::Mnemonic::*,
        register::Reg::{self, *},
    },
    compiler::VariableMap,
    error_handeling::error,
    parser::{
        assign::{Assign, AssignOp},
        block::BlockType,
        expr::ExprType,
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

fn compile_if_stmt(cc: &mut CompilerContext, ifs: &IFStmt, exit_tag: usize) {
    let condition_type = compile_expr(cc, &ifs.condition);
    match VariableType::Bool.cast(&condition_type) {
        Ok(_) => (),
        Err(msg) => error(msg, ifs.condition.loc.clone()),
    }

    let next_tag = match ifs.else_block.as_ref() {
        ElseBlock::None => exit_tag,
        _ => cc.codegen.get_id(),
    };
    cc.codegen.instr1(Pop, RAX);
    cc.codegen.instr2(Test, RAX, RAX);
    cc.codegen.instr1(Jz, format!(".L{next_tag}"));

    compile_block(cc, &ifs.then_block, BlockType::Condition);
    match ifs.else_block.as_ref() {
        ElseBlock::None => {
            cc.codegen.set_lable(format!(".L{next_tag}"));
        }
        ElseBlock::Else(b) => {
            cc.codegen.instr1(Jmp, format!(".L{exit_tag}"));
            cc.codegen.set_lable(format!(".L{next_tag}"));
            compile_block(cc, b, BlockType::Condition);
            cc.codegen.set_lable(format!(".L{exit_tag}"));
        }
        ElseBlock::Elif(iff) => {
            cc.codegen.instr1(Jmp, format!(".L{exit_tag}"));
            cc.codegen.set_lable(format!(".L{next_tag}"));
            compile_if_stmt(cc, iff, exit_tag);
        }
    }
}

pub fn compile_stmt(cc: &mut CompilerContext, stmt: &Stmt) {
    match &stmt.stype {
        StmtType::VariableDecl(v) => match insert_variable(cc, v) {
            Ok(_) => (),
            Err(msg) => error(msg, stmt.loc.clone()),
        },
        StmtType::Print(e) => {
            compile_expr(cc, e);
            match e.etype {
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
                    cc.codegen.instr1(Call, "print");
                }
            }
        }
        StmtType::If(ifs) => {
            let exit_tag = cc.codegen.get_id();
            compile_if_stmt(cc, ifs, exit_tag);
        }
        StmtType::Assign(a) => match compile_assgin(cc, a) {
            Ok(_) => (),
            Err(msg) => error(msg, stmt.loc.clone()),
        },
        StmtType::While(w) => {
            compile_while(cc, w);
        }
        StmtType::Expr(e) => match e.etype {
            ExprType::FunctionCall(_) => {
                compile_expr(cc, e);
            }
            _ => {
                println!("Warning: Expression with no effect ignored!");
            }
        },
        StmtType::Return(e) => {
            compile_expr(cc, e);
            cc.codegen.instr1(Pop, RAX);
            cc.codegen.instr0(Leave);
            cc.codegen.instr0(Ret);
        }
        StmtType::InlineAsm(instructs) => {
            for instr in instructs {
                match compile_inline_asm(cc, instr) {
                    Ok(_) => (),
                    Err(msg) => error(msg, stmt.loc.clone()),
                }
            }
        }
        _ => {
            todo!();
        }
    }
}

fn compile_inline_asm(cc: &mut CompilerContext, instr: &String) -> Result<(), String> {
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
                        return Err(format!(
                            "Could not find variable {} in this scope",
                            ident.clone()
                        ));
                    };
                    // let mem_acss = format!(
                    //     "{} [rbp-{}]",
                    //     mem_word(&v_map.vtype),
                    //     v_map.offset + v_map.vtype.size()
                    // );
                    let mem_acss =
                        Mem::dyn_sized(&v_map.vtype, RBP - v_map.stack_offset()).to_string();
                    let mut temp = String::new();
                    temp.push_str(chars[0..(first_index)].iter().collect::<String>().as_str());
                    temp.push_str(mem_acss.as_str());
                    temp.push_str(chars[index..].iter().collect::<String>().as_str());
                    final_instr = temp;
                    index += mem_acss.len()
                } else {
                    return Err("Invalid Identifier for Inline Asm".to_string());
                }
            } else {
                index += 1;
            }
        }
        cc.codegen.insert_raw(parse_asm(final_instr));
    } else {
        cc.codegen.insert_raw(parse_asm(instr.into()));
    }
    Ok(())
}

fn compile_while(cc: &mut CompilerContext, w_stmt: &WhileStmt) {
    let cond_tag = cc.codegen.get_id();
    cc.codegen.instr1(Jmp, format!(".L{cond_tag}"));
    let block_tag = cond_tag + 1;
    cc.codegen.set_lable(format!(".L{block_tag}"));
    compile_block(cc, &w_stmt.block, BlockType::Loop((cond_tag, block_tag)));
    cc.codegen.set_lable(format!(".L{cond_tag}"));
    // Jump after a compare
    let condition_type = compile_expr(cc, &w_stmt.condition);
    match VariableType::Bool.cast(&condition_type) {
        Ok(_) => (),
        Err(msg) => error(msg, w_stmt.condition.loc.clone()),
    }
    cc.codegen.instr1(Pop, RAX);
    cc.codegen.instr2(Test, RAX, RAX);
    cc.codegen.instr1(Jnz, format!(".L{block_tag}"));
    cc.codegen.set_lable(format!(".LE{block_tag}"));
}

fn assgin_op(cc: &mut CompilerContext, op: &AssignOp, v_map: &VariableMap) {
    let reg: Reg;
    let mem_acss = match &v_map.vtype {
        VariableType::Array(t, _) => {
            // cc.instruct_buf
            //     .push(asm!("mov rdx, [rbp-{}]", v_map.offset + v_map.vtype.size()));
            // cc.instruct_buf
            //     .push(asm!("imul rbx, {}", v_map.vtype.item_size()));
            // cc.instruct_buf.push(asm!("add rdx, rbx"));
            // format!("{} [rdx]", mem_word(&v_map.vtype))
            reg = Reg::AX_sized(t);
            // format!(
            //     "{} [rbp-{}+rbx*{}]",
            //     mem_word(&v_map.vtype),
            //     v_map.offset + v_map.vtype.size(),
            //     v_map.vtype.item_size()
            // )
            Mem::dyn_sized(
                &v_map.vtype,
                RBP - v_map.stack_offset() + RBX * v_map.vtype.item_size(),
            )
        }
        VariableType::Custom(_) => {
            cc.codegen
                .instr2(Mov, RDX, Mem::U(RBP - (v_map.offset + 8)));
            cc.codegen.instr2(Add, RDX, v_map.offset_inner);
            reg = Reg::AX_sized(&v_map.vtype_inner);
            // format!("{} [rdx]", mem_word(&v_map.vtype_inner))
            Mem::dyn_sized(&v_map.vtype_inner, RDX.into())
        }
        _ => {
            reg = Reg::AX_sized(&v_map.vtype);
            // format!(
            //     "{} [rbp-{}]",
            //     mem_word(&v_map.vtype),
            //     v_map.offset + v_map.vtype.size()
            // )
            Mem::dyn_sized(&v_map.vtype, RBP - v_map.stack_offset())
        }
    };
    cc.codegen.instr1(Pop, RAX);
    match op {
        AssignOp::Eq => {
            cc.codegen.instr2(Mov, mem_acss, reg);
        }
        AssignOp::PlusEq => {
            cc.codegen.instr2(Add, mem_acss, reg);
        }
        AssignOp::SubEq => {
            cc.codegen.instr2(Sub, mem_acss, reg);
        }
        AssignOp::MultiEq => {
            let b_reg = Reg::BX_sized(&v_map.vtype);
            cc.codegen.instr2(Mov, b_reg, &mem_acss);
            cc.codegen.instr2(Imul, reg, b_reg);
            cc.codegen.instr2(Mov, &mem_acss, reg);
        }
        AssignOp::DevideEq => {
            let b_reg = Reg::BX_sized(&v_map.vtype);
            cc.codegen.instr2(Mov, b_reg, reg);
            cc.codegen.instr2(Mov, reg, &mem_acss);
            cc.codegen.instr0(Cqo);
            cc.codegen.instr1(Idiv, RBX);
            cc.codegen.instr2(Mov, &mem_acss, reg);
        }
        AssignOp::ModEq => {
            let b_reg = Reg::BX_sized(&v_map.vtype);
            cc.codegen.instr2(Mov, b_reg, reg);
            cc.codegen.instr2(Mov, reg, &mem_acss);
            cc.codegen.instr0(Cqo);
            cc.codegen.instr1(Idiv, RBX);
            let d_reg = Reg::DX_sized(&v_map.vtype);
            cc.codegen.instr2(Mov, &mem_acss, d_reg);
        }
    }
}

fn compile_assgin(cc: &mut CompilerContext, assign: &Assign) -> Result<(), String> {
    match &assign.left.etype {
        ExprType::Variable(v) => {
            let Some(v_map) = get_vriable_map(cc, v) else {
                return Err("Trying to access an Undifined variable".to_string());
            };
            if !v_map.is_mut {
                return Err("Error: Variable is not mutable. Did you forgot to define it with '=' insted of ':=' ?".to_string());
            }
            let right_type = compile_expr(cc, &assign.right);
            match v_map.vtype.cast(&right_type) {
                Ok(_) => (),
                Err(msg) => return Err(msg),
            }
            assgin_op(cc, &assign.op, &v_map);
            Ok(())
        }
        ExprType::ArrayIndex(ai) => {
            let Some(v_map) = get_vriable_map(cc, &ai.ident) else {
                return Err("Trying to access an Undifined variable".to_string());
            };
            if !v_map.is_mut {
                return Err("Error: Variable is not mutable. Did you forgot to define it with '=' insted of ':=' ?".to_string());
            }
            let right_type = compile_expr(cc, &assign.right);
            match &v_map.vtype {
                VariableType::Array(t, _) => match t.cast(&right_type) {
                    Ok(_) => (),
                    Err(msg) => return Err(msg),
                },
                _ => unreachable!(),
            }
            compile_expr(cc, &ai.indexer);
            cc.codegen.instr1(Pop, RBX);
            assgin_op(cc, &assign.op, &v_map);
            Ok(())
        }
        ExprType::Access(ident, expr) => {
            let Some(v_map) = get_vriable_map(cc, ident) else {
                return Err("Trying to access an Undifined variable".to_string());
            };
            let VariableType::Custom(struct_ident) = v_map.vtype.clone() else {
                unreachable!();
            };
            let Some(struc) = cc.structs_map.get(&struct_ident) else {
                return Err("Structure type is not defined".to_string());
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
                        return Err("Item dose not exist in this struct!".into());
                    }
                    let right_type = compile_expr(cc, &assign.right);
                    match vtype.cast(&right_type) {
                        Ok(_) => (),
                        Err(msg) => return Err(msg),
                    }
                    let mut item_map = v_map.clone();
                    item_map.offset_inner = offset_inner;
                    item_map.vtype_inner = vtype;
                    assgin_op(cc, &assign.op, &item_map);
                }
                ExprType::ArrayIndex(_) => todo!(),
                ExprType::Access(_, _) => todo!(),
                _ => {
                    return Err("Unexpected Type for structure".to_string());
                }
            }

            Ok(())
        }
        _ => Err("Error: Expected a Variable type expression found Value".to_string()),
    }
}
