use anyhow::{Context, Result};
use std::io::{self, Write};

/// Greets the user with a welcome message.
fn greet_user() -> Result<()> {
    println!("Hello, world!");
    println!("How are you?");
    Ok(())
}

fn ask_name() -> Result<String> {
    print!("What is your name? ");
    io::stdout().flush().context("Failed to flush stdout")?;
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .context("Failed to read line from stdin")?;
    Ok(name.trim().to_string())
}

fn main() -> Result<()> {
    greet_user()?;
    let name = ask_name()?;
    println!("Nice to meet you, {}!", name);
    Ok(())
}
