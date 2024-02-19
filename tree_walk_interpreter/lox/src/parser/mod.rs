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
        &self.peek().token_type == token_type
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

    fn expression(&mut self) -> Box<dyn Expr> {
        self.equality()
    }

    fn equality(&mut self) -> Box<dyn Expr> {
        // equality -> comparison (('!=' | '==') comparison) *
        let mut expr = self.comparison();
        while self.match_token(vec![TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = Operator::new(self.previous());
            let right = self.comparison();
            expr = Box::new(Binary::new(expr, right, operator));
        }
        expr
    }

    fn comparison(&mut self) -> Box<dyn Expr> {
        // comparison -> term ( ( > | >= | < | <= ) term)*
        let mut expr = self.term();
        while self.match_token(vec![
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = Operator::new(self.previous());
            let right = self.term();
            expr = Box::new(Binary::new(expr, right, operator));
        }
        expr
    }

    fn term(&mut self) -> Box<dyn Expr> {
        // term -> factor (( - | +) factor)*
        let mut expr = self.factor();
        while self.match_token(vec![TokenType::Minus, TokenType::Plus]) {
            let operator = Operator::new(self.previous());
            let right = self.factor();
            expr = Box::new(Binary::new(expr, right, operator));
        }
        expr
    }

    fn factor(&mut self) -> Box<dyn Expr> {
        // factor -> unary ((/ | *) unary)*
        let mut expr = self.unary();
        while self.match_token(vec![TokenType::Slash, TokenType::Star]) {
            let operator = Operator::new(self.previous());
            let right = self.unary();
            expr = Box::new(Binary::new(expr, right, operator));
        }
        expr
    }

    fn unary(&mut self) -> Box<dyn Expr> {
        // unary -> ('!' | '-') unary | primary
        if self.match_token(vec![TokenType::Bang, TokenType::Minus]) {
            let operator = Operator::new(self.previous());
            let right = self.unary();
            return Box::new(Unary::new(operator, right));
        }
        self.primary()
    }

    fn primary(&mut self) -> Box<dyn Expr> {
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
