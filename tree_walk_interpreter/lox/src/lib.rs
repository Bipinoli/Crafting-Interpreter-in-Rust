use crate::scanner::Scanner;

mod error;
mod scanner;

pub fn run(source: &String) {
    println!("running: {}", source);
    let mut scanner = Scanner::new(source);
    dbg!(&scanner.source);

    scanner.scan_tokens();
    dbg!(scanner.tokens);
}
