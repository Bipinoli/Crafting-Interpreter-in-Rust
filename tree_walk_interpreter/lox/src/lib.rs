use crate::{
    parser::{visitors::interpreter::AstInterpreterVisitor, Parser},
    scanner::Scanner,
};

mod error;
mod parser;
mod scanner;

pub fn run(source: &String) {
    println!("running: {}", source);
    let mut scanner = Scanner::new(source);

    let tokens = scanner.scan_tokens();
    let mut parser = Parser::new(tokens);
    let statements = parser.parse();
    let interpreter = AstInterpreterVisitor::new();
    interpreter.interpret(statements);
}
