#[allow(unused_mut, unused_variables)]

fn main() {
    let x = foo; // function item
    println!("{}", std::mem::size_of_val(&x));

    let x1: fn() = foo; // function pointer
    println!("{}", std::mem::size_of_val(&x1));

    let mut y = bar::<i32>;
    println!("{}", std::mem::size_of_val(&y));

    // y = bar::<u32>; // <-- error
    // because function item always has unique
    // types if their signatures are same

    // they are actually not mutable even if mut keyword is defined
    let mut z = bar::<u32>;

    baz(bar::<u32>);
    baz(bar::<i32>);
}

fn foo() {}
fn bar<T>(_: u32) -> u32 {
    0
}
fn baz(f: fn(u32) -> u32) {
    println!("{}", std::mem::size_of_val(&f));
}
fn quox<F>(f: &mut F)
where
    F: Fn(),
{
}
