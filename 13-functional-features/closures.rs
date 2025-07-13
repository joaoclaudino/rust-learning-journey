// closures.rs - Exemplo de Closures
fn main() {
    let multiply = |a: i32, b: i32| a * b;
    println!("3 * 4 = {}", multiply(3, 4));

    let numbers = vec![1, 2, 3, 4];
    let squared: Vec<i32> = numbers.iter().map(|x| x * x).collect();
    println!("Squared numbers: {:?}", squared);
}