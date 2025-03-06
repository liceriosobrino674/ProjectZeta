use std::error::Error;
use std::fmt;
use std::io::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    println!("Enter your name:");
    io::stdin().read_line(&mut input)?;

    let name = input.trim();
    println!("Hello, {}!", name);

    Ok(())
}
