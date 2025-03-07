fn main() {
    let numbers = vec![100, 25, 47, 66, 99];

    for number in numbers {
        if number % 2 == 0 {
            println!("Even: {}", number);
        } else {
            println!("Odd: {}", number);
        }
    }
}
