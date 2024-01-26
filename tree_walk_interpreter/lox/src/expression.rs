use crate::scanner::token::{Token, TokenType};

pub trait Expr {}

pub struct Operator {
    token: Token,
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
    left: Box<dyn Expr>,
    right: Box<dyn Expr>,
    operator: Operator,
}

pub struct Literal {
    token: Token,
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
    operator: Operator,
    right: Box<dyn Expr>,
}

pub struct Grouping {
    expr: Box<dyn Expr>,
}

impl Expr for Binary {}
impl Expr for Unary {}
impl Expr for Grouping {}
impl Expr for Literal {}
