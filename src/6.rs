// Generate a random Rust code snippet
fn main() {
    let mut rng = rand::thread_rng();
    let number: u64 = rng.gen_range(1..=10);
    println!("{}", number);
}
