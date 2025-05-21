// Uso simplificado com if let para verificar enums

enum Status {
    Ativo,
    Inativo,
}

fn main() {
    let status = Status::Ativo;

    if let Status::Ativo = status {
        println!("O status � ativo!");
    } else {
        println!("O status n�o � ativo.");
    }
}
