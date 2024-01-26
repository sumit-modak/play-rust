fn main() {
    let m: Box<String> = Box::new(String::from("sumit"));
    hello(&m);
    hello(&(*m)[..])        // no semicolon still worked
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

// DerefMut trait for mutable references
