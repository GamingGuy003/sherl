use crate::components::input;
use std::io::{self, Write};

/// prints the prompt and reads input for testing purposes
pub fn prompt() -> io::Result<()> {
    print!("> ");
    io::stdout().flush()?;
    let command = input::read_line()?;
    println!("Command was: {}", command.trim());
    Ok(())
}
