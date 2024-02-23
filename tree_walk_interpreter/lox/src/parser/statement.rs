use super::expression::Expr;

pub enum Stmt {
    ExprStmt(Box<dyn Expr>),
    PrintStmt(Box<dyn Expr>),
}

