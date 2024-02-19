use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // single char tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Dot,
    Comma,
    Plus,
    Minus,
    Slash,
    Star,
    Semicolon,

    // one or two chars
    Equal,
    EqualEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Bang,
    BangEqual,

    // literals
    Identifier,
    String,
    Number,

    // keywords
    Or,
    And,
    If,
    Else,
    True,
    False,
    Fun,
    Return,
    Class,
    Super,
    This,
    Var,
    Nil,
    For,
    While,
    Print,

    Eof,
}
impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, line: usize) -> Token {
        Token {
            token_type,
            lexeme,
            line,
        }
    }
    pub fn to_string(&self) -> String {
        format!("{} {} {}", self.token_type, self.lexeme, self.line)
    }
}
