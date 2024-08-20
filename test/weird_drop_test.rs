struct S;

impl Drop for S {
    fn drop(&mut self) {
        println!("Dropping S");
    }
}

fn main() {
    let foo = S;
    println!("In main");
    foo;
    // the above line is similar to
    // let _ = foo;
    println!("In main 2");
}
