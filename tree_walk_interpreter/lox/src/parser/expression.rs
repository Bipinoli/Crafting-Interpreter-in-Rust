use crate::scanner::token::{Token, TokenType};

pub trait ExpressionVisitor {
    fn for_unary(&self, visitable: &Unary);
    fn for_binary(&self, visitable: &Binary);
    fn for_grouping(&self, visitable: &Grouping);
    fn for_literal(&self, visitable: &Literal);
}
pub trait VisitableExpression {
    fn accept(&self, visitor: Box<dyn ExpressionVisitor>);
}

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
impl Unary {
    pub fn new(operator: Operator, right: Box<dyn Expr>) -> Unary {
        Unary { operator, right }
    }
}

pub struct Grouping {
    expr: Box<dyn Expr>,
}
impl Grouping {
    pub fn new(expr: Box<dyn Expr>) -> Grouping {
        Grouping { expr }
    }
}

impl Expr for Binary {}
impl Expr for Unary {}
impl Expr for Grouping {}
impl Expr for Literal {}

impl VisitableExpression for Binary {
    fn accept(&self, visitor: Box<dyn ExpressionVisitor>) {
        visitor.for_binary(self)
    }
}
impl VisitableExpression for Unary {
    fn accept(&self, visitor: Box<dyn ExpressionVisitor>) {
        visitor.for_unary(self)
    }
}
impl VisitableExpression for Grouping {
    fn accept(&self, visitor: Box<dyn ExpressionVisitor>) {
        visitor.for_grouping(self)
    }
}
impl VisitableExpression for Literal {
    fn accept(&self, visitor: Box<dyn ExpressionVisitor>) {
        visitor.for_literal(self)
    }
}
