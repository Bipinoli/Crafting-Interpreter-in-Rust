#![allow(dead_code)]
use crate::scanner::token::{Token, TokenType};
use crate::vm::bytecode::{ByteCode, Opcode};

struct TokenStream<'a> {
    tokens: &'a Vec<Token>,
    cursor: usize,
}
impl<'a> TokenStream<'a> {
    fn from(tokens: &'a Vec<Token>) -> Self {
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
    let mut tokens = TokenStream::from(tokens);
    if is_end(tokens.peek()) {
        return emit_end(&tokens.next());
    }
    if is_number(tokens.peek()) {
        return binary_parser(&mut tokens);
    }
    dbg!(&tokens.peek());
    panic!("not implemented");
}

fn binary_parser(tokens: &mut TokenStream) -> ByteCode {
    let token = tokens.next();
    let mut left = emit_number(&token);
    loop {
        left = pratt_parser(left, &token, tokens);
        if is_end(tokens.peek()) {
            return left;
        }
    }
}

fn pratt_parser(left: ByteCode, left_token: &Token, tokens: &mut TokenStream) -> ByteCode {
    let op = tokens.next();
    if is_end(&op) || !is_binary_operator(&op) {
        return left;
    }
    let right_token = tokens.next();
    if is_end(&right_token) {
        eprintln!(
            "[{}] binary operator '{}' needs a valid right operand",
            right_token.line, op.lexeme
        );
        panic!()
    }
    if left_token.token_type != right_token.token_type {
        eprintln!(
            "[{}] binary operator '{}' needs the left and right operand to be of same type",
            right_token.line, op.lexeme
        );
        panic!()
    }
    let mut right = {
        if is_number(left_token) {
            emit_number(&right_token)
        } else {
            panic!("not implemented")
        }
    };
    loop {
        let next_op = tokens.peek();
        if is_end(next_op) {
            return ByteCode::merge_binary(&left, &right, opcode_from_op(&op), op.line as u32);
        }
        if get_binding_power(&(op.token_type)).right_operand
            >= get_binding_power(&(next_op.token_type)).left_operand
        {
            return ByteCode::merge_binary(&left, &right, opcode_from_op(&op), op.line as u32);
        }
        right = pratt_parser(right, &right_token, tokens);
    }
}

struct BindingPower {
    left_operand: f32,
    right_operand: f32,
}
fn get_binding_power(operator: &TokenType) -> BindingPower {
    match operator {
        TokenType::EqualEqual
        | TokenType::BangEqual
        | TokenType::Greater
        | TokenType::GreaterEqual
        | TokenType::Less
        | TokenType::LessEqual => BindingPower {
            left_operand: -1.1,
            right_operand: -1.0,
        },
        TokenType::Plus => BindingPower {
            left_operand: 1.0,
            right_operand: 1.1,
        },
        TokenType::Minus => BindingPower {
            left_operand: 2.0,
            right_operand: 2.1,
        },
        TokenType::Star => BindingPower {
            left_operand: 3.0,
            right_operand: 3.1,
        },
        TokenType::Slash => BindingPower {
            left_operand: 4.0,
            right_operand: 4.1,
        },
        TokenType::Bang => BindingPower {
            left_operand: 5.0,
            right_operand: 5.1,
        },
        _ => panic!("unknown binding power for the operator: {}", operator),
    }
}

fn is_end(token: &Token) -> bool {
    match token.token_type {
        TokenType::Eof => true,
        _ => false,
    }
}

fn is_binary_operator(token: &Token) -> bool {
    match token.token_type {
        TokenType::Plus
        | TokenType::Minus
        | TokenType::Star
        | TokenType::Slash
        | TokenType::EqualEqual
        | TokenType::Less
        | TokenType::LessEqual
        | TokenType::Greater
        | TokenType::GreaterEqual
        | TokenType::BangEqual
        | TokenType::Or
        | TokenType::And => true,
        _ => false,
    }
}

fn opcode_from_op(token: &Token) -> Opcode {
    match token.token_type {
        TokenType::Plus => Opcode::Add,
        TokenType::Minus => Opcode::Sub,
        TokenType::Star => Opcode::Mul,
        TokenType::Slash => Opcode::Div,
        TokenType::EqualEqual => Opcode::Equal,
        TokenType::BangEqual => Opcode::NotEqual,
        TokenType::Greater => Opcode::Greater,
        TokenType::GreaterEqual => Opcode::GreaterEqual,
        TokenType::Less => Opcode::Less,
        TokenType::LessEqual => Opcode::LessEqual,
        TokenType::Bang => Opcode::Not,
        _ => panic!("can't convert the token: {} to opcode", token.lexeme),
    }
}

fn is_number(token: &Token) -> bool {
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

fn emit_number(token: &Token) -> ByteCode {
    match token.token_type {
        TokenType::Number => {
            let num = token.lexeme.parse::<f64>().unwrap();
            let mut code = ByteCode::new();
            code.write_data(num);
            code.write_code(Opcode::Const as u8, token.line as u32);
            code.write_code(0, token.line as u32);
            code.write_code(Opcode::Ret as u8, token.line as u32);
            code
        }
        _ => panic!("can't emit number from a NaN"),
    }
}

#[cfg(test)]
mod tests {
    use crate::scanner::token::{Token, TokenType};
    use crate::vm::bytecode::ByteCode;

    use super::compile;

    #[test]
    fn it_works() {
        // 2 - 6 / 2 + 2 * 4
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
        bytecode.disasm("2 - 6 / 2 + 2 * 4");
    }
}
