#![allow(dead_code)]

use super::bytecode::ByteCode;
use super::bytecode::Opcode;

pub enum InterpretResult {
    Ok,
    CompileErr,
    RuntimeErr,
}

const STACK_LIMIT: usize = 256;
pub struct VM {
    stack: [f64; 256],
    ip: usize,
    sp: usize,
}

impl VM {
    pub fn new() -> Self {
        VM {
            stack: [0.0; 256],
            ip: 0,
            sp: 0,
        }
    }
    pub fn interpret(&mut self, byte_code: &ByteCode) -> InterpretResult {
        self.reset();
        loop {
            let instruction = byte_code.fetch_instruction(&mut self.ip);

            #[cfg(feature = "debug_exec_trace")]
            {
                println!("----------");
                println!("stack: [{}]", {
                    self.stack[..self.sp]
                        .iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<_>>()
                        .join(",")
                });
                byte_code.disasm_instruction(self.ip - 1);
            }

            match instruction {
                Opcode::Ret => return InterpretResult::Ok,
                Opcode::Const => {
                    let addr = byte_code.fetch_operand(&mut self.ip);
                    let constant = byte_code.fetch_data(addr as usize);
                    self.push(constant as f64);
                }
                Opcode::Neg => {
                    let v = self.pop();
                    self.push(-v);
                }
                Opcode::Add => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(a + b);
                }
                Opcode::Sub => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(a - b);
                }
                Opcode::Mul => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(a * b);
                }
                Opcode::Div => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(a / b);
                }
            }
        }
    }
    fn push(&mut self, v: f64) {
        if self.sp >= STACK_LIMIT {
            panic!("stackoverflow");
        }
        self.stack[self.sp] = v;
        self.sp += 1;
    }
    fn pop(&mut self) -> f64 {
        if self.sp == 0 {
            panic!("stack underflow");
        }
        let retval = self.stack[self.sp - 1];
        self.sp -= 1;
        retval
    }

    fn reset(&mut self) {
        self.ip = 0;
        self.sp = 0;
    }
}
