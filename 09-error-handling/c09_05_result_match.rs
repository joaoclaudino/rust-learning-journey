use std::fs::File;
use std::io::{self, Read};

fn main() {
    let resultado = ler_arquivo("exemplo.txt");

    match resultado {
        Ok(conteudo) => println!("Conteúdo:
{}", conteudo),
        Err(e) => println!("Erro ao ler: {}", e),
    }
}

fn ler_arquivo(nome: &str) -> Result<String, io::Error> {
    let mut f = File::open(nome)?;
    let mut buffer = String::new();
    f.read_to_string(&mut buffer)?;
    Ok(buffer)
}
