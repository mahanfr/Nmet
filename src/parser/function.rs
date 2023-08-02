use crate::parser::block::Block;

#[derive(Debug, Clone)]
pub struct FunctionArg {
    pub ident: String,
}

#[derive(Debug, Clone)]
pub struct Function {
    pub ident: String,
    pub args: Vec<FunctionArg>,
    pub block: Block,
}

