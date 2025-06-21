mod casa {
    pub mod cozinha {
        pub fn cozinhar() {
            println!("Cozinhando...");
        }

        fn segredo() {
            println!("Receita secreta!");
        }
    }

    pub fn acessar_cozinha() {
        self::cozinha::cozinhar();
    }
}

fn main() {
    casa::cozinha::cozinhar();
    casa::acessar_cozinha();
}
