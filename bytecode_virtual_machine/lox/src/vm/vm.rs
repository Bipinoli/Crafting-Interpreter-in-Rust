#![allow(dead_code)]
use super::bytecode::ByteCode;
use super::bytecode::Opcode;
use super::value::Value;

#[derive(Debug)]
pub enum InterpretResult {
    Ok(Value),
    CompileErr,
    RuntimeErr,
}

const STACK_LIMIT: usize = 256;
pub struct VM {
    stack: Vec<Value>,
    ip: usize,
    sp: usize,
}

impl VM {
    pub fn new() -> Self {
        VM {
            stack: Vec::new(),
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
                Opcode::Ret => return InterpretResult::Ok(self.stack.pop().unwrap()),
                Opcode::Num => {
                    let addr = byte_code.fetch_operand(&mut self.ip);
                    let constant = byte_code.fetch_number(addr as usize);
                    self.push(Value::Num(constant));
                }
                Opcode::Str => {
                    let addr = byte_code.fetch_operand(&mut self.ip);
                    let constant = byte_code.fetch_string(addr as usize);
                    self.push(Value::Str(constant.clone()));
                }
                Opcode::Neg => match self.pop() {
                    Value::Num(v) => self.push(Value::Num(-v)),
                    _ => panic!("Negate only works on number"),
                },
                Opcode::Add => {
                    let b = self.pop();
                    let a = self.pop();
                    if a.is_num() && b.is_num() {
                        let a = a.get_num();
                        let b = b.get_num();
                        self.push(Value::Num(a + b));
                    } else if a.is_string() && b.is_string() {
                        let a = a.get_string();
                        let b = b.get_string();
                        let result = format!("{}{}", a, b);
                        self.push(Value::Str(result));
                    } else {
                        panic!("only numnbers can be added")
                    }
                }
                Opcode::Sub => {
                    let b = self.pop();
                    let a = self.pop();
                    if a.is_num() && b.is_num() {
                        let a = a.get_num();
                        let b = b.get_num();
                        self.push(Value::Num(a - b));
                    } else {
                        panic!("only numnbers can be substracted")
                    }
                }
                Opcode::Mul => {
                    let b = self.pop();
                    let a = self.pop();
                    if a.is_num() && b.is_num() {
                        let a = a.get_num();
                        let b = b.get_num();
                        self.push(Value::Num(a * b));
                    } else {
                        panic!("only numnbers can be multiplied")
                    }
                }
                Opcode::Div => {
                    let b = self.pop();
                    let a = self.pop();
                    if a.is_num() && b.is_num() {
                        let a = a.get_num();
                        let b = b.get_num();
                        self.push(Value::Num(a / b));
                    } else {
                        panic!("only numnbers can be divided")
                    }
                }
                Opcode::True => {
                    self.push(Value::Bool(true));
                }
                Opcode::False => {
                    self.push(Value::Bool(false));
                }
                Opcode::Not => {
                    let v = self.pop();
                    match v {
                        Value::Bool(b) => {
                            self.push(Value::Bool(!b));
                        }
                        _ => {
                            panic!("Not operator only works on boolean value")
                        }
                    }
                }
                Opcode::Less => {
                    let b = self.pop();
                    let a = self.pop();
                    if a.is_num() && b.is_num() {
                        let a = a.get_num();
                        let b = b.get_num();
                        self.push(Value::Bool(a < b));
                    } else {
                        panic!("only numnbers can be compared with <")
                    }
                }
                Opcode::LessEqual => {
                    let b = self.pop();
                    let a = self.pop();
                    if a.is_num() && b.is_num() {
                        let a = a.get_num();
                        let b = b.get_num();
                        self.push(Value::Bool(a <= b));
                    } else {
                        panic!("only numnbers can be compared with <=")
                    }
                }
                Opcode::Greater => {
                    let b = self.pop();
                    let a = self.pop();
                    if a.is_num() && b.is_num() {
                        let a = a.get_num();
                        let b = b.get_num();
                        self.push(Value::Bool(a > b));
                    } else {
                        panic!("only numnbers can be compared with >")
                    }
                }
                Opcode::GreaterEqual => {
                    let b = self.pop();
                    let a = self.pop();
                    if a.is_num() && b.is_num() {
                        let a = a.get_num();
                        let b = b.get_num();
                        self.push(Value::Bool(a >= b));
                    } else {
                        panic!("only numnbers can be compared with >=")
                    }
                }
                Opcode::Equal => {
                    let b = self.pop();
                    let a = self.pop();
                    if a.is_bool() && b.is_bool() {
                        let a = a.get_bool();
                        let b = b.get_bool();
                        self.push(Value::Bool(a == b));
                    } else if a.is_num() && b.is_num() {
                        let a = a.get_num();
                        let b = b.get_num();
                        self.push(Value::Bool(a == b));
                    } else if a.is_string() && b.is_string() {
                        let a = a.get_string();
                        let b = b.get_string();
                        self.push(Value::Bool(a == b));
                    } else {
                        panic!("only booleans and numbers can be compared with ==")
                    }
                }
                Opcode::NotEqual => {
                    let b = self.pop();
                    let a = self.pop();
                    if a.is_bool() && b.is_bool() {
                        let a = a.get_bool();
                        let b = b.get_bool();
                        self.push(Value::Bool(a != b));
                    } else if a.is_num() && b.is_num() {
                        let a = a.get_num();
                        let b = b.get_num();
                        self.push(Value::Bool(a != b));
                    } else {
                        panic!("only booleans and numbers can be compared with !=")
                    }
                }
            }
        }
    }
    fn push(&mut self, v: Value) {
        if self.sp >= STACK_LIMIT {
            panic!("stackoverflow");
        }
        self.stack.push(v);
        self.sp += 1;
    }
    fn pop(&mut self) -> Value {
        if self.sp == 0 {
            panic!("stack underflow");
        }
        self.sp -= 1;
        self.stack.pop().unwrap()
    }

    fn reset(&mut self) {
        self.ip = 0;
        self.sp = 0;
    }
}
