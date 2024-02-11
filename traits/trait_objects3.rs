fn main() {
    let v1 = String::from("hello");
    let v2 = "world";
    v1.bar(Box::new(v1.clone()));
    v2.bar(Box::new(v2));
    println!("{:?}", v1.baz(7));
    println!("{:?}", v2.baz(8));
}

#[allow(anonymous_parameters)]
trait Foo: std::fmt::Debug {
    fn bar(&self, Box<dyn Foo>);
    fn baz(&self, i32) -> Box<dyn Foo>;
}

impl Foo for String {
    fn bar(&self, a: Box<dyn Foo>) {
        println!("{:?}", a);
    }
    fn baz(&self, x: i32) -> Box<dyn Foo> {
        if x > 0 {
            Box::new("returned a &str")
        } else {
            Box::new(String::from("returned a String"))
        }
    }
}

impl Foo for &str {
    fn bar(&self, a: Box<dyn Foo>) {
        println!("{:?}", a);
    }
    fn baz(&self, x: i32) -> Box<dyn Foo> {
        if x > 0 {
            Box::new(String::from("returned a String"))
        } else {
            Box::new("returned a &str")
        }
    }
}
