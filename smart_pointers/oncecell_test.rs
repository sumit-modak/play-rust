use std::cell::OnceCell;
use std::fmt::Debug;

struct Test<T> {
    inner: OnceCell<T>,
}

impl<T> Test<T> {
    fn new(value: T) -> Test<T> {
        let inner = OnceCell::new();
        let _ = inner.set(value);
        Test { inner }
    }
}

impl<T: Debug> Test<T> {
    fn test_print(&self) {
        // does not work
        println!("{:?}", self.inner.get_mut())
    }
}

fn main() {
    let mut x = Test::new("hello");
    x.test_print();
}
