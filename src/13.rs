fn main() {
    let mut rng = rand::thread_rng();
    let mut v = Vec::new();
    
    for i in 0..100 {
        v.push(rng.gen_range(1, 100));
    }
    
    println!("{:?}", &v);
}
