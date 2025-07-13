# 🦀 Rust Learning Journey

🇺🇸 **English**

This repository documents my journey learning Rust, following the official book ["The Rust Programming Language"](https://doc.rust-lang.org/book/).

My goal is to build a solid portfolio demonstrating each step of my learning process and share insights regularly on LinkedIn to attract recruiters and grow professionally.

## 📖 Project Structure

Each directory represents a chapter from the book, with practical exercises, code snippets, and notes.

## 🎯 Objectives

- Learn Rust deeply, from fundamentals to advanced concepts.
- Build an open-source portfolio showcasing my Rust skills.
- Engage with the tech community and recruiters through LinkedIn.

---

🇧🇷 **Português**

Este repositório documenta minha jornada de aprendizado em Rust, seguindo o livro oficial ["The Rust Programming Language"](https://doc.rust-lang.org/book/).

Meu objetivo é construir um portfólio sólido demonstrando cada etapa do meu processo de aprendizado e compartilhar insights regularmente no LinkedIn para atrair recrutadores e crescer profissionalmente.

## 📖 Estrutura do Projeto

Cada diretório representa um capítulo do livro, contendo exercícios práticos, trechos de código e anotações.

## 🎯 Objetivos

- Aprender Rust profundamente, desde fundamentos até conceitos avançados.
- Construir um portfólio open-source demonstrando minhas habilidades em Rust.
- Engajar com a comunidade técnica e recrutadores através do LinkedIn.

---

## 📚 Chapter Summaries / Resumos por Capítulo

### `01-getting-started`
- 🇺🇸 Set up Rust and wrote the classic “Hello, world!”.
- 🇧🇷 Configurei o ambiente e escrevi o clássico “Hello, world!”.

### `02-guessing-game`
- 🇺🇸 Built an interactive CLI number guessing game using standard library.
- 🇧🇷 Criei um jogo interativo de adivinhação de números via terminal usando a biblioteca padrão.

### `04-understanding-ownership`
- 🇺🇸 Learned core concepts of ownership, borrowing and lifetimes in Rust.
- 🇧🇷 Aprendi os conceitos fundamentais de ownership, empréstimo e lifetimes em Rust.

### `05-structs`
- 🇺🇸 Practiced defining and using `struct`s to model custom data types.
- 🇧🇷 Pratiquei a definição e uso de `structs` para modelar tipos de dados personalizados.

### `06-enums-pattern-matching`
- 🇺🇸 Used enums and pattern matching (`match`, `if let`) to control flow and logic.
- 🇧🇷 Usei enums e correspondência de padrões (`match`, `if let`) para controle de fluxo e lógica.

---

### `07-modules-crates`

- 🇺🇸 Learned how to structure Rust code with modules, visibility (`pub`), and external crates (`rand`).
- 🇧🇷 Aprendi a organizar o código Rust com módulos, visibilidade (`pub`) e crates externos (`rand`).

Exemplos incluem:
- Módulos simples e aninhados
- `pub use` e escopos
- Separação de arquivos
- Uso de crates com `Cargo.toml`

---

### `08-collections`

- 🇺🇸 Explored built-in collections like `Vec`, `String`, and `HashMap`.
- 🇧🇷 Explorei coleções embutidas como `Vec`, `String` e `HashMap`.

Exemplos incluem:
- Iterações e mutabilidade
- Strings com UTF-8
- Contagem de palavras com `HashMap`
- Uso de `entry`, `or_insert`, etc.

---

### `09-error-handling`

- 🇺🇸 Learned how to handle errors in Rust with `Result`, `Option`, `unwrap`, `expect`, and custom error types.
- 🇧🇷 Aprendi a tratar erros em Rust usando `Result`, `Option`, `unwrap`, `expect` e tipos de erro personalizados.

Exemplos incluem:
- Leitura de arquivos com propagação de erro (`?`)
- Uso de `panic!`
- Criação de enum de erro customizado

---
### `10-generics-traits-lifetimes`

- 🇺🇸 Implemented generic functions and structs, learned about trait bounds and lifetime annotations.
- 🇧🇷 Implementei funções e structs genéricos, aprendi sobre bounds de traits e anotação de lifetimes.

Exemplos incluem:
- `largest<T>()`, uso de `impl Trait`, `where`, e `Deref`
- Structs com lifetimes
- Funções com múltiplas referências

---

### `11-tests`

- 🇺🇸 Wrote automated unit tests with `#[test]`, learned about test organization, `should_panic`, and test filtering.
- 🇧🇷 Escrevi testes unitários automatizados com `#[test]`, organizei em módulos, usei `should_panic` e filtros para execução.

Exemplos incluem:
- Execução de testes com `cargo test`
- Ignorando testes longos com `#[ignore]`
- Usando `Result` nos testes
- Visualizando output com `--show-output`

---

### `12-cli-minigrep`

- 🇺🇸 Built a complete command-line program that searches for strings in files (like `grep`).
- 🇧🇷 Construi um programa de linha de comando completo que busca por strings em arquivos (estilo `grep`).

Funcionalidades implementadas:
- Argumentos com `std::env`
- Leitura de arquivos com `fs`
- Modularização (`main.rs` + `lib.rs`)
- Case-insensitive search com variável de ambiente `IGNORE_CASE`
- Escrita para stderr com `eprintln!`
- Testes para busca sensível e insensível
### `13-functional-features`

- 🇺🇸 Learned to use closures, iterators, and functional programming style.
- 🇧🇷 Aprendi a usar closures, iteradores e programação funcional em Rust.

Exemplos incluem:
- `.map()`, `.filter()`, `.fold()`, `.collect()`
- Implementação de iteradores personalizados com `impl Iterator`
- Refatoração funcional do projeto `minigrep`

---

### `14-cargo-crates`

- 🇺🇸 Customized Cargo build profiles, explored crates.io publishing, and created workspaces.
- 🇧🇷 Personalizei perfis de build do Cargo, explorei publicação no crates.io e criei workspaces.

Exemplos incluem:
- `Cargo.toml` com metadados
- Estrutura de workspaces com múltiplos projetos

---

### `15-smart-pointers`

- 🇺🇸 Learned to use smart pointers: `Box<T>`, `Rc<T>`, `RefCell<T>` and interior mutability.
- 🇧🇷 Aprendi a usar ponteiros inteligentes: `Box<T>`, `Rc<T>`, `RefCell<T>` e o padrão de mutabilidade interior.

Exemplos incluem:
- Armazenamento no heap com `Box`
- Compartilhamento de dados com `Rc`
- Mutabilidade em tempo de execução com `RefCell`
- Estruturas complexas com `Rc<RefCell<T>>`

🚀 Rust memory management mastered!



🚧 More chapters coming soon as I continue my journey! / Mais capítulos em breve!
