mod ferramentas {
    pub mod martelo {
        pub fn bater() {
            println!("Martelando!");
        }
    }
}

pub use ferramentas::martelo;

fn main() {
    martelo::bater();
}
