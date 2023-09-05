use crate::{
    compiler::VariableMap,
    error_handeling::error,
    parser::{
        assign::{Assign, AssignOp},
        expr::ExprType,
        stmt::{IFStmt, Stmt, StmtType, WhileStmt},
    },
};

use super::{expr::compile_expr, variables::insert_variable, CompilerContext};

fn compile_if_stmt(cc: &mut CompilerContext, ifs: &IFStmt) {
    todo!();
}

pub fn compile_stmt(cc: &mut CompilerContext, stmt: &Stmt) {
    match &stmt.stype {
        StmtType::VariableDecl(v) => {
            match insert_variable(cc, v) {
                Ok(_) => (),
                Err(msg) => error(msg, stmt.loc.clone()),
            }
        },
        StmtType::Print(e) => {
            todo!();
        }
        StmtType::If(ifs) => {
            compile_if_stmt(cc, ifs);
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
            todo!();
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
    todo!();
}

fn compile_while(cc: &mut CompilerContext, w_stmt: &WhileStmt) {
    todo!();
}

fn assgin_op(cc: &mut CompilerContext, op: &AssignOp, v_map: &VariableMap) {
    todo!();
}

fn compile_assgin(cc: &mut CompilerContext, assign: &Assign) -> Result<(), String> {
    match assign.left.etype {
        ExprType::Variable(_) | ExprType::ArrayIndex(_)=> (),
        _ => {
            error("Unsupported assgin operation",assign.left.loc.clone())
        }
    }
    let (etag,etype) = compile_expr(cc,&assign.right);
    let (vtag,vtype) = compile_expr(cc,&assign.left);
    let code = format!("store {} {}, {} {}, align {}",vtype.to_llvm_type(),vtag,etype.to_llvm_type(),etag,vtype.size());
    cc.instruct_buf.push(code); 
    Ok(())
}
