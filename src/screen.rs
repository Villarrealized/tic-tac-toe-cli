/// Clear the terminal screen using an ANSI escape sequence
pub fn clear() {
    print!("\x1B[2J\x1B[1;1H");
}
