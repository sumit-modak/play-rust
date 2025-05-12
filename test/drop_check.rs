struct Foo<'a, T: Default> {
    v: &'a mut T
}

impl<T: Default> std::ops::Drop for Foo<'_, T> {
    fn drop(&mut self) {
        std::mem::replace(self.v, T::default());
    }
}

#[allow(unused_variables, unused_assignments)]
fn main() {
    let (foo, mut t);
    // let (mut t, foo); // this way of declaring will let the program compile
    t = String::from("hello");
    foo = Foo { v: &mut t };
}
