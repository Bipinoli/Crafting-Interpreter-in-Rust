#![allow(dead_code)]

use super::bytecode::ByteCode;
use super::bytecode::Opcode;

pub enum InterpretResult {
    Ok,
    CompileErr,
    RuntimeErr,
}

pub fn interpret(byte_code: &ByteCode) -> InterpretResult {
    let mut ip: usize = 0;
    loop {
        let instruction = byte_code.fetch_instruction(&mut ip);

        #[cfg(feature = "debug_exec_trace")]
        byte_code.disasm_instruction(ip - 1);

        match instruction {
            Opcode::Ret => return InterpretResult::Ok,
            Opcode::Const => {
                let addr = byte_code.fetch_operand(&mut ip);
                let constant = byte_code.fetch_data(addr as usize);
                println!("constant: {}", constant);
            }
        }
    }
}
