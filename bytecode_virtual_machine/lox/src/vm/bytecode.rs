#![allow(dead_code)]

use std::usize;

#[repr(u8)]
#[derive(Debug)]
pub enum Opcode {
    Ret = 0,
    Const = 1,
}
impl TryFrom<u8> for Opcode {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Opcode::Ret),
            1 => Ok(Opcode::Const),
            _ => Err(()),
        }
    }
}

pub struct ByteCode {
    code: Vec<u8>,
    data: Vec<u8>,
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
    pub fn write_data(&mut self, byte: u8) {
        self.data.push(byte);
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
    fn disasm_instruction(&self, offset: usize) -> usize {
        print!("{:#06x} ", offset);
        let opcode = Opcode::try_from(self.code[offset]);
        if let Err(_) = opcode {
            panic!("No opcode with a byte: {}", self.code[offset]);
        };
        match opcode.unwrap() {
            Opcode::Ret => self.simple_instruction("Ret", offset),
            Opcode::Const => self.const_instruction("Const", offset),
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

