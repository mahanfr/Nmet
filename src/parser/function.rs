use crate::parser::block::Block;

use super::stmt::VariableType;

#[derive(Debug, Clone)]
pub struct FunctionArg {
    pub ident: String,
    pub typedef: VariableType,
}

#[derive(Debug, Clone)]
pub struct Function {
    pub ident: String,
    pub args: Vec<FunctionArg>,
    pub block: Block,
    pub ret_type: Option<VariableType>,
}
