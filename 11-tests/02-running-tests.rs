// Testes com diferentes comportamentos
#[test]
fn should_pass() {
    assert!(true);
}

#[test]
#[should_panic]
fn should_fail() {
    panic!("Erro intencional");
}