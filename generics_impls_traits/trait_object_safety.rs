trait MyTrait: std::fmt::Debug {
    fn foo(&self) -> Box<dyn MyTrait>;
    // this cant be done because it doesnt take self as args
    // so there is no way to invoke this function using this
    // trait object. The only way to invoke this function is
    // using generics or static dispatch
    // fn bar() {}
}

impl MyTrait for u32 {
    fn foo(&self) -> Box<dyn MyTrait> {
        Box::new(42)
    }
}

impl MyTrait for String {
    fn foo(&self) -> Box<dyn MyTrait> {
        Box::new(self.clone())
    }
}

fn my_function(x: Box<dyn MyTrait>) -> Box<dyn MyTrait> {
    x.foo()
}

fn main() {
    println!("{:?}", my_function(Box::new(13_u32)));
    println!("{:?}", my_function(Box::new(String::from("abc"))));
}
