use std::ops::Deref;

struct MyBox<T> (T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

fn main() {
    let x: i32 = 4;
    let y: MyBox<i32> = MyBox::new(x);

    assert_eq!(4, x);
    assert_eq!(4, *y);  // *y = *(y.deref())

    // let o1: MyBox<i32> = MyBox(4);
    // let o4: MyBox<i32> = MyBox::<i32>(4);
    // let o2: MyBox<i32> = MyBox::<i32>::new(4);
    // let o3: MyBox<i32> = MyBox::new(4);
    
}