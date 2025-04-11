use std::io;

fn main() {
    let input = String::from("Enter your name: ");
    println!("{}", input);

    loop {
        print!("Hello, {}! How are you doing today? ", input);
        io::stdout().flush().unwrap(); // Ensure the output is flushed before moving to the next line

        match std::io::stdin().read_line(&mut buf) {
            Ok(_) => {
                if let Some(line) = buf.split_whitespace() {
                    println!("You said: {}", line.trim());
                } else {
                    eprintln!("Invalid input. Please try again.");
                }
            },
            Err(e) => {
                eprintln!("An error occurred while reading from stdin: {}", e);
                break;
            }
        }
    }

    println!("Thank you for using this program!");
}
