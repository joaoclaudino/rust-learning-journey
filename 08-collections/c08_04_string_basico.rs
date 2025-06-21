fn main() {
    let mut s = String::from("Olá");
    s.push(',');
    s.push_str(" mundo!");
    println!("{}", s);
}
