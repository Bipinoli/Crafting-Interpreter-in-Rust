#![allow(dead_code)]
use super::bytecode::ByteCode;
use super::bytecode::Opcode;
use std::fmt;

pub enum InterpretResult {
    Ok,
    CompileErr,
    RuntimeErr,
}

#[derive(Clone, Copy)]
enum Value {
    Num(f64),
    Bool(bool),
    Nil,
}
impl Value {
    fn is_num(&self) -> bool {
        match self {
            Value::Num(_) => true,
            _ => false,
        }
    }
    fn is_bool(&self) -> bool {
        match self {
            Value::Bool(_) => true,
            _ => false,
        }
    }
    fn get_num(&self) -> f64 {
        match self {
            Value::Num(v) => v.clone(),
            _ => panic!("can't extract number from non number Value"),
        }
    }
    fn get_bool(&self) -> bool {
        match self {
            Value::Bool(b) => b.clone(),
            _ => panic!("can't extract boolean from non boolean Value)"),
        }
    }
}
impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Nil => write!(f, "Nil"),
            Value::Bool(v) => write!(f, "{v}"),
            Value::Num(n) => write!(f, "{n}"),
        }
    }
}

const STACK_LIMIT: usize = 256;
pub struct VM {
    stack: [Value; 256],
    ip: usize,
    sp: usize,
}

impl VM {
    pub fn new() -> Self {
        VM {
            stack: [Value::Nil; 256],
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
                    self.push(Value::Num(constant));
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
        self.stack[self.sp] = v;
        self.sp += 1;
    }
    fn pop(&mut self) -> Value {
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
