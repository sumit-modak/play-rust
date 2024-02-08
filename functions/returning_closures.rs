fn main() {
    let foo = new_make_fn("hello");
    let bar = new_make_fn("world");
    foo();
    bar();
    // foo(); // error because foo implements FnOnce so the ownership of the new struct object gets dropped on first call
    // bar(); // error because bar implements FnOnce so the ownership of the new struct object gets dropped on first call
}

// the closure returned borrows `z`
// so the lifetime of the closure is tied to the lifetime of `z`
// which means the lifetime of the closure and `z` is static
// fn make_fn() -> impl Fn() {
//     let z = String::new();
//     || {
//         println!("{}", z);
//     }
// }

fn new_make_fn(x: &str) -> impl FnOnce() {
    let z = String::from(x);
    // let z = 8738;
    move || {
        println!("{}", z);
        std::mem::drop(z);
    }
}
