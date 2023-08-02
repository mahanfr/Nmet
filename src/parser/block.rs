use crate::parser::stmt::Stmt;

#[derive(Debug, Clone)]
pub struct Block {
    pub stmts: Vec<Stmt>,
}
