use super::Expr;

pub enum Stmt {
    ExprStmt(Box<dyn Expr>),
    PrintStmt(Box<dyn Expr>),
}
