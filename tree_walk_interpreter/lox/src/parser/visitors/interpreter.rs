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

fn is_equal(left: Box<dyn Any>, right: Box<dyn Any>) -> bool {
    if is_nil(&left) && is_nil(&right) {
        true
    } else if is_nil(&left) || is_nil(&right) {
        false
    } else if is_string(&left) && is_string(&right) {
        let left = *left.downcast::<String>().unwrap();
        let right = *right.downcast::<String>().unwrap();
        left == right
    } else if is_f64(&left) && is_f64(&right) {
        let left = *left.downcast::<f64>().unwrap();
        let right = *right.downcast::<f64>().unwrap();
        left == right
    } else {
        panic!("invalid operand in ==")
    }
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
            TokenType::EqualEqual => Box::new(is_equal(left, right)),
            TokenType::BangEqual => Box::new(!is_equal(left, right)),
            _ => panic!("invalid operator in binary"),
        }
    }

    fn for_literal(&self, expr: &Literal) -> Box<dyn Any> {
        expr.get_value()
    }

    fn for_grouping(&self, expr: &Grouping) -> Box<dyn Any> {
        evaluate(&*expr.expr)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        parser::expression::{Binary, Expr, Grouping, Literal, Operator, Unary},
        scanner::token::{Token, TokenType},
    };

    use super::AstInterpreterVisitor;

    #[test]
    fn it_works() {
        // 10 - (5 + 3) == (10 - (-10)) / (5 + 5)
        let literal_10 = Literal::new(Token::new(TokenType::Number, "10".to_owned(), 1));
        let value = literal_10.accept(Box::new(AstInterpreterVisitor::new()));
        let value = *value.downcast::<f64>().unwrap();
        assert_eq!(value, 10.0);

        let literal_5 = Literal::new(Token::new(TokenType::Number, "5".to_owned(), 1));
        let value = literal_5.accept(Box::new(AstInterpreterVisitor::new()));
        let value = *value.downcast::<f64>().unwrap();
        assert_eq!(value, 5.0);

        let literal_3 = Literal::new(Token::new(TokenType::Number, "3".to_owned(), 1));
        let value = literal_3.accept(Box::new(AstInterpreterVisitor::new()));
        let value = *value.downcast::<f64>().unwrap();
        assert_eq!(value, 3.0);

        let bin_5_3 = Binary::new(
            Box::new(literal_5),
            Box::new(literal_3),
            Operator::new(Token::new(TokenType::Plus, "+".to_owned(), 1)),
        );
        let value = bin_5_3.accept(Box::new(AstInterpreterVisitor::new()));
        let value = *value.downcast::<f64>().unwrap();
        assert_eq!(value, 8.0);
        // let left = Binary::new(
        //     Box::new(),
        //     Box::new(Grouping::new(Box::new(Binary::new(
        //         Box::new(Literal::new(Token::new(
        //             TokenType::Number,
        //             "5".to_owned(),
        //             1,
        //         ))),
        //         Box::new(Literal::new(Token::new(
        //             TokenType::Number,
        //             "3".to_owned(),
        //             1,
        //         ))),
        //         Operator::new(Token::new(TokenType::Plus, "+".to_owned(), 1)),
        //     )))),
        //     Operator::new(Token::new(TokenType::Minus, "-".to_owned(), 1)),
        // );

        // let left_val = left.accept(Box::new(AstInterpreterVisitor::new()));
        // let left_val = *left_val.downcast::<f64>().unwrap();
        // assert_eq!(left_val, 2.0);
    }
}
