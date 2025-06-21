mod saudacao {
    pub fn ola() {
        println!("Olá de dentro do módulo!");
    }
}

fn main() {
    saudacao::ola();
}
