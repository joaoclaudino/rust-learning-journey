use std::collections::HashMap;

fn main() {
    let mut mapa = HashMap::new();
    mapa.entry("chave1").or_insert(100);
    mapa.entry("chave2").or_insert(200);
    println!("{:?}", mapa);
}
