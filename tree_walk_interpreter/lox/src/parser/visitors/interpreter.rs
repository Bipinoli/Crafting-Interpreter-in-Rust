use crate::parser::expression::{Binary, Expr, ExpressionVisitor, Grouping, Literal, Unary};
use crate::scanner::token::TokenType;
use std::any::{Any, TypeId};

pub struct AstInterpreterVisitor {}

impl AstInterpreterVisitor {
    pub fn new() -> Self {
        AstInterpreterVisitor {}
    }
}

fn evaluate(expr: &dyn Expr) -> Box<dyn Any> {
    expr.accept(Box::new(AstInterpreterVisitor::new()))
}

impl ExpressionVisitor for AstInterpreterVisitor {
    fn for_unary(&self, expr: &Unary) -> Box<dyn Any> {
        let right = evaluate(expr);
        match expr.operator.token.token_type {
            TokenType::Minus => {
                let value = *right.downcast::<f64>().unwrap();
                Box::new(-value)
            }
            TokenType::Bang => {
                if (*right).type_id() == TypeId::of::<bool>() {
                    let value = *right.downcast::<bool>().unwrap();
                    Box::new(value)
                } else if (*right).type_id() == TypeId::of::<()>() {
                    Box::new(false)
                } else {
                    Box::new(true)
                }
            }
            _ => panic!("invalid token in unary"),
        }
    }

    fn for_binary(&self, expr: &Binary) -> Box<dyn Any> {}

    fn for_literal(&self, expr: &Literal) -> Box<dyn Any> {
        let value = expr.get_value();
        value.1
    }

    fn for_grouping(&self, expr: &Grouping) -> Box<dyn Any> {}
}
