#[derive(Debug)]
struct Foo {
    s: String,
    v: Vec<u8>,
}

fn main() {
    let var = Foo {
        s: String::from("hello"),
        v: vec![1, 2, 3, 4, 5],
    };

    let s1 = var.s;
    println!("{s1} {:?}", var.v);
}
