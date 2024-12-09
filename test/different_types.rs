struct Foo {
    inner: String,
}

impl Foo {
    fn get(&self) -> &String {
        &self.inner
    }
    fn get(self) -> String {
        self.inner
    }
}

fn main() {
    let foo = Foo {
        inner: format!("hello"),
    };
    dbg!(foo.get());
}
