use std::fs::File;
use std::io::{self, Read};

fn ler_arquivo(nome: &str) -> Result<String, io::Error> {
    let mut f = File::open(nome)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn main() {
    match ler_arquivo("exemplo.txt") {
        Ok(txt) => println!("Conteúdo:
{}", txt),
        Err(e) => println!("Erro: {}", e),
    }
}
