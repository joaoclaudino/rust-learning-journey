mod lib;
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let config = lib::Config::build(&args).unwrap();
    lib::run(config).unwrap();
}