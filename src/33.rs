fn main() {
    // Example of using Rust's type system to create complex structures

    let mut list = Vec::new();

    while !list.is_empty() {
        if list.len() > 10 {
            break;
        }
        list.push(String::from("Item " + &i.to_string()));
    }

    println!("Your items: {:?}", list);
}
