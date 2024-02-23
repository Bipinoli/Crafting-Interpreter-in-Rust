use core::fmt;
use std::error::Error;

use crate::{
    error,
    scanner::token::{Token, TokenType},
};

use self::expression::{Binary, Expr, Grouping, Literal, Operator, Unary};

pub mod expression;
pub mod visitors;
pub use visitors::ast_printer;

pub struct Parser<'a> {
    tokens: &'a Vec<Token>,
    current: usize,
}

#[derive(Debug)]
struct ParserError {
    message: String,
}
impl Error for ParserError {}
impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl<'a> Parser<'a> {
    /// Recursive decent parser ///
    /// precendence rule:
    ///  1. primary : number | string | true | false | nil | ( expr )
    ///  2. unary -> ! | -
    ///  3. factor -> / | *
    ///  4. term -> - | +
    ///  5. comparision -> < | <= | > | >=
    ///  6. equality -> != | ==

    pub fn new(tokens: &'a Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Box<dyn Expr> {
        match self.expression() {
            Ok(result) => result,
            Err(_e) => Box::new(Literal::new(Token::new(TokenType::Nil, String::new(), 0))),
        }
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

    fn expression(&mut self) -> Result<Box<dyn Expr>, ParserError> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Box<dyn Expr>, ParserError> {
        // equality -> comparison (('!=' | '==') comparison) *
        let mut expr = self.comparison().unwrap();
        while self.match_token(vec![TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = Operator::new(self.previous());
            let right = self.comparison().unwrap();
            expr = Box::new(Binary::new(expr, right, operator));
        }
        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Box<dyn Expr>, ParserError> {
        // comparison -> term ( ( > | >= | < | <= ) term)*
        let mut expr = self.term().unwrap();
        while self.match_token(vec![
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = Operator::new(self.previous());
            let right = self.term().unwrap();
            expr = Box::new(Binary::new(expr, right, operator));
        }
        Ok(expr)
    }

    fn term(&mut self) -> Result<Box<dyn Expr>, ParserError> {
        // term -> factor (( - | +) factor)*
        let mut expr = self.factor().unwrap();
        while self.match_token(vec![TokenType::Minus, TokenType::Plus]) {
            let operator = Operator::new(self.previous());
            let right = self.factor().unwrap();
            expr = Box::new(Binary::new(expr, right, operator));
        }
        Ok(expr)
    }

    fn factor(&mut self) -> Result<Box<dyn Expr>, ParserError> {
        // factor -> unary ((/ | *) unary)*
        let mut expr = self.unary().unwrap();
        while self.match_token(vec![TokenType::Slash, TokenType::Star]) {
            let operator = Operator::new(self.previous());
            let right = self.unary().unwrap();
            expr = Box::new(Binary::new(expr, right, operator));
        }
        Ok(expr)
    }

    fn unary(&mut self) -> Result<Box<dyn Expr>, ParserError> {
        // unary -> ('!' | '-') unary | primary
        if self.match_token(vec![TokenType::Bang, TokenType::Minus]) {
            let operator = Operator::new(self.previous());
            let right = self.unary().unwrap();
            return Ok(Box::new(Unary::new(operator, right)));
        }
        self.primary()
    }

    fn primary(&mut self) -> Result<Box<dyn Expr>, ParserError> {
        // primary -> NUMBER | STRING | 'true' | 'false' | 'nil' | '(' expression ')'
        if self.match_token(vec![
            TokenType::Number,
            TokenType::String,
            TokenType::True,
            TokenType::False,
            TokenType::Nil,
        ]) {
            return Ok(Box::new(Literal::new(self.previous())));
        }
        if self.match_token(vec![TokenType::LeftParen]) {
            let expr = self.expression().unwrap();
            // consume the matching bracket after that
            return if let TokenType::RightParen = self.peek().token_type {
                self.advance();
                Ok(Box::new(Grouping::new(expr)))
            } else {
                Err(self.report_error(self.peek(), "No matching bracket for (".to_string()))
            };
        }
        Err(self.report_error(&self.previous(), "Invalid expression".to_string()))
    }

    fn report_error(&self, token: &Token, message: String) -> ParserError {
        let location = if token.token_type == TokenType::Eof {
            " at end".to_string()
        } else {
            format!(" at '{}'", token.lexeme)
        };
        error::report(token.line, message.clone(), location);
        ParserError { message }
    }

    fn synchronize(&mut self) {
        // after reporting error we should consume the tokens until the next statement
        // to avoid reporting false errors
        //
        self.advance();
        while !self.is_at_end() {
            if self.previous().token_type == TokenType::Eof {
                return;
            }
            match self.peek().token_type {
                TokenType::Class
                | TokenType::Fun
                | TokenType::Return
                | TokenType::Var
                | TokenType::For
                | TokenType::While
                | TokenType::If
                | TokenType::Print => return,
                _ => (),
            }
            self.advance();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let tokens = vec![
            Token::new(TokenType::Identifier, "a".to_string(), 1),
            Token::new(TokenType::Eof, "".to_string(), 1),
        ];
        let mut parser = Parser::new(&tokens);
        let expression = parser.parse();
    }
}
