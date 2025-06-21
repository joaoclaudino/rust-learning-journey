use rand::Rng;

fn main() {
    let numero: u8 = rand::thread_rng().gen_range(1..=10);
    println!("Número aleatório entre 1 e 10: {}", numero);
}
