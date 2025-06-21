fn main() {
    let v = vec![1, 2, 3];
    let elemento = v.get(1);

    match elemento {
        Some(valor) => println!("Encontrado: {}", valor),
        None => println!("Nada encontrado"),
    }
}
