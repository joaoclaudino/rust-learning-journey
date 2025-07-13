// refcell.rs - Exemplo com RefCell
use std::cell::RefCell;

fn main() {
    let data = RefCell::new(vec![1, 2, 3]);
    data.borrow_mut().push(4);
    println!("Data: {:?}", data.borrow());
}