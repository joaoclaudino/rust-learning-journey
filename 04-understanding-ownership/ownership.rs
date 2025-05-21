// Cap�tulo 4 - Exemplo de Ownership em Rust
// Este exemplo mostra como Rust trata a propriedade dos valores.

fn main() {
    let s1 = String::from("hello");
    let s2 = s1; // s1 perdeu a propriedade, transferindo para s2

    // println!("{}", s1); // Isso gera erro, pois s1 n�o � mais v�lido
    println!("s2 � v�lido e possui valor: {}", s2);
}
