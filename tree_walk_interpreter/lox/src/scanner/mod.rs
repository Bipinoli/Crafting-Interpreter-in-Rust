use crate::error;
use crate::scanner::token::{Token, TokenType};
use std::iter::Peekable;
use std::process;
use std::str::Chars;

mod token;

pub struct Scanner<'a> {
    source: Peekable<Chars<'a>>,
    tokens: Vec<Token>,
    cur_lexeme: String,
    line: usize,
}
impl Scanner {
    pub fn new(source: String) -> Scanner {
        Scanner {
            source: source.chars().peekable(),
            tokens: vec![],
            cur_lexeme: String::new(),
            line: 1,
        }
    }
    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while self.source.peek() != None {
            self.scan_token();
        }
        self.add_token(TokenType::Eof, String::new(), self.line);
        &self.tokens
    }

    fn scan_token(&mut self) {
        match self.source.next() {
            None => {}
            Some(c) => match c {
                ' ' | '\r' | '\t' => {}
                '\n' => self.line += 1,

                '(' => self.add_token(TokenType::LeftParen, String::from("("), self.line),
                ')' => self.add_token(TokenType::RightParen, String::from(")"), self.line),
                '{' => self.add_token(TokenType::LeftBrace, String::from("{"), self.line),
                '}' => self.add_token(TokenType::RightBrace, String::from("}"), self.line),
                ',' => self.add_token(TokenType::Comma, String::from(","), self.line),
                '.' => self.add_token(TokenType::Dot, String::from("."), self.line),
                '-' => self.add_token(TokenType::Minus, String::from("-"), self.line),
                '+' => self.add_token(TokenType::Plus, String::from("+"), self.line),
                '*' => self.add_token(TokenType::Star, String::from("*"), self.line),
                ';' => self.add_token(TokenType::Semicolon, String::from(";"), self.line),

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
                        _ => self.add_token(TokenType::Slash, String::from("/"), self.line),
                    }
                }

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
                self.add(match_token, match_lexeme, self.line);
            }
            _ => {
                self.add_token(unmatch_token, unmatch_lexeme, self.line);
            }
        }
    }

    fn add_token(&mut self, token_type: TokenType, lexeme: String, line: usize) {
        self.tokens.push(Token::new(token_type, lexeme, line));
    }
}
