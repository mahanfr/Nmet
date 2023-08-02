use crate::parser::expr::Expr;
use crate::parser::block::Block;

#[derive(Debug, Clone)]
pub enum Stmt {
    // expr
    Expr(Expr),
    VariableDecl(VariableDeclare),
    // expr = expr
    Assgin(Assgin),
    Print(Expr),
    While(WhileStmt),
    If(IFStmt),
    Return(Expr),
    Break,
    Continue,
}

#[derive(Debug, Clone)]
pub struct Assgin {
    pub left: Expr,
    pub right: Expr,
    pub op: AssginOp,
}

#[derive(Debug, Clone)]
pub enum AssginOp {
    Eq,
}

#[derive(Debug, Clone)]
pub struct IFStmt {
    pub condition: Expr,
    pub then_block: Block,
    pub else_block: Box<ElseBlock>,
}

#[derive(Debug, Clone)]
pub enum ElseBlock {
    Elif(IFStmt),
    Else(Block),
    None,
}

#[derive(Debug, Clone)]
pub struct WhileStmt {
    pub condition: Expr,
    pub block: Block,
}

#[derive(Debug, Clone)]
pub struct VariableDeclare {
    pub mutable: bool,
    pub ident: String,
    pub init_value: Option<Expr>,
}

