use vm::bytecode::{ByteCode, Opcode};

mod vm;

fn main() {
    let mut chunk = ByteCode::new();
    chunk.write_data(23.2 as u8);
    chunk.write_code(Opcode::Const as u8, 1);
    chunk.write_code(0, 1);
    chunk.write_code(Opcode::Ret as u8, 1);
    chunk.disasm("chunk");
}
