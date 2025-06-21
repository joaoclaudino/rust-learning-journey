mod segredo {
    fn mensagem_privada() {
        println!("Você não deveria ver isso!");
    }

    pub fn mensagem_publica() {
        println!("Você pode ver esta mensagem!");
        mensagem_privada();
    }
}

fn main() {
    segredo::mensagem_publica();
}
