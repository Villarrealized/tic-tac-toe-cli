use std::io::{stdin, Error};

/// Return a Result with user input stripped of whitespace and newline characters
pub fn get_input() -> Result<String, Error> {
    let mut input = String::new();
    stdin().read_line(&mut input)?;

    // remove the new line and extra whitespace
    Ok(String::from(input.trim()))
}
