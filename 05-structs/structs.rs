// Exemplo básico com Structs em Rust

// Definição da struct Usuario
struct Usuario {
    nome: String,
    idade: u32,
    ativo: bool,
}

fn main() {
    let usuario1 = Usuario {
        nome: String::from("João"),
        idade: 45,
        ativo: true,
    };

    println!("Nome: {}, Idade: {}, Ativo: {}", usuario1.nome, usuario1.idade, usuario1.ativo);
}
