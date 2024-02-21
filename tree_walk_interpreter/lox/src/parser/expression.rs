use crate::scanner::token::{Token, TokenType};
use std::any::Any;

pub trait ExpressionVisitor {
    fn for_unary(&self, expr: &Unary) -> Box<dyn Any>;
    fn for_binary(&self, expr: &Binary) -> Box<dyn Any>;
    fn for_grouping(&self, expr: &Grouping) -> Box<dyn Any>;
    fn for_literal(&self, expr: &Literal) -> Box<dyn Any>;
}

pub trait Expr {
    fn accept(&self, visitor: Box<dyn ExpressionVisitor>) -> Box<dyn Any>;
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
    fn accept(&self, visitor: Box<dyn ExpressionVisitor>) -> Box<dyn Any> {
        visitor.for_binary(self)
    }
}

impl Expr for Unary {
    fn accept(&self, visitor: Box<dyn ExpressionVisitor>) -> Box<dyn Any> {
        visitor.for_unary(self)
    }
}

impl Expr for Grouping {
    fn accept(&self, visitor: Box<dyn ExpressionVisitor>) -> Box<dyn Any> {
        visitor.for_grouping(self)
    }
}

impl Expr for Literal {
    fn accept(&self, visitor: Box<dyn ExpressionVisitor>) -> Box<dyn Any> {
        visitor.for_literal(self)
    }
}
