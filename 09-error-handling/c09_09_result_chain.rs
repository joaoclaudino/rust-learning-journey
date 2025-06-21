use std::fs::File;
use std::io::{self, Read};

fn carregar_dados(caminho: &str) -> Result<String, io::Error> {
    File::open(caminho)
        .and_then(|mut f| {
            let mut s = String::new();
            f.read_to_string(&mut s)?;
            Ok(s)
        })
}

fn main() {
    match carregar_dados("exemplo.txt") {
        Ok(dados) => println!("Dados: {}", dados),
        Err(e) => println!("Erro: {}", e),
    }
}
