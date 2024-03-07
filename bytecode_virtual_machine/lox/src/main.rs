use vm::bytecode::{ByteCode, Opcode};
use vm::vm::VM;

mod vm;

fn main() {
    let mut chunk = ByteCode::new();
    let mut machine = VM::new();
    chunk.write_data(24.2);
    chunk.write_data(2.0);
    chunk.write_code(Opcode::Const as u8, 1);
    chunk.write_code(0, 1);
    chunk.write_code(Opcode::Const as u8, 2);
    chunk.write_code(1, 2);
    chunk.write_code(Opcode::Div as u8, 3);
    chunk.write_code(Opcode::Ret as u8, 1);
    // chunk.disasm("chunk");

    machine.interpret(&chunk);
}
