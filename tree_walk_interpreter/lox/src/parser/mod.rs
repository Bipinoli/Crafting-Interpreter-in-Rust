use crate::scanner::token::{Token, TokenType};

use self::expression::{Binary, Expr, Grouping, Literal, Operator, Unary};

pub mod expression;
pub mod visitors;

struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    /// Recursive decent parser ///
    /// precendence rule:
    ///  1. primary : number | string | true | false | nil | ( expr )
    ///  2. unary -> ! | -
    ///  3. factor -> / | *
    ///  4. term -> - | +
    ///  5. comparision -> < | <= | > | >=
    ///  6. equality -> != | ==

    fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    fn match_token(&mut self, token_types: Vec<TokenType>) -> bool {
        for token_type in token_types {
            if self.check(&token_type) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        match self.peek().token_type {
            token_type => true,
            _ => false,
        }
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        match self.peek().token_type {
            TokenType::Eof => true,
            _ => false,
        }
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }

    fn expression(&self) -> Box<dyn Expr> {
        self.equality()
    }

    fn equality(&self) -> Box<dyn Expr> {
        // equality -> comparison (('!=' | '==') comparison) *
        let left = self.comparison();
    }

    fn comparison(&self) -> Box<dyn Expr> {
        // comparison -> term ( ( > | >= | < | <= ) term)*
    }

    fn term(&self) -> Box<dyn Expr> {
        // term -> factor (( - | +) factor)*
    }

    fn factor(&self) -> Box<dyn Expr> {
        // factor -> unary ((/ | *) unary)*
        let mut expr = Box::new(self.unary());
        while self.match_token(vec![TokenType::Slash, TokenType::Star]) {
            let operator = Operator::new(self.previous());
            let right = Box::new(self.unary());
            expr = Binary::new(expr, right, operator);
        }
        expr
    }

    fn unary(&self) -> Box<dyn Expr> {
        // unary -> ('!' | '-') unary | primary
        if self.match_token(vec![TokenType::Bang, TokenType::Minus]) {
            let operator = Operator::new(self.previous());
            let right = self.unary();
            return Box::new(Unary::new(operator, right));
        }
        self.primary()
    }

    fn primary(&self) -> Box<dyn Expr> {
        // primary -> NUMBER | STRING | 'true' | 'false' | 'nil' | '(' expression ')'
        if self.match_token(vec![
            TokenType::Number,
            TokenType::String,
            TokenType::True,
            TokenType::False,
            TokenType::Nil,
        ]) {
            return Box::new(Literal::new(self.previous()));
        }
        if self.match_token(vec![TokenType::LeftParen]) {
            let expr = self.expression();
            // consume the matching bracket after that
            match self.peek().token_type {
                TokenType::RightParen => {
                    self.advance();
                }
                _ => panic!("No matching bracket for ("),
            }
            return Box::new(Grouping::new(expr));
        }
        panic!("unknown token in primary category");
    }
}
