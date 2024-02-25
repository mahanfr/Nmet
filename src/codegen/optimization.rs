use std::fmt::Display;

use crate::{
    error_handeling::CompilationError,
    parser::{expr::Op, types::VariableType}, compiler::CompilerContext,
};

use super::{instructions::{Opr, Instr}, mnemonic::Mnemonic};

#[derive(Debug, Clone, PartialEq)]
pub struct ExprOpr {
    pub value: Opr,
    pub vtype: VariableType,
    pub instrs: Vec<Instr>
}

impl ExprOpr {
    pub fn new(value: impl Into<Opr>, vtype: VariableType) -> Self {
        Self {
            value: value.into(),
            instrs: Vec::new(),
            vtype,
        }
    }

    pub fn instr2(&mut self, mnemonic: Mnemonic, opr1: impl Into<Opr>, opr2: impl Into<Opr>) {
        self.instrs
            .push(Instr::new2(mnemonic, opr1, opr2));
    }

    pub fn instr1(&mut self, mnemonic: Mnemonic, opr1: impl Into<Opr>) {
        self.instrs
            .push(Instr::new1(mnemonic, opr1));
    }

    pub fn instr0(&mut self, mnemonic: Mnemonic) {
        self.instrs.push(Instr::new0(mnemonic));
    }

    pub fn new_instr(&mut self, instr: Instr) {
        self.instrs.push(instr);
    }

    pub fn set_lable(&mut self, lable: impl Display) {
        let lable = lable.to_string();
        self.instrs.push(Instr::new1(Mnemonic::Lable, Opr::Loc(lable)));
    }
}

pub fn compact_binary_expr(
    left: &ExprOpr,
    right: &ExprOpr,
    op: &Op,
) -> Result<ExprOpr, CompilationError> {
    let vtype = left.vtype.cast(&right.vtype)?;
    let l_val = left.value.get_literal_value();
    let r_val = right.value.get_literal_value();
    match op {
        Op::Plus => Ok(ExprOpr::new(l_val + r_val, vtype)),
        Op::Sub => Ok(ExprOpr::new(l_val - r_val, vtype)),
        Op::Multi => Ok(ExprOpr::new(l_val * r_val, vtype)),
        Op::Devide => Ok(ExprOpr::new(l_val / r_val, vtype)),
        Op::Mod => Ok(ExprOpr::new(l_val % r_val, vtype)),
        Op::Or => Ok(ExprOpr::new(l_val | r_val, vtype)),
        Op::And => Ok(ExprOpr::new(l_val & r_val, vtype)),
        Op::Lsh => Ok(ExprOpr::new(l_val << r_val, vtype)),
        Op::Rsh => Ok(ExprOpr::new(l_val >> r_val, vtype)),
        Op::LogicalOr => Ok(ExprOpr::new(l_val | r_val, VariableType::Bool)),
        Op::LogicalAnd => Ok(ExprOpr::new(l_val & r_val, VariableType::Bool)),
        Op::Not => {
            return Err(CompilationError::InValidBinaryOperation(
                op.to_owned(),
                left.vtype.to_string(),
                right.vtype.to_string(),
            ));
        }
    }
}
