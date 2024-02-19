use crate::{
    parser::{ast_printer::AstPrinterVisitor, Parser},
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

    let expression = parser.parse();
    let ast = expression.accept(Box::new(AstPrinterVisitor::new()));
    println!("{ast}");
}
