#![allow(unused_imports, unused_variables)]
use scanner::Scanner;
use std::io::Write;
use std::{env, fs, process};
use vm::bytecode::{ByteCode, Opcode};
use vm::vm::VM;

use crate::compiler::compile;

mod compiler;
mod scanner;
mod vm;
//
// fn main() {
//     let args: Vec<String> = env::args().collect();
//     if args.len() == 1 {
//         repl();
//     } else if args.len() == 2 {
//         run_file(&args[1]);
//     } else {
//         eprintln!("wrong arguments!");
//         process::exit(64);
//     }
//
//     let mut chunk = ByteCode::new();
//     let mut machine = VM::new();
//     chunk.write_data(24.2);
//     chunk.write_data(2.0);
//     chunk.write_code(Opcode::Const as u8, 1);
//     chunk.write_code(0, 1);
//     chunk.write_code(Opcode::Const as u8, 2);
//     chunk.write_code(1, 2);
//     chunk.write_code(Opcode::Div as u8, 3);
//     chunk.write_code(Opcode::Ret as u8, 1);
//     // chunk.disasm("chunk");
//
//     machine.interpret(&chunk);
// }

fn main() {
    let code = "2 - 6 / 2 + 2 * 4".to_owned();
    dbg!(&code);
    let mut scanner = Scanner::new(&code);
    let tokens = scanner.scan_tokens();
    let bytecode = compile(&tokens);
    let mut vm = VM::new();
    vm.interpret(&bytecode);

    // let args: Vec<String> = env::args().collect();
    // if args.len() > 2 {
    //     println!("Usage: lox [script]");
    //     // exit code as per: https://man.freebsd.org/cgi/man.cgi?query=sysexits&apropos=0&sektion=0&manpath=FreeBSD+4.3-RELEASE&format=html
    //     process::exit(64);
    // } else if args.len() == 2 {
    //     run_file(&args[1]);
    // } else {
    //     repl();
    // }
}

fn run_file(file_path: &String) {
    let file_content = fs::read_to_string(file_path);
    match file_content {
        Err(e) => {
            println!("{}. {}", file_path, e.to_string());
            process::exit(65);
        }
        Ok(content) => (),
    }
}

fn repl() {
    println!("Lox REPL (enter q to exit)");
    loop {
        print!("> ");
        let mut input = String::new();
        std::io::stdout().flush().expect("flush to stdout failed!");
        std::io::stdin()
            .read_line(&mut input)
            .expect("can not read user input");
        if input.trim() == String::from("q") {
            break;
        }
        ();
    }
}
