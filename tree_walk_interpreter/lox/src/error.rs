pub fn report(line: i32, message: String, location: String) {
    println!("[line {}] Error {}: {}", line, location, message);
}