use crate::scanner::token::{Token, TokenType};
use std::any::Any;
use std::error::Error;

pub trait ExpressionVisitor {
    fn for_unary(&self, expr: &Unary) -> Result<Box<dyn Any>, Box<dyn Error>>;
    fn for_binary(&self, expr: &Binary) -> Result<Box<dyn Any>, Box<dyn Error>>;
    fn for_grouping(&self, expr: &Grouping) -> Result<Box<dyn Any>, Box<dyn Error>>;
    fn for_literal(&self, expr: &Literal) -> Result<Box<dyn Any>, Box<dyn Error>>;
}

pub trait Expr {
    fn accept(&self, visitor: Box<dyn ExpressionVisitor>) -> Result<Box<dyn Any>, Box<dyn Error>>;
}

pub struct Operator {
    pub token: Token,
}
impl Operator {
    pub fn new(token: Token) -> Operator {
        match token.token_type {
            TokenType::EqualEqual
            | TokenType::BangEqual
            | TokenType::Less
            | TokenType::LessEqual
            | TokenType::Greater
            | TokenType::GreaterEqual
            | TokenType::Plus
            | TokenType::Minus
            | TokenType::Star
            | TokenType::Slash => Operator { token },
            _ => panic!("invalid token for operator"),
        }
    }
}

pub struct Binary {
    pub left: Box<dyn Expr>,
    pub right: Box<dyn Expr>,
    pub operator: Operator,
}
impl Binary {
    pub fn new(left: Box<dyn Expr>, right: Box<dyn Expr>, operator: Operator) -> Binary {
        Binary {
            left,
            right,
            operator,
        }
    }
}

pub struct Literal {
    pub token: Token,
}
impl Literal {
    pub fn new(token: Token) -> Literal {
        match token.token_type {
            TokenType::Number
            | TokenType::String
            | TokenType::True
            | TokenType::False
            | TokenType::Nil => Literal { token },
            _ => panic!("invalid token for literal"),
        }
    }
    pub fn get_value(&self) -> Result<Box<dyn Any>, ()> {
        match self.token.token_type {
            TokenType::Number => {
                let n: f64 = self.token.lexeme.parse().unwrap();
                Ok(Box::new(n))
            }
            TokenType::String => Ok(Box::new(self.token.lexeme.clone())),
            TokenType::True => Ok(Box::new(true)),
            TokenType::False => Ok(Box::new(false)),
            TokenType::Nil => Ok(Box::new(())),
            _ => Err(()),
        }
    }
}

pub struct Unary {
    pub operator: Operator,
    pub right: Box<dyn Expr>,
}
impl Unary {
    pub fn new(operator: Operator, right: Box<dyn Expr>) -> Unary {
        Unary { operator, right }
    }
}

pub struct Grouping {
    pub expr: Box<dyn Expr>,
}
impl Grouping {
    pub fn new(expr: Box<dyn Expr>) -> Grouping {
        Grouping { expr }
    }
}

impl Expr for Binary {
    fn accept(&self, visitor: Box<dyn ExpressionVisitor>) -> Result<Box<dyn Any>, Box<dyn Error>> {
        visitor.for_binary(self)
    }
}

impl Expr for Unary {
    fn accept(&self, visitor: Box<dyn ExpressionVisitor>) -> Result<Box<dyn Any>, Box<dyn Error>> {
        visitor.for_unary(self)
    }
}

impl Expr for Grouping {
    fn accept(&self, visitor: Box<dyn ExpressionVisitor>) -> Result<Box<dyn Any>, Box<dyn Error>> {
        visitor.for_grouping(self)
    }
}

impl Expr for Literal {
    fn accept(&self, visitor: Box<dyn ExpressionVisitor>) -> Result<Box<dyn Any>, Box<dyn Error>> {
        visitor.for_literal(self)
    }
}
