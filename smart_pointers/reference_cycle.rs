use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None
        }
    }
}

fn main() {
    let a: Rc<List> = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a rc count = {}", Rc::strong_count(&a));

    let b: Rc<List> = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count = {}", Rc::strong_count(&a));
    println!("b rc count = {}", Rc::strong_count(&b));

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("a rc count = {}", Rc::strong_count(&a));
    println!("b rc count = {}", Rc::strong_count(&b));
}
