use std::fmt;

#[derive(Debug)]
enum MeuErro {
    Invalido,
    Nulo,
}

impl fmt::Display for MeuErro {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

fn checar(valor: Option<i32>) -> Result<i32, MeuErro> {
    match valor {
        Some(v) if v > 0 => Ok(v),
        Some(_) => Err(MeuErro::Invalido),
        None => Err(MeuErro::Nulo),
    }
}

fn main() {
    match checar(Some(0)) {
        Ok(v) => println!("Valor: {}", v),
        Err(e) => println!("Erro: {}", e),
    }
}
