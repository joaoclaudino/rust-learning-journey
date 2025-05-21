// Uso simplificado com if let para verificar enums

enum Status {
    Ativo,
    Inativo,
}

fn main() {
    let status = Status::Ativo;

    if let Status::Ativo = status {
        println!("O status é ativo!");
    } else {
        println!("O status não é ativo.");
    }
}
