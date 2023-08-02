use crate::parser::function::Function;

use super::stmt::VariableDeclare;

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
    StaticVar(VariableDeclare),
}
