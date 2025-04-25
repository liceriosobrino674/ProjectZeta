fn main() {
    let mut data = vec![1, 2, 3];
    println!("Original data: {:?}", data);
    
    // Modify the data by adding 5 to each element in the vector
    data = data.iter_mut().map(|&x| x + 5).collect();
    
    println!("Modified data: {:?}", data);
}
