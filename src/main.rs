use anyhow::Result;

/// Greets the user with a welcome message.
fn greet_user() -> Result<()> {
    println!("Hello, world!");
    println!("How are you?");
    println!("Welcome to Rust programming!");
    Ok(())
}

fn main() -> Result<()> {
    greet_user()?;
    Ok(())
}
