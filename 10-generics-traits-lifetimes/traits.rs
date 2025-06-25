trait Descreve {
    fn descricao(&self) -> String;
}

struct Artigo { titulo: String }
impl Descreve for Artigo {
    fn descricao(&self) -> String {
        format!("Artigo: {}", self.titulo)
    }
}