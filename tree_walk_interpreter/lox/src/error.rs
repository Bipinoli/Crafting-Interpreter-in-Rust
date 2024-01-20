pub fn report(line: usize, message: String, location: String) {
    println!("[line {}] Error {}: {}", line, location, message);
}
