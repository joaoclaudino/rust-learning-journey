// Capítulo 4 - Exemplo de Ownership em Rust
// Este exemplo mostra como Rust trata a propriedade dos valores.

fn main() {
    let s1 = String::from("hello");
    let s2 = s1; // s1 perdeu a propriedade, transferindo para s2

    // println!("{}", s1); // Isso gera erro, pois s1 não é mais válido
    println!("s2 é válido e possui valor: {}", s2);
}
