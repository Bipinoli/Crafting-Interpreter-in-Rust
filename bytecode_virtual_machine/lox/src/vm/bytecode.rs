#![allow(dead_code)]

use std::usize;

#[repr(u8)]
#[derive(Debug)]
pub enum Opcode {
    Ret = 0,
    Const = 1,
    Neg = 2,
    Add = 3,
    Sub = 4,
    Mul = 5,
    Div = 6,
}
impl TryFrom<u8> for Opcode {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Opcode::Ret),
            1 => Ok(Opcode::Const),
            2 => Ok(Opcode::Neg),
            3 => Ok(Opcode::Add),
            4 => Ok(Opcode::Sub),
            5 => Ok(Opcode::Mul),
            6 => Ok(Opcode::Div),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
pub struct ByteCode {
    code: Vec<u8>,
    data: Vec<f64>,
    line_info: Vec<u32>,
}
impl ByteCode {
    pub fn new() -> Self {
        ByteCode {
            code: Vec::new(),
            data: Vec::new(),
            line_info: Vec::new(),
        }
    }
    pub fn write_code(&mut self, byte: u8, line: u32) {
        self.code.push(byte);
        self.line_info.push(line);
    }
    pub fn write_data(&mut self, byte: f64) {
        self.data.push(byte);
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
    pub fn fetch_data(&self, addr: usize) -> f64 {
        if addr >= self.data.len() {
            panic!("attempted to fetch data outside the data section boundary");
        }
        self.data[addr]
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
        for (i, item) in self.data.iter().enumerate() {
            println!("{:#06x} {}", i, item);
        }
    }
    pub fn disasm_instruction(&self, offset: usize) -> usize {
        print!("{:#06x} ", offset);
        let opcode = Opcode::try_from(self.code[offset]);
        if let Err(_) = opcode {
            panic!("No opcode with a byte: {}", self.code[offset]);
        };
        match opcode.unwrap() {
            Opcode::Ret => self.simple_instruction("Ret", offset),
            Opcode::Const => self.const_instruction("Const", offset),
            Opcode::Neg => self.simple_instruction("Neg", offset),
            Opcode::Add => self.simple_instruction("Add", offset),
            Opcode::Sub => self.simple_instruction("Sub", offset),
            Opcode::Mul => self.simple_instruction("Mul", offset),
            Opcode::Div => self.simple_instruction("Div", offset),
        }
    }
    fn simple_instruction(&self, name: &str, offset: usize) -> usize {
        println!("{name}");
        offset + 1
    }
    fn const_instruction(&self, name: &str, offset: usize) -> usize {
        let data_offset = self.code[offset + 1] as usize;
        if data_offset >= self.data.len() {
            panic!("attemptinng to read outside of data section");
        }
        let value = self.data[data_offset];
        println!("{} {:#06x} '{}'", name, data_offset, value);
        offset + 2
    }
}
