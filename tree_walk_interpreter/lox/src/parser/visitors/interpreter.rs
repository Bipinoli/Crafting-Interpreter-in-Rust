use crate::parser::expression::{Binary, Expr, ExpressionVisitor, Grouping, Literal, Unary};
use crate::scanner::token::TokenType;
use core::fmt;
use std::any::{Any, TypeId};
use std::error::Error;

#[derive(Debug)]
struct RuntimeError {
    message: String,
}
impl Error for RuntimeError {}
impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

pub struct AstInterpreterVisitor {}

impl AstInterpreterVisitor {
    pub fn new() -> Self {
        AstInterpreterVisitor {}
    }
    pub fn interpret(expr: &dyn Expr) {
        let val = evaluate(expr);
    }
}

fn evaluate(expr: &dyn Expr) -> Result<Box<dyn Any>, Box<dyn Error>> {
    expr.accept(Box::new(AstInterpreterVisitor::new()))
}

fn is_string(val: &Box<dyn Any>) -> bool {
    (**val).type_id() == TypeId::of::<String>()
}
fn is_nil(val: &Box<dyn Any>) -> bool {
    (**val).type_id() == TypeId::of::<()>()
}
fn is_f64(val: &Box<dyn Any>) -> bool {
    (**val).type_id() == TypeId::of::<f64>()
}
fn is_bool(val: &Box<dyn Any>) -> bool {
    (**val).type_id() == TypeId::of::<f64>()
}

fn is_equal(left: Box<dyn Any>, right: Box<dyn Any>) -> Result<bool, ()> {
    if is_nil(&left) && is_nil(&right) {
        Ok(true)
    } else if is_nil(&left) || is_nil(&right) {
        Ok(false)
    } else if is_string(&left) && is_string(&right) {
        let left = *left.downcast::<String>().unwrap();
        let right = *right.downcast::<String>().unwrap();
        Ok(left == right)
    } else if is_f64(&left) && is_f64(&right) {
        let left = *left.downcast::<f64>().unwrap();
        let right = *right.downcast::<f64>().unwrap();
        Ok(left == right)
    } else {
        Err(())
    }
}

impl ExpressionVisitor for AstInterpreterVisitor {
    fn for_unary(&self, expr: &Unary) -> Result<Box<dyn Any>, Box<dyn Error>> {
        let right = evaluate(&*expr.right);
        if let Err(e) = right {
            return Err(e);
        }
        let right = right.unwrap();
        match expr.operator.token.token_type {
            TokenType::Minus => {
                let value = *right.downcast::<f64>().unwrap();
                Ok(Box::new(-value))
            }
            TokenType::Bang => {
                if is_bool(&right) {
                    let value = *right.downcast::<bool>().unwrap();
                    Ok(Box::new(value))
                } else if is_nil(&right) {
                    Ok(Box::new(false))
                } else {
                    Ok(Box::new(true))
                }
            }
            _ => Err(Box::new(RuntimeError {
                message: format!(
                    "[RuntimeError] line:{} invalid unary operator {}",
                    expr.operator.token.line, expr.operator.token.lexeme,
                ),
            })),
        }
    }

    fn for_binary(&self, expr: &Binary) -> Result<Box<dyn Any>, Box<dyn Error>> {
        let left = evaluate(&*expr.left);
        let right = evaluate(&*expr.right);
        if let Err(e) = left {
            return Err(e);
        }
        if let Err(e) = right {
            return Err(e);
        }
        let left = left.unwrap();
        let right = right.unwrap();
        match expr.operator.token.token_type {
            TokenType::Minus => {
                let left = *left.downcast::<f64>().unwrap();
                let right = *right.downcast::<f64>().unwrap();
                Ok(Box::new(left - right))
            }
            TokenType::Slash => {
                let left = *left.downcast::<f64>().unwrap();
                let right = *right.downcast::<f64>().unwrap();
                Ok(Box::new(left / right))
            }
            TokenType::Star => {
                let left = *left.downcast::<f64>().unwrap();
                let right = *right.downcast::<f64>().unwrap();
                Ok(Box::new(left * right))
            }
            TokenType::Plus => {
                if is_string(&left) && is_string(&right) {
                    let left = *left.downcast::<String>().unwrap();
                    let right = *right.downcast::<String>().unwrap();
                    let result = format!("{}{}", left, right);
                    Ok(Box::new(result))
                } else if is_f64(&left) && is_f64(&right) {
                    let left = *left.downcast::<f64>().unwrap();
                    let right = *right.downcast::<f64>().unwrap();
                    Ok(Box::new(left + right))
                } else {
                    Err(Box::new(RuntimeError {
                        message: format!(
                            "[RuntimeError] line:{} '+' works only between numbers or string",
                            expr.operator.token.line
                        ),
                    }))
                }
            }
            TokenType::Greater => {
                if is_f64(&left) && is_f64(&right) {
                    let left = *left.downcast::<f64>().unwrap();
                    let right = *right.downcast::<f64>().unwrap();
                    Ok(Box::new(left > right))
                } else {
                    Err(Box::new(RuntimeError {
                        message: format!(
                            "[RuntimeError] line:{} '>' works only between numbers",
                            expr.operator.token.line
                        ),
                    }))
                }
            }
            TokenType::GreaterEqual => {
                if is_f64(&left) && is_f64(&right) {
                    let left = *left.downcast::<f64>().unwrap();
                    let right = *right.downcast::<f64>().unwrap();
                    Ok(Box::new(left >= right))
                } else {
                    Err(Box::new(RuntimeError {
                        message: format!(
                            "[RuntimeError] line:{} '>=' works only between numbers",
                            expr.operator.token.line
                        ),
                    }))
                }
            }
            TokenType::Less => {
                if is_f64(&left) && is_f64(&right) {
                    let left = *left.downcast::<f64>().unwrap();
                    let right = *right.downcast::<f64>().unwrap();
                    Ok(Box::new(left < right))
                } else {
                    Err(Box::new(RuntimeError {
                        message: format!(
                            "[RuntimeError] line:{} '<' works only between numbers",
                            expr.operator.token.line
                        ),
                    }))
                }
            }
            TokenType::LessEqual => {
                if is_f64(&left) && is_f64(&right) {
                    let left = *left.downcast::<f64>().unwrap();
                    let right = *right.downcast::<f64>().unwrap();
                    Ok(Box::new(left <= right))
                } else {
                    Err(Box::new(RuntimeError {
                        message: format!(
                            "[RuntimeError] line:{} '<=' works only between numbers",
                            expr.operator.token.line
                        ),
                    }))
                }
            }
            TokenType::EqualEqual => match is_equal(left, right) {
                Ok(result) => Ok(Box::new(result)),
                Err(_) => Err(Box::new(RuntimeError {
                    message: format!(
                        "[RuntimeError] line:{} '==' has incompatible operands",
                        expr.operator.token.line
                    ),
                })),
            },
            TokenType::BangEqual => match is_equal(left, right) {
                Ok(result) => Ok(Box::new(!result)),
                Err(_) => Err(Box::new(RuntimeError {
                    message: format!(
                        "[RuntimeError] line:{} '!=' has incompatible operands",
                        expr.operator.token.line
                    ),
                })),
            },
            _ => Err(Box::new(RuntimeError {
                message: format!(
                    "[RuntimeError] line:{} invalid binary operator '{}'",
                    expr.operator.token.line, expr.operator.token.lexeme
                ),
            })),
        }
    }

    fn for_literal(&self, expr: &Literal) -> Result<Box<dyn Any>, Box<dyn Error>> {
        match expr.get_value() {
            Ok(result) => Ok(result),
            Err(_) => Err(Box::new(RuntimeError {
                message: format!(
                    "[RuntimeError] line:{} unknown literal {}",
                    expr.token.line, expr.token.lexeme
                ),
            })),
        }
    }

    fn for_grouping(&self, expr: &Grouping) -> Result<Box<dyn Any>, Box<dyn Error>> {
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
        // 10 - (5 + 3) == (10 - (-10)) / (5 * 2)
        let literal_10 = Literal::new(Token::new(TokenType::Number, "10".to_owned(), 1));
        let value = literal_10
            .accept(Box::new(AstInterpreterVisitor::new()))
            .unwrap();
        let value = *value.downcast::<f64>().unwrap();
        assert_eq!(value, 10.0);

        let literal_5 = Literal::new(Token::new(TokenType::Number, "5".to_owned(), 1));
        let value = literal_5
            .accept(Box::new(AstInterpreterVisitor::new()))
            .unwrap();
        let value = *value.downcast::<f64>().unwrap();
        assert_eq!(value, 5.0);

        let literal_3 = Literal::new(Token::new(TokenType::Number, "3".to_owned(), 1));
        let value = literal_3
            .accept(Box::new(AstInterpreterVisitor::new()))
            .unwrap();
        let value = *value.downcast::<f64>().unwrap();
        assert_eq!(value, 3.0);

        let bin_5_3 = Binary::new(
            Box::new(literal_5),
            Box::new(literal_3),
            Operator::new(Token::new(TokenType::Plus, "+".to_owned(), 1)),
        );
        let value = bin_5_3
            .accept(Box::new(AstInterpreterVisitor::new()))
            .unwrap();
        let value = *value.downcast::<f64>().unwrap();
        assert_eq!(value, 8.0);

        let left = Binary::new(
            Box::new(literal_10),
            Box::new(Grouping::new(Box::new(bin_5_3))),
            Operator::new(Token::new(TokenType::Minus, "-".to_owned(), 1)),
        );
        let value = left.accept(Box::new(AstInterpreterVisitor::new())).unwrap();
        let value = *value.downcast::<f64>().unwrap();
        assert_eq!(value, 2.0);

        let bin_10_min_10 = Binary::new(
            Box::new(Literal::new(Token::new(
                TokenType::Number,
                "10".to_owned(),
                1,
            ))),
            Box::new(Unary::new(
                Operator::new(Token::new(TokenType::Minus, "-".to_owned(), 1)),
                Box::new(Literal::new(Token::new(
                    TokenType::Number,
                    "10".to_owned(),
                    1,
                ))),
            )),
            Operator::new(Token::new(TokenType::Minus, "-".to_owned(), 1)),
        );
        let value = bin_10_min_10
            .accept(Box::new(AstInterpreterVisitor::new()))
            .unwrap();
        let value = *value.downcast::<f64>().unwrap();
        assert_eq!(value, 20.0);

        let five_2 = Binary::new(
            Box::new(Literal::new(Token::new(
                TokenType::Number,
                "5".to_owned(),
                1,
            ))),
            Box::new(Literal::new(Token::new(
                TokenType::Number,
                "2".to_owned(),
                1,
            ))),
            Operator::new(Token::new(TokenType::Star, "*".to_owned(), 1)),
        );
        let value = five_2
            .accept(Box::new(AstInterpreterVisitor::new()))
            .unwrap();
        let value = *value.downcast::<f64>().unwrap();
        assert_eq!(value, 10.0);

        let right = Binary::new(
            Box::new(bin_10_min_10),
            Box::new(five_2),
            Operator::new(Token::new(TokenType::Slash, "/".to_owned(), 1)),
        );
        let value = right
            .accept(Box::new(AstInterpreterVisitor::new()))
            .unwrap();
        let value = *value.downcast::<f64>().unwrap();
        assert_eq!(value, 2.0);

        let expr = Binary::new(
            Box::new(left),
            Box::new(right),
            Operator::new(Token::new(TokenType::EqualEqual, "==".to_owned(), 1)),
        );
        let value = expr.accept(Box::new(AstInterpreterVisitor::new())).unwrap();
        let value = *value.downcast::<bool>().unwrap();
        assert_eq!(value, true);
    }
}
