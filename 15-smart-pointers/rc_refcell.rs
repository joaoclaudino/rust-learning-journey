// rc_refcell.rs - Combinação de Rc e RefCell
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let shared = Rc::new(RefCell::new(vec![1]));
    let s1 = Rc::clone(&shared);
    let s2 = Rc::clone(&shared);

    s1.borrow_mut().push(2);
    s2.borrow_mut().push(3);

    println!("Shared data: {:?}", shared.borrow());
}