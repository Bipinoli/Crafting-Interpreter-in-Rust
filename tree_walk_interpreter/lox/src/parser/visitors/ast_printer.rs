use std::vec;

use crate::parser::expression::{Binary, Grouping, Literal, Unary};
use crate::parser::expression::{Expr, ExpressionVisitor};

pub struct AstPrinterVisitor {}

impl AstPrinterVisitor {
    pub fn new() -> AstPrinterVisitor {
        AstPrinterVisitor {}
    }
}

fn parenthesize(name: String, exprs: Vec<&Box<dyn Expr>>) -> String {
    let mut sub_rslts: Vec<String> = vec![];
    for expr in exprs {
        sub_rslts.push((*expr).accept(Box::new(AstPrinterVisitor::new())));
    }
    format!("({name} {})", sub_rslts.join(" "))
}

impl ExpressionVisitor for AstPrinterVisitor {
    fn for_unary(&self, expr: &Unary) -> String {
        let name = (*expr).operator.token.lexeme.clone();
        let right = &(*expr).right;
        parenthesize(name, vec![right])
    }
    fn for_binary(&self, expr: &Binary) -> String {
        let name = (*expr).operator.token.lexeme.clone();
        let left = &expr.left;
        let right = &expr.right;
        parenthesize(name, vec![left, right])
    }
    fn for_literal(&self, expr: &Literal) -> String {
        expr.token.lexeme.clone()
    }
    fn for_grouping(&self, expr: &Grouping) -> String {
        let name = "group".to_owned();
        let expr = &expr.expr;
        parenthesize(name, vec![expr])
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        parser::expression::{Binary, Expr, Grouping, Literal, Operator, Unary},
        scanner::token::{Token, TokenType},
    };

    use super::AstPrinterVisitor;

    #[test]
    fn it_works() {
        let expr = Binary::new(
            Box::new(Unary::new(
                Operator::new(Token::new(TokenType::Minus, "-".to_owned(), 1)),
                Box::new(Literal::new(Token::new(
                    TokenType::Number,
                    "123".to_owned(),
                    1,
                ))),
            )),
            Box::new(Grouping::new(Box::new(Literal::new(Token::new(
                TokenType::Number,
                "321".to_owned(),
                1,
            ))))),
            Operator::new(Token::new(TokenType::Star, "*".to_owned(), 1)),
        );
        let expected = "(* (- 123) (group 321))".to_owned();
        let ast = expr.accept(Box::new(AstPrinterVisitor::new()));
        println!("{ast}");
        assert_eq!(ast, expected);
    }
}
