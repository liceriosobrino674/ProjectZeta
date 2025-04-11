use std::fs;

fn main() {
    let file_path = "example.txt";
    if let Ok(contents) = fs::read_to_string(file_path) {
        println!("File content: {}", contents);
    } else {
        eprintln!("Error reading the file: {:?}", file_path);
    }
}
