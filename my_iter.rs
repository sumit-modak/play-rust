struct MyData<T> {
    data: Box<[T]>,
}

impl<T> MyData<T> {
    fn new(n: usize) -> Self {
        let self = Box::new([T; n]);
    }
}

fn main() {}
