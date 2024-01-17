mod error;

pub fn run(source: &String) {
    println!("running: {}", source);
    error::report(23, "bad pattern", "x = 22");
}