fn main() {
    let texto = Some("42");
    let numero = texto.map(|s| s.parse::<i32>().unwrap());
    println!("Número: {:?}", numero);
}
