use std::collections::HashMap;

fn main() {
    let texto = "hello world wonderful world";
    let mut mapa = HashMap::new();

    for palavra in texto.split_whitespace() {
        let contagem = mapa.entry(palavra).or_insert(0);
        *contagem += 1;
    }

    println!("{:?}", mapa);
}
