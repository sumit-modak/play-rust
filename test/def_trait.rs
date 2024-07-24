struct Error(String);

trait MyTrait {
    type MyType = Result<(), Error>;

    fn call(&self) -> Self::MyType;
}

struct MyStruct {
    inner: u32,
}

impl MyTrait for MyStruct {
    fn call(&self) -> MyTrait::MyType {
        println!("{}", self.inner);
        Ok(())
    }
}

fn main() {
    let x = MyStruct { inner: 7 };
    x.call();
}
