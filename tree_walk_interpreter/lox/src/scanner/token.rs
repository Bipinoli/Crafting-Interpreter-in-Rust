#[derive(Debug)]
enum TokenType {
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
    Eof
}

struct Token {
    token_type: TokenType,
    lexeme: String,
    line: u32,
}

impl Token {
    fn to_string(&self)-> String {
        format!("{} {} {}", self.token_type, self.lexeme, self.line)
    }
}
