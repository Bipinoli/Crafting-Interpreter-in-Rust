#![allow(dead_code)]

use std::{string, usize};

#[repr(u8)]
#[derive(Debug)]
pub enum Opcode {
    Ret = 0,
    Num = 1,
    Neg = 2,
    Add = 3,
    Sub = 4,
    Mul = 5,
    Div = 6,
    True = 7,
    False = 8,
    Not = 9,
    Equal = 10,
    Greater = 11,
    Less = 12,
    GreaterEqual = 13,
    LessEqual = 14,
    NotEqual = 15,

    Str = 16,
}
impl TryFrom<u8> for Opcode {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Opcode::Ret),
            1 => Ok(Opcode::Num),
            2 => Ok(Opcode::Neg),
            3 => Ok(Opcode::Add),
            4 => Ok(Opcode::Sub),
            5 => Ok(Opcode::Mul),
            6 => Ok(Opcode::Div),
            7 => Ok(Opcode::True),
            8 => Ok(Opcode::False),
            9 => Ok(Opcode::Not),
            10 => Ok(Opcode::Equal),
            11 => Ok(Opcode::Greater),
            12 => Ok(Opcode::Less),
            13 => Ok(Opcode::GreaterEqual),
            14 => Ok(Opcode::LessEqual),
            15 => Ok(Opcode::NotEqual),
            16 => Ok(Opcode::Str),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
pub struct ByteCode {
    pub code: Vec<u8>,
    pub numbers: Vec<f64>,
    pub strings: Vec<String>,
    pub line_info: Vec<u32>,
}
impl ByteCode {
    pub fn new() -> Self {
        ByteCode {
            code: Vec::new(),
            numbers: Vec::new(),
            strings: Vec::new(),
            line_info: Vec::new(),
        }
    }
    pub fn is_nop(&self) -> bool {
        self.code.len() == 0
    }
    pub fn merge_binary(left: &ByteCode, right: &ByteCode, operation: Opcode, line: u32) -> Self {
        let mut code = ByteCode::new();
        let num_offset = left.numbers.len();
        let str_offset = left.strings.len();
        ByteCode::steal_data(&mut code, left);
        ByteCode::steal_data(&mut code, right);
        ByteCode::steal_code(&mut code, left, 0, 0);
        ByteCode::steal_code(&mut code, right, num_offset as u8, str_offset as u8);
        code.write_code(operation as u8, line);
        code.write_code(Opcode::Ret as u8, line);
        code
    }
    fn steal_data(target: &mut ByteCode, source: &ByteCode) {
        for n in &source.numbers {
            target.write_number(n.clone());
        }
        for s in &source.strings {
            target.write_string(s.clone());
        }
    }
    fn steal_code(target: &mut ByteCode, source: &ByteCode, num_offset: u8, str_offset: u8) {
        let mut cursor = 0;
        loop {
            let opcode = Opcode::try_from(source.code[cursor]).unwrap();
            match opcode {
                Opcode::Ret => return,
                Opcode::Num => {
                    target.write_code(Opcode::Num as u8, source.line_info[cursor]);
                    cursor += 1;
                    let addr = source.code[cursor];
                    target.write_code(addr + num_offset, source.line_info[cursor]);
                }
                _ => {
                    target.write_code(source.code[cursor], source.line_info[cursor]);
                }
            }
            cursor += 1;
        }
    }
    pub fn write_code(&mut self, byte: u8, line: u32) {
        self.code.push(byte);
        self.line_info.push(line);
    }
    pub fn write_number(&mut self, byte: f64) {
        self.numbers.push(byte);
    }
    pub fn write_string(&mut self, str: String) {
        self.strings.push(str);
    }
    pub fn fetch_instruction(&self, ip: &mut usize) -> Opcode {
        if *ip >= self.code.len() {
            panic!("attempted to fetch instruction from outside the code section");
        }
        let retval = Opcode::try_from(self.code[*ip]).expect("Not a valid opcode at the given ip");
        *ip += 1;
        retval
    }
    pub fn fetch_operand(&self, ip: &mut usize) -> u8 {
        if *ip >= self.code.len() {
            panic!("attempted to fetch operand from outside the code section");
        }
        let retval = self.code[*ip];
        *ip += 1;
        retval
    }
    pub fn fetch_number(&self, addr: usize) -> f64 {
        if addr >= self.numbers.len() {
            panic!("attempted to fetch data outside the data section boundary");
        }
        self.numbers[addr]
    }
    pub fn fetch_string(&self, addr: usize) -> &String {
        if addr >= self.strings.len() {
            panic!("attempted to fetch data outside the data section boundary");
        }
        &self.strings[addr]
    }
    pub fn disasm(&self, name: &str) {
        println!("====== Code section ({name}) ======");
        let mut offset = 0;
        while offset < self.code.len() {
            offset = self.disasm_instruction(offset);
        }
        self.disasm_data(name);
    }
    fn disasm_data(&self, name: &str) {
        println!("====== data section ({name}) ======");
        println!("Numbers: [{}]", {
            self.numbers
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        });
        println!("Strings: [{}]", { self.strings.join(", ") });
    }
    pub fn disasm_instruction(&self, offset: usize) -> usize {
        print!("{:#06x} ", offset);
        let opcode = Opcode::try_from(self.code[offset]);
        if let Err(_) = opcode {
            panic!("No opcode with a byte: {}", self.code[offset]);
        };
        match opcode.unwrap() {
            Opcode::Ret => self.simple_instruction("Ret", offset),
            Opcode::Num => self.num_instruction("Num", offset),
            Opcode::Str => self.str_instruction("Str", offset),
            Opcode::Neg => self.simple_instruction("Neg", offset),
            Opcode::Add => self.simple_instruction("Add", offset),
            Opcode::Sub => self.simple_instruction("Sub", offset),
            Opcode::Mul => self.simple_instruction("Mul", offset),
            Opcode::Div => self.simple_instruction("Div", offset),
            Opcode::True => self.simple_instruction("True", offset),
            Opcode::False => self.simple_instruction("False", offset),
            Opcode::Not => self.simple_instruction("Not", offset),
            Opcode::Equal => self.simple_instruction("==", offset),
            Opcode::NotEqual => self.simple_instruction("!=", offset),
            Opcode::Greater => self.simple_instruction(">", offset),
            Opcode::GreaterEqual => self.simple_instruction(">=", offset),
            Opcode::Less => self.simple_instruction("<", offset),
            Opcode::LessEqual => self.simple_instruction("<=", offset),
        }
    }
    fn simple_instruction(&self, name: &str, offset: usize) -> usize {
        println!("{name}");
        offset + 1
    }
    fn num_instruction(&self, name: &str, offset: usize) -> usize {
        let data_offset = self.code[offset + 1] as usize;
        if data_offset >= self.numbers.len() {
            panic!("attemptinng to read outside of data section");
        }
        let value = self.numbers[data_offset];
        println!("{} {:#06x} '{}'", name, data_offset, value);
        offset + 2
    }
    fn str_instruction(&self, name: &str, offset: usize) -> usize {
        let data_offset = self.code[offset + 1] as usize;
        if data_offset >= self.strings.len() {
            panic!("attemptinng to read outside of data section");
        }
        let value = &self.strings[data_offset];
        println!("{} {:#06x} '{}'", name, data_offset, value);
        offset + 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn merge_works() {
        // left = 2 + 3
        // right = 6 / 2
        // total = (2 + 3) - (6 / 2)
        let mut left = ByteCode::new();
        left.write_number(2.0);
        left.write_number(3.0);
        left.write_code(Opcode::Num as u8, 1);
        left.write_code(0, 1);
        left.write_code(Opcode::Num as u8, 1);
        left.write_code(1, 1);
        left.write_code(Opcode::Add as u8, 1);
        left.write_code(Opcode::Ret as u8, 1);

        let mut right = ByteCode::new();
        right.write_number(6.0);
        right.write_number(2.0);
        right.write_code(Opcode::Num as u8, 2);
        right.write_code(0, 2);
        right.write_code(Opcode::Num as u8, 2);
        right.write_code(1, 2);
        right.write_code(Opcode::Div as u8, 2);
        right.write_code(Opcode::Ret as u8, 2);

        let merged = ByteCode::merge_binary(&left, &right, Opcode::Sub, 2);

        let mut expected = ByteCode::new();
        expected.write_number(2.0);
        expected.write_number(3.0);
        expected.write_code(Opcode::Num as u8, 1);
        expected.write_code(0, 1);
        expected.write_code(Opcode::Num as u8, 1);
        expected.write_code(1, 1);
        expected.write_code(Opcode::Add as u8, 1);
        expected.write_number(6.0);
        expected.write_number(2.0);
        expected.write_code(Opcode::Num as u8, 2);
        expected.write_code(2, 2);
        expected.write_code(Opcode::Num as u8, 2);
        expected.write_code(3, 2);
        expected.write_code(Opcode::Div as u8, 2);
        expected.write_code(Opcode::Sub as u8, 2);
        expected.write_code(Opcode::Ret as u8, 2);

        assert_eq!(merged.disasm(""), expected.disasm(""));
    }
}
