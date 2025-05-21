// Exemplo de implementa��o de m�todos em Structs

struct Retangulo {
    largura: u32,
    altura: u32,
}

impl Retangulo {
    // m�todo para calcular �rea
    fn area(&self) -> u32 {
        self.largura * self.altura
    }
}

fn main() {
    let ret = Retangulo {
        largura: 30,
        altura: 50,
    };

    println!("�rea do ret�ngulo: {}", ret.area());
}
