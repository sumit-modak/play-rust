use std::cell::RefCell;
use std::fmt::Debug;

#![allow(unused_mut)]

struct Test<T> {
    inner: RefCell<T>,
}

impl<T> Test<T> {
    fn new(value: T) -> Test<T> {
        Test {
            inner: RefCell::new(value),
        }
    }
}

impl<T: Debug> Test<T> {
    fn test_print(&self) {
        println!("{:?}", self.inner.borrow_mut())
    }
}

fn main() {
    let mut x = Test::new("hello");
    x.test_print();
}
