// Exemplo de enum em Rust

enum Direcao {
    Norte,
    Sul,
    Leste,
    Oeste,
}

fn main() {
    let dir = Direcao::Norte;

    match dir {
        Direcao::Norte => println!("Indo para o Norte!"),
        Direcao::Sul => println!("Indo para o Sul!"),
        Direcao::Leste => println!("Indo para o Leste!"),
        Direcao::Oeste => println!("Indo para o Oeste!"),
    }
}
