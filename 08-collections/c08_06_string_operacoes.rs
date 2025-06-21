fn main() {
    let s1 = String::from("Rust");
    let s2 = String::from("ace");
    let s3 = s1 + &s2;
    println!("{}", s3);
}
