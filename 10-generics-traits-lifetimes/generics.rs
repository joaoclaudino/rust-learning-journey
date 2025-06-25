fn maior<T: PartialOrd + Copy>(lista: &[T]) -> T {
    let mut maior = lista[0];
    for &item in lista.iter() {
        if item > maior {
            maior = item;
        }
    }
    maior
}