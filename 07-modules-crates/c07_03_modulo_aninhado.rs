mod animais {
    pub mod cachorro {
        pub fn latir() {
            println!("Au au!");
        }
    }

    pub mod gato {
        pub fn miar() {
            println!("Miau!");
        }
    }
}

fn main() {
    animais::cachorro::latir();
    animais::gato::miar();
}
