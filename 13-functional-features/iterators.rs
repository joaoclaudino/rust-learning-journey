// iterators.rs - Iteradores em Rust
fn main() {
    let nums = vec![1, 2, 3, 4, 5];

    let even: Vec<_> = nums.iter().filter(|&&x| x % 2 == 0).collect();
    println!("Even numbers: {:?}", even);

    let sum: i32 = nums.iter().sum();
    println!("Sum of numbers: {}", sum);
}