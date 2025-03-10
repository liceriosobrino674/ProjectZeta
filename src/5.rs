use std::fs;

fn main() {
    let path = "path/to/file";
    let contents = fs::read_to_string(path).unwrap();
    println!("{}", contents);
}
