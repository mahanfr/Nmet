use crate::{
    asm,
    error_handeling::error,
    parser::{
        assign::{Assign, AssignOp},
        expr::ExprType,
        stmt::{ElseBlock, IFStmt, Stmt, StmtType, WhileStmt}, types::VariableType,
    },
};

use super::{
    compile_block,
    expr::compile_expr,
    mem_word, rbs,
    variables::{find_variable, get_vriable_map, insert_variable, VariableMap},
    CompilerContext,
};

fn compile_if_stmt(cc: &mut CompilerContext, ifs: &IFStmt, exit_tag: usize) {
    let condition_type = compile_expr(cc, &ifs.condition);
    match VariableType::Bool.cast(&condition_type) {
        Ok(_) => (),
        Err(msg) => error(msg,ifs.condition.loc.clone())
    }
    let next_tag = match ifs.else_block.as_ref() {
        ElseBlock::None => exit_tag,
        _ => cc.instruct_buf.len(),
    };
    cc.instruct_buf.push(asm!("pop rax"));
    cc.instruct_buf.push(asm!("test rax, rax"));
    cc.instruct_buf.push(asm!("jz .L{}", next_tag));

    compile_block(cc, &ifs.then_block);
    match ifs.else_block.as_ref() {
        ElseBlock::None => {
            cc.instruct_buf.push(asm!(".L{}:", next_tag));
        }
        ElseBlock::Else(b) => {
            cc.instruct_buf.push(asm!("jmp .L{}", exit_tag));
            cc.instruct_buf.push(asm!(".L{}:", next_tag));
            compile_block(cc, b);
            cc.instruct_buf.push(asm!(".L{}:", exit_tag));
        }
        ElseBlock::Elif(iff) => {
            cc.instruct_buf.push(asm!("jmp .L{}", exit_tag));
            cc.instruct_buf.push(asm!(".L{}:", next_tag));
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
                    cc.instruct_buf.push(asm!("mov rax, 1"));
                    cc.instruct_buf.push(asm!("mov rdi, 1"));
                    cc.instruct_buf.push(asm!("pop rbx"));
                    cc.instruct_buf.push(asm!("pop rcx"));
                    cc.instruct_buf.push(asm!("mov rsi, rcx"));
                    cc.instruct_buf.push(asm!("mov rdx, rbx"));
                    cc.instruct_buf.push(asm!("syscall"));
                }
                _ => {
                    cc.instruct_buf.push(asm!("pop rdi"));
                    cc.instruct_buf.push(asm!("call print"));
                }
            }
        }
        StmtType::If(ifs) => {
            let exit_tag = cc.instruct_buf.len();
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
            cc.instruct_buf.push(asm!("pop rax"));
            cc.instruct_buf.push(asm!("leave"));
            cc.instruct_buf.push(asm!("ret"));
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
                    let Some(v_map) = find_variable(cc,ident.clone()) else {
                            return Err(format!(
                                "Could not find variable {} in this scope",
                                ident.clone()
                            ));
                        };
                    let mem_acss = format!(
                        "{} [rbp-{}]",
                        mem_word(&v_map.vtype),
                        v_map.offset + v_map.vtype.size()
                    );
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
        cc.instruct_buf.push(asm!("{}", final_instr));
    } else {
        cc.instruct_buf.push(asm!("{}", instr));
    }
    Ok(())
}

fn compile_while(cc: &mut CompilerContext, w_stmt: &WhileStmt) {
    let cond_tag = cc.instruct_buf.len();
    cc.instruct_buf.push(asm!("jmp .L{}", cond_tag));
    let block_tag = cond_tag + 1;
    cc.instruct_buf.push(asm!(".L{}:", block_tag));
    compile_block(cc, &w_stmt.block);
    cc.instruct_buf.push(asm!(".L{}:", cond_tag));
    // Jump after a compare
    let condition_type = compile_expr(cc, &w_stmt.condition);
    match VariableType::Bool.cast(&condition_type) {
        Ok(_)=> (),
        Err(msg) => error(msg,w_stmt.condition.loc.clone())
    }
    cc.instruct_buf.push(asm!("pop rax"));
    cc.instruct_buf.push(asm!("test rax, rax"));
    cc.instruct_buf.push(asm!("jnz .L{}", block_tag));
}

fn assgin_op(cc: &mut CompilerContext, op: &AssignOp, v_map: &VariableMap) {
    let mem_acss = if v_map.vtype.item_size() != v_map.vtype.size() {
        format!(
            "{} [rbp-{}+rbx*{}]",
            mem_word(&v_map.vtype),
            v_map.offset + v_map.vtype.size(),
            v_map.vtype.item_size()
        )
    } else {
        format!(
            "{} [rbp-{}]",
            mem_word(&v_map.vtype),
            v_map.offset + v_map.vtype.size()
        )
    };
    let reg = rbs("a", &v_map.vtype);
    cc.instruct_buf.push(asm!("pop rax"));
    match op {
        AssignOp::Eq => {
            cc.instruct_buf.push(asm!("mov {mem_acss},{reg}"));
        }
        AssignOp::PlusEq => {
            cc.instruct_buf.push(asm!("add {mem_acss},{reg}"));
        }
        AssignOp::SubEq => {
            cc.instruct_buf.push(asm!("sub {mem_acss},{reg}"));
        }
        AssignOp::MultiEq => {
            let b_reg = rbs("b", &v_map.vtype);
            cc.instruct_buf.push(asm!("mov {b_reg},{mem_acss}"));
            cc.instruct_buf.push(asm!("imul {reg},{b_reg}"));
            cc.instruct_buf.push(asm!("mov {mem_acss},{reg}"));
        }
        AssignOp::DevideEq => {
            let b_reg = rbs("b", &v_map.vtype);
            cc.instruct_buf.push(asm!("mov {b_reg},{reg}"));
            cc.instruct_buf.push(asm!("mov {reg},{mem_acss}"));
            cc.instruct_buf.push(asm!("cqo"));
            cc.instruct_buf.push(asm!("idiv rbx"));
            cc.instruct_buf.push(asm!("mov {mem_acss},{reg}"));
        }
        AssignOp::ModEq => {
            let b_reg = rbs("b", &v_map.vtype);
            cc.instruct_buf.push(asm!("mov {b_reg},{reg}"));
            cc.instruct_buf.push(asm!("mov {reg},{mem_acss}"));
            cc.instruct_buf.push(asm!("cqo"));
            cc.instruct_buf.push(asm!("idiv rbx"));
            let d_reg = rbs("d", &v_map.vtype);
            cc.instruct_buf.push(asm!("mov {mem_acss},{d_reg}"));
        }
    }
}

fn compile_assgin(cc: &mut CompilerContext, assign: &Assign) -> Result<(), String> {
    match &assign.left.etype {
        ExprType::Variable(v) => {
            let Some(v_map) = get_vriable_map(cc,v) else {
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
            let Some(v_map) = get_vriable_map(cc,&ai.ident) else {
                    return Err("Trying to access an Undifined variable".to_string());
                };
            if !v_map.is_mut {
                return Err("Error: Variable is not mutable. Did you forgot to define it with '=' insted of ':=' ?".to_string());
            }
            let right_type = compile_expr(cc, &assign.right);
            match &v_map.vtype {
                VariableType::Array(t, _) => {
                    match t.cast(&right_type) {
                        Ok(_) => (),
                        Err(msg) => return Err(msg),
                    }
                }
                _ => unreachable!()
            }
            compile_expr(cc, &ai.indexer);
            cc.instruct_buf.push(asm!("pop rbx"));
            assgin_op(cc, &assign.op, &v_map);
            Ok(())
        }
        _ => Err("Error: Expected a Variable type expression found Value".to_string()),
    }
}
