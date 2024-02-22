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

fn is_string(val: &Box<dyn Any>) -> bool {
    (*val).type_id() == TypeId::of::<String>()
}
fn is_nil(val: &Box<dyn Any>) -> bool {
    (*val).type_id() == TypeId::of::<()>()
}
fn is_f64(val: &Box<dyn Any>) -> bool {
    (*val).type_id() == TypeId::of::<f64>()
}
fn is_bool(val: &Box<dyn Any>) -> bool {
    (*val).type_id() == TypeId::of::<f64>()
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
                if is_bool(&right) {
                    let value = *right.downcast::<bool>().unwrap();
                    Box::new(value)
                } else if is_nil(&right) {
                    Box::new(false)
                } else {
                    Box::new(true)
                }
            }
            _ => panic!("invalid token in unary"),
        }
    }

    fn for_binary(&self, expr: &Binary) -> Box<dyn Any> {
        let left = evaluate(&*expr.left);
        let right = evaluate(&*expr.right);
        match expr.operator.token.token_type {
            TokenType::Minus => {
                let left = *left.downcast::<f64>().unwrap();
                let right = *right.downcast::<f64>().unwrap();
                Box::new(left - right)
            }
            TokenType::Slash => {
                let left = *left.downcast::<f64>().unwrap();
                let right = *right.downcast::<f64>().unwrap();
                Box::new(left / right)
            }
            TokenType::Star => {
                let left = *left.downcast::<f64>().unwrap();
                let right = *right.downcast::<f64>().unwrap();
                Box::new(left * right)
            }
            TokenType::Plus => {
                if is_string(&left) && is_string(&right) {
                    let left = *left.downcast::<String>().unwrap();
                    let right = *right.downcast::<String>().unwrap();
                    let result = format!("{}{}", left, right);
                    Box::new(result)
                } else if is_f64(&left) && is_f64(&right) {
                    let left = *left.downcast::<f64>().unwrap();
                    let right = *right.downcast::<f64>().unwrap();
                    Box::new(left + right)
                } else {
                    panic!("+ operator in binary only works with string and number")
                }
            }
            TokenType::Greater => {
                if is_f64(&left) && is_f64(&right) {
                    let left = *left.downcast::<f64>().unwrap();
                    let right = *right.downcast::<f64>().unwrap();
                    Box::new(left > right)
                } else {
                    panic!("> operator needs numbers as operand")
                }
            }
            TokenType::GreaterEqual => {
                if is_f64(&left) && is_f64(&right) {
                    let left = *left.downcast::<f64>().unwrap();
                    let right = *right.downcast::<f64>().unwrap();
                    Box::new(left >= right)
                } else {
                    panic!(">= operator needs numbers as operand")
                }
            }
            TokenType::Less => {
                if is_f64(&left) && is_f64(&right) {
                    let left = *left.downcast::<f64>().unwrap();
                    let right = *right.downcast::<f64>().unwrap();
                    Box::new(left < right)
                } else {
                    panic!("< operator needs numbers as operand")
                }
            }
            TokenType::LessEqual => {
                if is_f64(&left) && is_f64(&right) {
                    let left = *left.downcast::<f64>().unwrap();
                    let right = *right.downcast::<f64>().unwrap();
                    Box::new(left <= right)
                } else {
                    panic!("<= operator needs numbers as operand")
                }
            }
            TokenType::EqualEqual => {}
            TokenType::BangEqual => {}
            _ => panic!("invalid operator in binary"),
        }
    }

    fn for_literal(&self, expr: &Literal) -> Box<dyn Any> {
        let value = expr.get_value();
        value.1
    }

    fn for_grouping(&self, expr: &Grouping) -> Box<dyn Any> {}
}
