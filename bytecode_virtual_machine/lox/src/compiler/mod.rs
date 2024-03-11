#![allow(dead_code)]
use crate::scanner::token::Token;
use crate::vm::bytecode::{ByteCode, Opcode};

pub fn compile(tokens: &Vec<Token>) -> ByteCode {
    let mut bytecode = ByteCode::new();
    bytecode.write_data(24.2);
    bytecode.write_data(2.0);
    bytecode.write_code(Opcode::Const as u8, 1);
    bytecode.write_code(0, 1);
    bytecode.write_code(Opcode::Const as u8, 2);
    bytecode.write_code(1, 2);
    bytecode.write_code(Opcode::Div as u8, 3);
    bytecode.write_code(Opcode::Ret as u8, 1);

    bytecode
}
