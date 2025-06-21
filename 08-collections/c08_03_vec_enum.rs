enum Celula {
    Inteiro(i32),
    Flutuante(f64),
    Texto(String),
}

fn main() {
    let linha = vec![
        Celula::Inteiro(10),
        Celula::Flutuante(3.14),
        Celula::Texto(String::from("Olá")),
    ];

    for cel in linha {
        match cel {
            Celula::Inteiro(i) => println!("Inteiro: {}", i),
            Celula::Flutuante(f) => println!("Flutuante: {}", f),
            Celula::Texto(t) => println!("Texto: {}", t),
        }
    }
}
