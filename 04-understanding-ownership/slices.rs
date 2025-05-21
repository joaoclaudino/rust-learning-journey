// Exemplo de Slices
// Slices permitem acessar partes específicas de dados.

fn primeira_palavra(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn main() {
    let frase = String::from("olá mundo");
    let palavra = primeira_palavra(&frase);
    println!("A primeira palavra é: '{}'", palavra);
}
