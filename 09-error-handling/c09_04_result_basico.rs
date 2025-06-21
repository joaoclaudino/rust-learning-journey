fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("Divisão por zero"))
    } else {
        Ok(a / b)
    }
}

fn main() {
    match divide(10, 2) {
        Ok(resultado) => println!("Resultado: {}", resultado),
        Err(e) => println!("Erro: {}", e),
    }
}
