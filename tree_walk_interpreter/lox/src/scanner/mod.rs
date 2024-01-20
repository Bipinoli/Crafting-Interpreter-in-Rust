use std::collections::HashMap;
use crate::error;
use crate::scanner::token::{Token, TokenType};
use std::iter::Peekable;
use std::process;
use std::str::Chars;

mod token;

pub struct Scanner<'a> {
    source: Peekable<Chars<'a>>,
    tokens: Vec<Token>,
    line: usize,
}
impl Scanner {
    pub fn new(source: String) -> Scanner {
        Scanner {
            source: source.chars().peekable(),
            tokens: vec![],
            line: 1,
        }
    }
    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while self.source.peek() != None {
            self.scan_token();
        }
        self.add_token(TokenType::Eof, String::new());
        &self.tokens
    }

    fn scan_token(&mut self) {
        match self.source.next() {
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
                '-' => self.add_token(TokenType::Minus, String::from("-")),
                '+' => self.add_token(TokenType::Plus, String::from("+")),
                '*' => self.add_token(TokenType::Star, String::from("*")),
                ';' => self.add_token(TokenType::Semicolon, String::from(";")),

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
                    match self.source.peek() {
                        Some('/') => {
                            // matched a comment line
                            while self.source.next() != Some('\n') {}
                        }
                        _ => self.add_token(TokenType::Slash, String::from("/")),
                    }
                }

               '"' => self.match_string(),
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => self.match_number(c),
               'a'..='z' | 'A'..='Z' => self.match_keyword_or_identifier(c),

                _ => {
                    error::report(
                        self.line,
                        String::from("Unexpected character!"),
                        String::new(),
                    );
                    process::exit(65);
                }
            },
        }
    }

    fn match_string(&mut self) {

    }

    fn match_number(&mut self, starting_char: char) {

    }

    fn match_keyword_or_identifier(&mut self, starting_char: char) {
        let mut lexeme = String::from(starting_char);
        loop {
            match self.source.peek() {
                Some(c) => {
                    match c {
                        'a'..='z' | 'A'..='Z' | '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                            lexeme.push(c.clone());
                            self.source.next();
                        }
                        _ => { break; }
                    }
                }
                None => { break; }
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
        match self.source.peek() {
            Some(c) if c == to_match => {
                self.source.next();
                self.add(match_token, match_lexeme);
            }
            _ => {
                self.add_token(unmatch_token, unmatch_lexeme);
            }
        }
    }

    fn add_token(&mut self, token_type: TokenType, lexeme: String) {
        self.tokens.push(Token::new(token_type, lexeme, self.line));
    }
}
