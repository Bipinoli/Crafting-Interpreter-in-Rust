#![allow(dead_code)]
use crate::scanner::token::{Token, TokenType};
use crate::vm::bytecode::{ByteCode, Opcode};

struct TokenStream<'a> {
    tokens: &'a Vec<Token>,
    cursor: usize,
}
impl<'a> TokenStream<'a> {
    fn new(tokens: &'a Vec<Token>) -> Self {
        TokenStream { tokens, cursor: 0 }
    }
    fn peek(&self) -> &Token {
        &self.tokens[self.cursor]
    }
    fn next(&mut self) -> Token {
        let retval = self.tokens[self.cursor].clone();
        self.cursor += 1;
        retval
    }
}

pub fn compile(tokens: &Vec<Token>) -> ByteCode {
    todo!()
}

fn binary_parser() -> ByteCode {
    todo!()
}

// fn pratt_parser(left: ByteCode, tokens: &TokenStream) -> ByteCode {
//     //TODO: currently only works with number
//     //TODO: assumption that there is no syntax errors
//     let token = tokens.peek();
//     if let TokenType::Eof = token.token_type {
//         return left;
//     }
//     if !is_binary_operator(&token) {
//         eprintln!("Line {}: next token is not a binary operator", token.line);
//         panic!("bad");
//     }
//     let operator = tokens.next();
//     let right = tokens.next();
//     if let TokenType::Eof = right.token_type {
//         eprintln("Line {}: binary operator takes two operands", token.line);
//         panic!("bad");
//     }
//     loop {
//         let next_operator = tokens.peek();
//         if let  TokenType::Eof = next_operator.token_type {
//             return
//         }
//     }
// }

fn is_binary_operator(token: &Token) -> bool {
    match token.token_type {
        TokenType::Plus | TokenType::Minus | TokenType::Slash | TokenType::Star => true,
        _ => false,
    }
}

fn is_binary_operand(token: &Token) -> bool {
    match token.token_type {
        TokenType::Number => true,
        _ => false,
    }
}

fn emit_end(token: &Token) -> ByteCode {
    let mut code = ByteCode::new();
    code.write_code(Opcode::Ret as u8, token.line as u32);
    code
}

// fn parser(left: ByteCode, tokens: &TokenStream) -> ByteCode {
//     if let Token::Eof = lexer.peek() {
//         return match left {
//             Some(v) => v,
//             None => ParseTree::Leaf(Token::End),
//         };
//     }
//     let left_operator = match left {
//         Some(left) => (left, lexer.next()),
//         None => (ParseTree::Leaf(lexer.next()), lexer.next()),
//     };
//     let left = left_operator.0;
//     let operator = left_operator.1;
//     let mut right: ParseTree = ParseTree::Leaf(lexer.next());
//     loop {
//         let next_operator = lexer.peek();
//         if let Token::End = next_operator {
//             return ParseTree::Branch {
//                 operator: operator.clone(),
//                 left: Box::new(left),
//                 right: Box::new(right),
//             };
//         }
//         if get_binding_power(&operator).right >= get_binding_power(next_operator).left {
//             return ParseTree::Branch {
//                 operator: operator.clone(),
//                 left: Box::new(left),
//                 right: Box::new(right),
//             };
//         }
//         right = parse(Some(right), lexer);
//     }
// }

#[cfg(test)]
mod tests {
    use crate::scanner::token::{Token, TokenType};
    use crate::vm::bytecode::ByteCode;

    use super::compile;

    #[test]
    fn it_works() {
        let tokens = vec![
            Token {
                token_type: TokenType::Number,
                lexeme: "2".to_string(),
                line: 1,
            },
            Token {
                token_type: TokenType::Minus,
                lexeme: "-".to_string(),
                line: 1,
            },
            Token {
                token_type: TokenType::Number,
                lexeme: "6".to_string(),
                line: 1,
            },
            Token {
                token_type: TokenType::Slash,
                lexeme: "/".to_string(),
                line: 1,
            },
            Token {
                token_type: TokenType::Number,
                lexeme: "2".to_string(),
                line: 1,
            },
            Token {
                token_type: TokenType::Plus,
                lexeme: "+".to_string(),
                line: 1,
            },
            Token {
                token_type: TokenType::Number,
                lexeme: "2".to_string(),
                line: 1,
            },
            Token {
                token_type: TokenType::Star,
                lexeme: "*".to_string(),
                line: 1,
            },
            Token {
                token_type: TokenType::Number,
                lexeme: "4".to_string(),
                line: 1,
            },
            Token {
                token_type: TokenType::Eof,
                lexeme: "".to_string(),
                line: 1,
            },
        ];

        let bytecode = compile(&tokens);

        todo!();
    }
}
