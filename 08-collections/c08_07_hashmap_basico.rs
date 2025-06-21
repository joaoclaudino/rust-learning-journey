use std::collections::HashMap;

fn main() {
    let mut placar = HashMap::new();
    placar.insert(String::from("Azul"), 10);
    placar.insert(String::from("Vermelho"), 50);

    if let Some(pontos) = placar.get("Azul") {
        println!("Pontos Azul: {}", pontos);
    }
}
