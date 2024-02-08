trait Hi {
    fn say_hi(&self);
}

trait Hello {
    fn say_hello(&self);
}

impl Hi for &str {
    fn say_hi(&self) {
        println!("hi");
    }
}

impl Hello for &str {
    fn say_hello(&self) {
        println!("hello");
    }
}

// impl Hi for Box<str> {
//     fn say_hi(&self) {
//         println!("Box<&str> hi");
//     }
// }

// impl Hello for Box<str> {
//     fn say_hello(&self) {
//         println!("Box<&str> hello");
//     }
// }

trait Foo: Hi + Hello {}

impl Foo for &str {}
// impl Foo for Box<str> {}

fn func(s: &dyn Foo) {
    s.say_hi();
    s.say_hello();
}

fn main() {
    let s = "string";
    func(&s);
}
