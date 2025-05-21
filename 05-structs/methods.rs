// Exemplo de implementação de métodos em Structs

struct Retangulo {
    largura: u32,
    altura: u32,
}

impl Retangulo {
    // método para calcular área
    fn area(&self) -> u32 {
        self.largura * self.altura
    }
}

fn main() {
    let ret = Retangulo {
        largura: 30,
        altura: 50,
    };

    println!("Área do retângulo: {}", ret.area());
}
