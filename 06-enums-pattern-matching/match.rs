enum Moeda {
    Real,
    Euro,
    Dolar,
}

fn valor_em_centavos(moeda: Moeda) -> u8 {
    match moeda {
        Moeda::Real => 100,
        Moeda::Euro => 110,
        Moeda::Dolar => 90,
    }
}

fn main() {
    let moeda = Moeda::Euro;
    println!("Valor em centavos: {}", valor_em_centavos(moeda));
}