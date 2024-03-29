use std::env;
use std::fs;
use std::io::Write;
use std::process;

use lox_core;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        println!("Usage: lox [script]");
        // exit code as per: https://man.freebsd.org/cgi/man.cgi?query=sysexits&apropos=0&sektion=0&manpath=FreeBSD+4.3-RELEASE&format=html
        process::exit(64);
    } else if args.len() == 2 {
        run_file(&args[1]);
    } else {
        run_prompt();
    }
}

fn run_file(file_path: &String) {
    let file_content = fs::read_to_string(file_path);
    match file_content {
        Err(e) => {
            println!("{}. {}", file_path, e.to_string());
            process::exit(65);
        }
        Ok(content) => lox_core::run(&content),
    }
}

fn run_prompt() {
    println!("Lox REPL (enter exit() / q to exit)");
    loop {
        print!("> ");
        let mut input = String::new();
        std::io::stdout().flush().expect("flush to stdout failed!");
        std::io::stdin()
            .read_line(&mut input)
            .expect("can not read user input");

        if input.trim() == String::from("exit()") {
            break;
        }
        if input.trim() == String::from("q") {
            break;
        }
        lox_core::run(&input);
    }
}
