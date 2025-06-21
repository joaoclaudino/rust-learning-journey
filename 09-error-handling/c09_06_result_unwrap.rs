use std::fs::read_to_string;

fn main() {
    let conteudo = read_to_string("exemplo.txt").unwrap();
    println!("Arquivo: {}", conteudo);
}
