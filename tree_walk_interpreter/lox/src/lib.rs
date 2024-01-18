mod error;

pub fn run(source: &String) {
    println!("running: {}", source);
    error::report(23, String::from("bad pattern"), String::from("x = 22"));
}
