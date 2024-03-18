#![allow(dead_code)]
use crate::scanner::token::{Token, TokenType};
use std::collections::HashMap;
use std::process;

pub mod token;

pub struct Scanner {
    pub source: Vec<char>,
    pub tokens: Vec<Token>,
    line: usize,
    cursor: usize,
}
impl Scanner {
    pub fn new(source: &String) -> Scanner {
        Scanner {
            source: source.chars().collect(),
            tokens: vec![],
            line: 1,
            cursor: 0,
        }
    }
    fn peek(&self) -> Option<&char> {
        if self.cursor >= self.source.len() {
            None
        } else {
            Some(&self.source[self.cursor])
        }
    }
    fn peek_further(&self) -> Option<&char> {
        if self.cursor + 1 >= self.source.len() {
            None
        } else {
            Some(&self.source[self.cursor + 1])
        }
    }
    fn next(&mut self) -> Option<char> {
        if self.cursor >= self.source.len() {
            None
        } else {
            let retval = self.source[self.cursor].clone();
            self.cursor += 1;
            Some(retval)
        }
    }
    fn rewind(&mut self) {
        self.cursor -= 1;
    }
    fn prev_non_whitespace(&self) -> Option<&char> {
        let mut cursor = self.cursor;
        loop {
            if cursor == 0 {
                return None;
            }
            cursor -= 1;
            if !Self::is_whitespace(&self.source[cursor]) {
                return Some(&self.source[cursor]);
            }
        }
    }
    fn next_non_whitespace(&self) -> Option<&char> {
        let mut cursor = self.cursor;
        loop {
            if cursor >= self.source.len() {
                return None;
            }
            cursor += 1;
            if !Self::is_whitespace(&self.source[cursor]) {
                return Some(&self.source[cursor]);
            }
        }
    }
    fn eat_whitespace(&mut self) {
        while self.peek() != None {
            if !Self::is_whitespace(self.peek().unwrap()) {
                break;
            }
            self.next();
        }
    }
    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while self.peek() != None {
            self.scan_token();
        }
        self.add_token(TokenType::Eof, String::new());
        &self.tokens
    }

    fn scan_token(&mut self) {
        match self.next() {
            None => {}
            Some(c) => match c {
                ' ' | '\r' | '\t' => {}
                '\n' => self.line += 1,

                '(' => self.add_token(TokenType::LeftParen, String::from("(")),
                ')' => self.add_token(TokenType::RightParen, String::from(")")),
                '{' => self.add_token(TokenType::LeftBrace, String::from("{")),
                '}' => self.add_token(TokenType::RightBrace, String::from("}")),
                ',' => self.add_token(TokenType::Comma, String::from(",")),
                '.' => self.add_token(TokenType::Dot, String::from(".")),
                ';' => self.add_token(TokenType::Semicolon, String::from(";")),

                '+' => self.add_token(TokenType::Plus, String::from("+")),
                '-' => {
                    if self.is_signed_number() {
                        self.eat_whitespace();
                        self.match_number('-');
                    } else {
                        self.add_token(TokenType::Minus, String::from("-"));
                    }
                }

                '*' => self.add_token(TokenType::Star, String::from("*")),
                '!' => {
                    self.match_next_char(
                        '=',
                        TokenType::BangEqual,
                        String::from("!="),
                        TokenType::Bang,
                        String::from("!"),
                    );
                }
                '=' => {
                    self.match_next_char(
                        '=',
                        TokenType::EqualEqual,
                        String::from("=="),
                        TokenType::Equal,
                        String::from("="),
                    );
                }
                '<' => {
                    self.match_next_char(
                        '=',
                        TokenType::LessEqual,
                        String::from("<="),
                        TokenType::Less,
                        String::from("<"),
                    );
                }
                '>' => {
                    self.match_next_char(
                        '=',
                        TokenType::GreaterEqual,
                        String::from(">="),
                        TokenType::Greater,
                        String::from(">"),
                    );
                }

                '/' => {
                    match self.peek() {
                        Some('/') => {
                            // matched a comment line
                            while self.next() != Some('\n') {}
                        }
                        _ => self.add_token(TokenType::Slash, String::from("/")),
                    }
                }

                '"' => self.match_string(),

                _ => {
                    if Self::is_digit(c) {
                        self.match_number(c);
                    } else if Self::is_alphabetic(c) {
                        self.match_keyword_or_identifier(c);
                    } else {
                        eprintln!("Line {} has unexpected character", self.line);
                        process::exit(65);
                    }
                }
            },
        }
    }

    fn match_string(&mut self) {
        let mut lexeme = String::new();
        loop {
            match self.next() {
                Some('"') => {
                    self.add_token(TokenType::String, lexeme.clone());
                    break;
                }
                Some('\n') => {
                    self.line += 1;
                    lexeme.push('\n');
                }
                Some(c) => {
                    lexeme.push(c.clone());
                }
                None => {
                    eprintln!("Line {} has Unterminated string", self.line);
                    process::exit(65);
                }
            }
        }
    }

    fn match_number(&mut self, starting_char: char) {
        let mut lexeme = String::from(starting_char);
        let mut decimal_read = false;
        loop {
            match self.peek() {
                Some(c) if Self::is_digit(c.clone()) => {
                    lexeme.push(c.clone());
                    self.next();
                }
                Some('.') if !decimal_read => match self.peek_further() {
                    Some(c) if Self::is_digit(c.clone()) => {
                        lexeme.push('.');
                        decimal_read = true;
                        self.next();
                    }
                    _ => {}
                },
                _ => {
                    break;
                }
            }
        }
        self.add_token(TokenType::Number, lexeme);
    }

    fn match_keyword_or_identifier(&mut self, starting_char: char) {
        let mut lexeme = String::from(starting_char);
        loop {
            match self.peek() {
                Some(c) if Self::is_alphabetic(c.clone()) || Self::is_digit(c.clone()) => {
                    lexeme.push(c.clone());
                    self.next();
                }
                _ => {
                    break;
                }
            }
        }
        let mut keywords = HashMap::from([
            ("or", TokenType::Or),
            ("and", TokenType::And),
            ("if", TokenType::If),
            ("else", TokenType::Else),
            ("true", TokenType::True),
            ("false", TokenType::False),
            ("fun", TokenType::Fun),
            ("return", TokenType::Return),
            ("class", TokenType::Class),
            ("super", TokenType::Super),
            ("this", TokenType::This),
            ("var", TokenType::Var),
            ("nil", TokenType::Nil),
            ("for", TokenType::For),
            ("while", TokenType::While),
            ("print", TokenType::Print),
        ]);
        match keywords.remove(lexeme.as_str()) {
            None => {
                self.add_token(TokenType::Identifier, lexeme);
            }
            Some(keyword_token) => {
                self.add_token(keyword_token, lexeme);
            }
        }
    }

    fn match_next_char(
        &mut self,
        to_match: char,
        match_token: TokenType,
        match_lexeme: String,
        unmatch_token: TokenType,
        unmatch_lexeme: String,
    ) {
        match self.peek() {
            Some(c) if c.clone() == to_match => {
                self.next();
                self.add_token(match_token, match_lexeme);
            }
            _ => {
                self.add_token(unmatch_token, unmatch_lexeme);
            }
        }
    }

    fn add_token(&mut self, token_type: TokenType, lexeme: String) {
        self.tokens.push(Token::new(token_type, lexeme, self.line));
    }

    fn is_signed_number(&mut self) -> bool {
        self.rewind();
        let next = self.next_non_whitespace();
        if next == None {
            self.next();
            return false;
        }
        let next_is_digit = Self::is_digit(next.unwrap().clone());
        if !next_is_digit {
            self.next();
            return false;
        }
        let retval = match self.prev_non_whitespace() {
            None => true,
            Some(c) => {
                if *c == '-' || *c == '+' {
                    true
                } else {
                    false
                }
            }
        };
        self.next();
        retval
    }

    fn is_digit(ch: char) -> bool {
        match ch {
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => true,
            _ => false,
        }
    }

    fn is_alphabetic(ch: char) -> bool {
        match ch {
            'a'..='z' | 'A'..='Z' => true,
            _ => false,
        }
    }

    fn is_whitespace(ch: &char) -> bool {
        *ch == ' ' || *ch == '\r' || *ch == '\n'
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn single_letter() {
        let source = "a".to_string();
        let mut scanner = Scanner::new(&source);
        let tokens = scanner.scan_tokens();
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].token_type, TokenType::Identifier);
        assert_eq!(tokens[0].lexeme, "a".to_string());
        assert_eq!(tokens[1].token_type, TokenType::Eof);
    }

    #[test]
    fn simple_expression() {
        let source = "4 - 6 / 3 == 2".to_string();
        let mut scanner = Scanner::new(&source);
        let tokens = scanner.scan_tokens();
        assert_eq!(tokens.len(), 8);
        assert_eq!(tokens[0].token_type, TokenType::Number);
        assert_eq!(tokens[0].lexeme, "4".to_string());
        assert_eq!(tokens[1].token_type, TokenType::Minus);
        assert_eq!(tokens[1].lexeme, "-".to_string());
        assert_eq!(tokens[2].token_type, TokenType::Number);
        assert_eq!(tokens[2].lexeme, "6".to_string());
        assert_eq!(tokens[3].token_type, TokenType::Slash);
        assert_eq!(tokens[3].lexeme, "/".to_string());
        assert_eq!(tokens[4].token_type, TokenType::Number);
        assert_eq!(tokens[4].lexeme, "3".to_string());
        assert_eq!(tokens[5].token_type, TokenType::EqualEqual);
        assert_eq!(tokens[5].lexeme, "==".to_string());
        assert_eq!(tokens[6].token_type, TokenType::Number);
        assert_eq!(tokens[6].lexeme, "2".to_string());
    }

    #[test]
    fn signed_number() {
        let source = "4 - - 2.3 ".to_string();
        let mut scanner = Scanner::new(&source);
        let tokens = scanner.scan_tokens();
        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[0].token_type, TokenType::Number);
        assert_eq!(tokens[0].lexeme, "4".to_string());
        assert_eq!(tokens[1].token_type, TokenType::Minus);
        assert_eq!(tokens[1].lexeme, "-".to_string());
        assert_eq!(tokens[2].token_type, TokenType::Number);
        assert_eq!(tokens[2].lexeme, "-2.3".to_string());
        assert_eq!(tokens[3].token_type, TokenType::Eof);
    }
}
