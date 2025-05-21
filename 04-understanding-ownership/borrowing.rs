// Exemplo de Borrowing (Empr�stimo)
// Demonstra como usar refer�ncias para emprestar valores sem perder a propriedade.

fn calcular_tamanho(s: &String) -> usize {
    s.len() // apenas empresta o valor, sem tomar posse
}

fn main() {
    let s1 = String::from("hello");
    let tamanho = calcular_tamanho(&s1);
    println!("O tamanho de '{}' � {}.", s1, tamanho);
}
