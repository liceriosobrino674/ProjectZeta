// Example of using channels to handle file reading and output.
use std::io::{self, Write};

fn main() {
    let reader = std::fs::read_to_string("input.txt").unwrap();
    let writer = io::stdout();

    let mut buf = String::new();
    reader.read_to_string(&mut buf).unwrap();
    let content = format!("{}", buf);

    writer.write_all(&content.as_bytes()).unwrap();
}
