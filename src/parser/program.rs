use crate::parser::expr::Expr;
use crate::parser::function::Function;

#[derive(Debug, Clone)]
pub struct StaticVariable {
    pub ident: String,
    pub value: Expr,
}

#[derive(Debug, Clone)]
pub struct ProgramFile {
    pub shebang: String,
    pub file_path: String,
    // pub attrs: Vec<Attr>
    pub items: Vec<ProgramItem>,
}

#[derive(Debug, Clone)]
pub enum ProgramItem {
    Func(Function),
    StaticVar(StaticVariable),
}
